use super::{LINE_MAX, PROMPT};
use crate::{eval, features, pipeline, syscalls};

unsafe fn clear_line(line: &[u8], _cursor: usize) {
    syscalls::write(1, b"\r".as_ptr(), 1);
    for _ in 0..(PROMPT.len() + line.len()) {
        syscalls::write(1, b" ".as_ptr(), 1);
    }
    syscalls::write(1, b"\r".as_ptr(), 1);
    syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
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
            let n = syscalls::read(0, rx_buf.as_mut_ptr(), rx_buf.len() as u64);
            if n == 0 {
                syscalls::yield_cpu();
                continue;
            }
            if n < 0 {
                syscalls::exit(0);
            }
            let n = n as usize;
            let mut i = 0;
            while i < n {
                let b = rx_buf[i];
                if b == 0x1B && i + 2 < n && rx_buf[i + 1] == b'[' {
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
                    // Printable ASCII, plus raw UTF-8 lead/continuation bytes
                    // (0x80..=0xFF): the line buffer stores and echoes bytes
                    // verbatim, so a multi-byte sequence (e.g. Cyrillic,
                    // 2 bytes per character in UTF-8) round-trips correctly
                    // as long as every byte of it is accepted here — the old
                    // `(0x20..0x7F)`-only check silently dropped every byte
                    // outside 7-bit ASCII, so typing in a non-Latin layout
                    // produced nothing at all.
                    c if (0x20..0x7F).contains(&c) || c >= 0x80 => {
                        if line_len < LINE_MAX - 1 {
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
