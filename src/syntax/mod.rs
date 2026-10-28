//! Allocation-free shell word scanning and quote removal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Word,
    Pipe,
    Input,
    Output,
    Background,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub len: usize,
}

fn operator(byte: u8) -> Option<Kind> {
    match byte {
        b'|' => Some(Kind::Pipe),
        b'<' => Some(Kind::Input),
        b'>' => Some(Kind::Output),
        b'&' => Some(Kind::Background),
        _ => None,
    }
}
fn escaped_in_double(byte: u8) -> bool {
    matches!(byte, b'"' | b'\\' | b'$' | b'\n')
}

pub fn next(line: &[u8], cursor: &mut usize) -> Result<Option<Token>, &'static str> {
    while *cursor < line.len() && line[*cursor].is_ascii_whitespace() {
        *cursor += 1;
    }
    if *cursor == line.len() {
        return Ok(None);
    }
    let start = *cursor;
    if let Some(kind) = operator(line[start]) {
        *cursor += 1;
        return Ok(Some(Token {
            kind,
            start,
            len: 1,
        }));
    }
    let mut quote = 0;
    while *cursor < line.len() {
        let byte = line[*cursor];
        if quote == 0 && (byte.is_ascii_whitespace() || operator(byte).is_some()) {
            break;
        }
        if byte == b'\\' && quote != b'\'' {
            if *cursor + 1 == line.len() {
                return Err("trailing escape");
            }
            if quote == 0 || escaped_in_double(line[*cursor + 1]) {
                *cursor += 2;
                continue;
            }
        }
        if byte == quote {
            quote = 0;
        } else if quote == 0 && matches!(byte, b'\'' | b'"') {
            quote = byte;
        }
        *cursor += 1;
    }
    if quote != 0 {
        return Err("unterminated quote");
    }
    Ok(Some(Token {
        kind: Kind::Word,
        start,
        len: *cursor - start,
    }))
}

/// Remove quoting without dropping empty quoted words. The boolean protects
/// the whole word from globbing if any portion was quoted or escaped.
pub fn decode(raw: &[u8], output: &mut [u8]) -> Result<(usize, bool), &'static str> {
    let (mut i, mut length, mut quote, mut protected) = (0, 0, 0, false);
    while i < raw.len() {
        let mut byte = raw[i];
        if byte == b'\\' && quote != b'\'' {
            if i + 1 == raw.len() {
                return Err("trailing escape");
            }
            if quote == 0 || escaped_in_double(raw[i + 1]) {
                protected = true;
                i += 1;
                byte = raw[i];
                if byte == b'\n' {
                    i += 1;
                    continue;
                }
            }
        } else if byte == quote {
            quote = 0;
            protected = true;
            i += 1;
            continue;
        } else if quote == 0 && matches!(byte, b'\'' | b'"') {
            quote = byte;
            protected = true;
            i += 1;
            continue;
        }
        if length == output.len() {
            return Err("argument too long");
        }
        output[length] = byte;
        length += 1;
        i += 1;
    }
    if quote != 0 {
        return Err("unterminated quote");
    }
    Ok((length, protected))
}

/// Expansion eligibility in the original spelling. Variables/history may
/// expand in double quotes; tilde expansion must be entirely unquoted.
pub fn expandable(line: &[u8], at: usize, double_quotes: bool) -> bool {
    let (mut i, mut quote) = (0, 0);
    while i < at {
        let byte = line[i];
        if byte == b'\\'
            && quote != b'\''
            && i + 1 < line.len()
            && (quote == 0 || escaped_in_double(line[i + 1]))
        {
            if i + 1 == at {
                return false;
            }
            i += 2;
            continue;
        }
        if byte == quote {
            quote = 0;
        } else if quote == 0 && matches!(byte, b'\'' | b'"') {
            quote = byte;
        }
        i += 1;
    }
    quote == 0 || (double_quotes && quote == b'"')
}

#[cfg(test)]
mod tests;
