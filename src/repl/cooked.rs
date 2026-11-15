use super::{LINE_MAX, PROMPT};
use crate::{eval, io, syscalls};

pub unsafe fn cooked_mode_repl() -> ! {
    let mut line = [0u8; LINE_MAX];
    unsafe {
        loop {
            syscalls::write(1, PROMPT.as_ptr(), PROMPT.len());
            let n = io::read_line(&mut line);
            if n == 0 {
                continue;
            }
            let mut end = n;
            while end > 0
                && (line[end - 1] == b'\n' || line[end - 1] == b'\r' || line[end - 1] == 0)
            {
                end -= 1;
            }
            if end == 0 {
                continue;
            }
            eval::eval_line(&line[..end]);
        }
    }
}
