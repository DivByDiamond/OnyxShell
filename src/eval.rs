use crate::repl::LINE_MAX;
use crate::{commands, features, io, pipeline, syntax};

pub fn has_op(s: &[u8]) -> bool {
    let mut cursor = 0;
    while let Ok(Some(token)) = syntax::next(s, &mut cursor) {
        if token.kind != syntax::Kind::Word {
            return true;
        }
    }
    false
}

pub unsafe fn eval_line(raw: &[u8]) {
    unsafe {
        evaluate(raw, true);
    }
}

pub unsafe fn eval_script_line(raw: &[u8]) {
    unsafe {
        evaluate(raw, false);
    }
}

unsafe fn evaluate(raw: &[u8], interactive: bool) {
    unsafe {
        if raw.is_empty() {
            return;
        }

        let hist = if interactive {
            features::history_expand(raw)
        } else {
            features::buffer::line_to_vec(raw)
        };
        let s = hist.as_slice();
        if s.is_empty() {
            return;
        }

        if interactive {
            features::history_push(s);
        }
        let expanded = features::expand_tilde(s);
        let expanded = features::expand_vars(expanded.as_slice());
        let s = expanded.as_slice();
        if s.is_empty() {
            return;
        }
        if has_op(s) {
            static mut G_LINE: [u8; LINE_MAX] = [0; LINE_MAX];
            if s.len() >= LINE_MAX {
                io::write_error("expanded line too long");
                return;
            }
            let n = s.len();
            G_LINE[..n].copy_from_slice(&s[..n]);
            let p = pipeline::parse(&G_LINE[..n]);
            pipeline::execute(&G_LINE[..n], &p);
            return;
        }

        static mut G_TOKEN_OFFSETS: [(usize, usize); 16] = [(0, 0); 16];
        let ntok = io::tokenize(s, &mut G_TOKEN_OFFSETS);
        if ntok == 0 {
            return;
        }

        static mut G_EXPANDED_ARGS: [[u8; 128]; 32] = [[0u8; 128]; 32];
        static mut G_ARGS: [&[u8]; 32] = [&[]; 32];
        let mut n_args = 0usize;
        for &(off, len) in &G_TOKEN_OFFSETS[..ntok] {
            let mut decoded = [0u8; 128];
            let (length, protected) = match syntax::decode(&s[off..off + len], &mut decoded[..127])
            {
                Ok(value) => value,
                Err(error) => {
                    io::write_error(error);
                    return;
                }
            };
            if protected {
                G_EXPANDED_ARGS[n_args] = decoded;
                G_ARGS[n_args] = &G_EXPANDED_ARGS[n_args][..length];
                n_args += 1;
                if n_args == 32 {
                    break;
                }
                continue;
            }
            let expansions = features::glob_expand(&decoded[..length]);
            for j in 0..expansions.len() {
                if n_args >= 32 {
                    break;
                }
                let exp = expansions[j];
                let mut elen = 0;
                while elen < 127 && exp[elen] != 0 {
                    elen += 1;
                }
                G_EXPANDED_ARGS[n_args] = exp;
                G_ARGS[n_args] = &G_EXPANDED_ARGS[n_args][..elen];
                n_args += 1;
            }
            if n_args >= 32 {
                break;
            }
        }
        commands::dispatch(&G_ARGS[..n_args]);
    }
}
