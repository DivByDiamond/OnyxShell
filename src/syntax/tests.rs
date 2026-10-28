use super::*;

fn words(line: &[u8]) -> Vec<Vec<u8>> {
    let mut cursor = 0;
    let mut words = Vec::new();
    while let Some(token) = next(line, &mut cursor).unwrap() {
        assert_eq!(token.kind, Kind::Word);
        let mut buffer = [0; 256];
        let (length, _) = decode(&line[token.start..token.start + token.len], &mut buffer).unwrap();
        words.push(buffer[..length].to_vec());
    }
    words
}

#[test]
fn lua_code_is_one_argument_and_operators_stay_literal() {
    let parsed = words(br#"lua -e 'print("QEMU_LUA_OK",6*7); print(1<2, "a|b>c")'"#);
    assert_eq!(parsed.len(), 3);
    assert_eq!(
        parsed[2],
        br#"print("QEMU_LUA_OK",6*7); print(1<2, "a|b>c")"#
    );
}

#[test]
fn empty_adjacent_and_escaped_words() {
    let parsed = words(br#"echo '' "" pre' two 'post one\ two "a\"b\\c" '<|>'"#);
    assert_eq!(
        parsed,
        vec![
            b"echo".to_vec(),
            vec![],
            vec![],
            b"pre two post".to_vec(),
            b"one two".to_vec(),
            b"a\"b\\c".to_vec(),
            b"<|>".to_vec()
        ]
    );
}

#[test]
fn quote_and_escape_disable_glob_for_the_word() {
    for raw in [b"'*'".as_slice(), b"\"?\"", b"\\*", b"a'['b"] {
        assert!(decode(raw, &mut [0; 32]).unwrap().1);
    }
    assert!(!decode(b"*.lua", &mut [0; 32]).unwrap().1);
}

#[test]
fn malformed_quotes_escapes_and_buffer_overflow_are_errors() {
    for raw in [
        b"'unterminated".as_slice(),
        b"\"unterminated",
        b"trailing\\",
    ] {
        assert!(next(raw, &mut 0).is_err());
        assert!(decode(raw, &mut [0; 32]).is_err());
    }
    assert_eq!(decode(b"abcd", &mut [0; 3]), Err("argument too long"));
}

#[test]
fn single_quotes_and_escapes_protect_expansion() {
    let line = br#"echo '$HOME !42 ~' "$HOME" \$HOME"#;
    let positions: Vec<_> = line
        .iter()
        .enumerate()
        .filter(|(_, byte)| **byte == b'$')
        .map(|(index, _)| index)
        .collect();
    assert!(!expandable(line, positions[0], true));
    assert!(expandable(line, positions[1], true));
    assert!(!expandable(line, positions[1], false));
    assert!(!expandable(line, positions[2], true));
}
