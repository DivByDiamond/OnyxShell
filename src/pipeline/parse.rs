use super::{MAX_ARGS_PER, MAX_SEGMENTS, Pipeline, Segment};
use crate::{io, syntax};

pub unsafe fn seg_to_args<'b>(
    line: &[u8],
    seg: &Segment,
    buf: &'b mut [[u8; 64]; MAX_ARGS_PER],
    args: &'b mut [&'b [u8]; MAX_ARGS_PER],
) -> &'b [&'b [u8]] {
    let mut lengths = [0usize; MAX_ARGS_PER];
    for i in 0..seg.n_args {
        let (offset, length) = seg.args[i];
        match syntax::decode(&line[offset..offset + length], &mut buf[i][..63]) {
            Ok((length, _)) => {
                lengths[i] = length;
                buf[i][length] = 0;
            }
            Err(error) => {
                io::write_error(error);
                return &args[..0];
            }
        }
    }
    for i in 0..seg.n_args {
        args[i] = &buf[i][..lengths[i]];
    }
    &args[..seg.n_args]
}

pub fn parse(line: &[u8]) -> Pipeline {
    let mut p = Pipeline {
        segments: [Segment {
            args: [(0, 0); MAX_ARGS_PER],
            n_args: 0,
        }; MAX_SEGMENTS],
        n_segments: 0,
        stdout_file: (0, 0),
        stdin_file: (0, 0),
        background: false,
    };
    let mut cursor = 0;
    loop {
        let token = match syntax::next(line, &mut cursor) {
            Ok(Some(token)) => token,
            Ok(None) => break,
            Err(error) => {
                io::write_error(error);
                p.n_segments = 0;
                return p;
            }
        };
        match token.kind {
            syntax::Kind::Word => {
                if p.n_segments == 0 {
                    p.n_segments = 1;
                }
                let segment = &mut p.segments[p.n_segments - 1];
                if segment.n_args == MAX_ARGS_PER {
                    io::write_error("too many pipeline arguments");
                    p.n_segments = 0;
                    return p;
                }
                segment.args[segment.n_args] = (token.start, token.len);
                segment.n_args += 1;
            }
            syntax::Kind::Pipe => {
                if p.n_segments == 0
                    || p.segments[p.n_segments - 1].n_args == 0
                    || p.n_segments == MAX_SEGMENTS
                {
                    io::write_error("invalid pipeline");
                    p.n_segments = 0;
                    return p;
                }
                p.n_segments += 1;
            }
            syntax::Kind::Input | syntax::Kind::Output => match syntax::next(line, &mut cursor) {
                Ok(Some(file)) if file.kind == syntax::Kind::Word => {
                    let span = (file.start, file.len);
                    if token.kind == syntax::Kind::Input {
                        p.stdin_file = span;
                    } else {
                        p.stdout_file = span;
                    }
                }
                _ => {
                    io::write_error("missing redirect filename");
                    p.n_segments = 0;
                    return p;
                }
            },
            syntax::Kind::Background => {
                if !matches!(syntax::next(line, &mut cursor), Ok(None)) {
                    io::write_error("background marker must end command");
                    p.n_segments = 0;
                    return p;
                }
                p.background = true;
                break;
            }
        }
    }
    if p.n_segments > 0 && p.segments[p.n_segments - 1].n_args == 0 {
        io::write_error("missing pipeline command");
        p.n_segments = 0;
    }
    p
}
