use super::{LINE_MAX, PROMPT};
use crate::{eval, features, pipeline, syscalls};

/// Cyrillic mode flag: when set, Latin letters map to Russian (ЙЦУКЕН) layout.
static mut G_CYRILLIC: bool = false;

unsafe fn clear_line(line: &[u8], _cursor: usize) {
    // Erase the PROMPT plus the line text (the old version only covered
    // line.len() columns, wiping the prompt but never restoring it —
    // history navigation left commands without the "osh$ " prefix).
    syscalls::write(1, b"\r".as_ptr(), 1);
    for _ in 0..(PROMPT.len() + line.len()) {
        syscalls::write(1, b" ".as_ptr(), 1);
    }
    syscalls::write(1, b"\r".as_ptr(), 1);
    syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
}

/// Try to read more bytes after an ESC to catch split escape sequences.
/// UART may deliver ESC, [, A as separate reads; spin-wait briefly.
unsafe fn drain_esc(rx_buf: &mut [u8; 16], n: &mut usize, i: usize) {
    // We have ESC at position i, need [ and direction byte.
    // Spin a few times to let UART deliver the rest.
    for _ in 0..5000 {
        if *n - i >= 3 {
            return;
        }
        let r = syscalls::read(0, rx_buf.as_mut_ptr().add(*n), (rx_buf.len() - *n) as u64);
        if r > 0 {
            *n += r as usize;
        }
    }
}
pub unsafe fn raw_mode_repl() -> ! {
    let mut line: [u8; LINE_MAX] = [0u8; LINE_MAX];
    let mut rx_buf = [0u8; 16];
    loop {
        let mut line_len: usize = 0;
        let mut cursor: usize = 0;
        syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
        features::nav_reset();
        'line_loop: loop {
            let mut n = syscalls::read(0, rx_buf.as_mut_ptr(), rx_buf.len() as u64);
            if n == 0 {
                syscalls::yield_cpu();
                continue;
            }
            if n < 0 {
                syscalls::exit(0);
            }
            let mut n = n as usize;
            let mut i = 0;
            while i < n {
                let b = rx_buf[i];
                if b == 0x1B {
                    // ESC received — wait for rest of escape sequence if split across reads
                    if i + 2 >= n {
                        drain_esc(&mut rx_buf, &mut n, i);
                    }
                    if i + 2 < n && rx_buf[i + 1] == b'[' {
                        match rx_buf[i + 2] {
                            b'A' => {
                                if let Some(entry) = features::nav_up() {
                                    clear_line(&line[..line_len], cursor);
                                    let cn = entry.len().min(LINE_MAX - 1);
                                    line[..cn].copy_from_slice(&entry[..cn]);
                                    line_len = cn;
                                    cursor = cn;
                                    syscalls::write(1, line.as_ptr(), line_len);
                                }
                                i += 3;
                                continue;
                            }
                            b'B' => {
                                match features::nav_down() {
                                    Some(entry) => {
                                        clear_line(&line[..line_len], cursor);
                                        let cn = entry.len().min(LINE_MAX - 1);
                                        line[..cn].copy_from_slice(&entry[..cn]);
                                        line_len = cn;
                                        cursor = cn;
                                        syscalls::write(1, line.as_ptr(), line_len);
                                    }
                                    None => {
                                        clear_line(&line[..line_len], cursor);
                                        line_len = 0;
                                        cursor = 0;
                                    }
                                }
                                i += 3;
                                continue;
                            }
                            b'C' => {
                                if cursor < line_len {
                                    cursor += 1;
                                    syscalls::write(1, b"\x1B[C".as_ptr(), 3);
                                }
                                i += 3;
                                continue;
                            }
                            b'D' => {
                                if cursor > 0 {
                                    cursor -= 1;
                                    syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                                }
                                i += 3;
                                continue;
                            }
                            _ => {}
                        }
                    }
                    i += 1;
                    continue;
                }
                match b {
                    b'\r' | b'\n' => {
                        syscalls::write(1, b"\r\n".as_ptr(), 2);
                        break 'line_loop;
                    }
                    0x7F | 0x08 => {
                        if cursor > 0 {
                            for j in (cursor - 1)..line_len {
                                line[j] = line[j + 1];
                            }
                            line_len -= 1;
                            cursor -= 1;
                            syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                            syscalls::write(1, line.as_ptr().add(cursor), line_len - cursor);
                            syscalls::write(1, b" ".as_ptr(), 1);
                            for _ in 0..(line_len - cursor + 1) {
                                syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                            }
                        }
                        i += 1;
                        continue;
                    }
                    0x09 => {
                        let result = features::tab_complete(&line[..line_len], cursor);
                        let new_line = result.line.as_slice();
                        let new_cursor = result.cursor;
                        clear_line(&line[..line_len], cursor);
                        let cn = new_line.len().min(LINE_MAX - 1);
                        line[..cn].copy_from_slice(&new_line[..cn]);
                        line_len = cn;
                        cursor = new_cursor.min(cn);
                        if result.printed {
                            syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
                        }
                        syscalls::write(1, line.as_ptr(), line_len);
                        let back = line_len.saturating_sub(cursor);
                        for _ in 0..back {
                            syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                        }
                        i += 1;
                        continue;
                    }
                    0x03 => {
                        syscalls::write(1, b"^C\r\n".as_ptr(), 4);
                        break 'line_loop;
                    }
                    0x04 => {
                        if line_len == 0 {
                            syscalls::write(1, b"exit\r\n".as_ptr(), 6);
                            syscalls::exit(0);
                        }
                        i += 1;
                        continue;
                    }
                    0x0B => {
                        // Ctrl+K: toggle Cyrillic/Latin keyboard layout
                        G_CYRILLIC = !G_CYRILLIC;
                        if G_CYRILLIC {
                            syscalls::write(1, b"\r\n[RU]\r\n".as_ptr(), 8);
                        } else {
                            syscalls::write(1, b"\r\n[EN]\r\n".as_ptr(), 8);
                        }
                        syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
                        syscalls::write(1, line.as_ptr(), line_len);
                        // Move cursor back to correct position
                        let back = line_len.saturating_sub(cursor);
                        for _ in 0..back {
                            syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                        }
                        i += 1;
                        continue;
                    }
                    c if (0x20..0x7F).contains(&c) => {
                        // Apply Cyrillic keymap if enabled
                        let (b1, b2) = if G_CYRILLIC {
                            features::keymap::latin_to_cyrillic(c)
                        } else {
                            (0, 0)
                        };
                        if b1 != 0 {
                            // Cyrillic character: 2 UTF-8 bytes
                            if line_len + 1 < LINE_MAX {
                                for j in (cursor..line_len).rev() {
                                    line[j + 2] = line[j];
                                }
                                line[cursor] = b1;
                                line[cursor + 1] = b2;
                                line_len += 2;
                                syscalls::write(1, line.as_ptr().add(cursor), line_len - cursor);
                                cursor += 2;
                                for _ in 0..(line_len - cursor) {
                                    syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                                }
                            }
                        } else if line_len < LINE_MAX - 1 {
                            // Latin character
                            for j in (cursor..line_len).rev() {
                                line[j + 1] = line[j];
                            }
                            line[cursor] = c;
                            line_len += 1;
                            syscalls::write(1, line.as_ptr().add(cursor), line_len - cursor);
                            cursor += 1;
                            for _ in 0..(line_len - cursor) {
                                syscalls::write(1, b"\x1B[D".as_ptr(), 3);
                            }
                        }
                        i += 1;
                        continue;
                    }
                    _ => {
                        i += 1;
                        continue;
                    }
                }
            }
        }
        line[line_len] = 0;
        if line_len == 0 {
            continue;
        }
        features::history_push(&line[..line_len]);
        if eval::has_op(&line[..line_len]) {
            static mut G_RAW: [u8; LINE_MAX] = [0u8; LINE_MAX];
            let n = line_len.min(LINE_MAX - 1);
            G_RAW[..n].copy_from_slice(&line[..n]);
            G_RAW[n] = 0;
            let p = pipeline::parse(&G_RAW[..n]);
            pipeline::execute(&G_RAW[..n], &p);
            continue;
        }
        eval::eval_line(&line[..line_len]);
    }
}
