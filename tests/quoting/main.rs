#[path = "../../src/syntax/mod.rs"]
mod syntax;
mod io {
    pub fn write_error(_message: &str) {}
    pub static OUTPUT: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());
    pub fn write_raw(bytes: &[u8]) {
        OUTPUT.lock().unwrap().extend_from_slice(bytes);
    }
    pub fn newline() {
        write_raw(b"\n");
    }
    pub fn is_whitespace(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }
}
mod features {
    pub mod buffer {
        pub use std::vec::Vec;
    }
}
#[path = "../../src/features/env.rs"]
mod env;
mod pipeline {
    pub const MAX_SEGMENTS: usize = 8;
    pub const MAX_ARGS_PER: usize = 16;
    #[derive(Clone, Copy)]
    pub struct Segment {
        pub args: [(usize, usize); MAX_ARGS_PER],
        pub n_args: usize,
    }
    pub struct Pipeline {
        pub segments: [Segment; MAX_SEGMENTS],
        pub n_segments: usize,
        pub stdout_file: (usize, usize),
        pub stdin_file: (usize, usize),
        pub background: bool,
    }
    pub mod parse {
        include!("../../src/pipeline/parse.rs");
    }
}

#[test]
fn quoted_lua_pipeline_and_redirect_filename_decode() {
    let line = br#"lua -e 'print("<|>", 6*7)'|cat > '/tmp/lua result' <"/tmp/input file" &"#;
    let parsed = pipeline::parse::parse(line);
    assert_eq!(parsed.n_segments, 2);
    assert!(parsed.background);
    let mut buffers = [[0; 64]; 16];
    let mut arguments: [&[u8]; 16] = [&[]; 16];
    let args = unsafe {
        pipeline::parse::seg_to_args(line, &parsed.segments[0], &mut buffers, &mut arguments)
    };
    assert_eq!(args, &[b"lua".as_slice(), b"-e", br#"print("<|>", 6*7)"#]);
    for (span, expected) in [
        (parsed.stdout_file, b"/tmp/lua result".as_slice()),
        (parsed.stdin_file, b"/tmp/input file".as_slice()),
    ] {
        let mut buffer = [0; 64];
        let length = syntax::decode(&line[span.0..span.0 + span.1], &mut buffer)
            .unwrap()
            .0;
        assert_eq!(&buffer[..length], expected);
    }
}

#[test]
fn ordinary_pipeline_and_invalid_operator_sequences() {
    let parsed = pipeline::parse::parse(b"echo hello | cat > /tmp/out");
    assert_eq!(parsed.n_segments, 2);
    assert_eq!(parsed.segments[0].n_args, 2);
    for line in [b"echo |".as_slice(), b"| cat", b"echo >", b"echo > | cat"] {
        assert_eq!(pipeline::parse::parse(line).n_segments, 0);
    }
}

#[test]
fn real_variable_expansion_preserves_single_quoted_lua_and_quoted_values() {
    unsafe {
        env::env_init();
        assert_eq!(env::env_get(b"PATH"), Some(b"/bin".as_slice()));
        env::env_set(b"REMOVE", b"temporary");
        env::env_unset(b"REMOVE");
        assert_eq!(env::env_get(b"REMOVE"), None);
        let mut strings = [[0; env::ENV_KEY_MAX + env::ENV_VAL_MAX + 2]; env::ENV_MAX];
        let mut pointers = [0; env::ENV_MAX + 1];
        let count = env::build_envp(&mut strings, &mut pointers);
        assert_eq!(count, 2);
        assert_eq!(&strings[0][..17], b"HOME=/users/root\0");
        assert_eq!(pointers[0], strings[0].as_ptr() as u64);
        assert_eq!(pointers[count], 0);
        env::env_list();
        assert_eq!(
            io::OUTPUT.lock().unwrap().as_slice(),
            b"HOME=/users/root\nPATH=/bin\n"
        );
        env::env_set(b"HOME", b"/users/test");
        env::env_set(b"QTEST", b"a\"b|c");
        let expanded = env::expand_vars(br#"lua -e 'print("$QTEST", "~", "!42")' "$QTEST""#);
        let mut cursor = 0;
        let mut decoded = Vec::new();
        while let Some(token) = syntax::next(expanded.as_slice(), &mut cursor).unwrap() {
            assert_eq!(token.kind, syntax::Kind::Word);
            let mut buffer = [0; 128];
            let length =
                syntax::decode(&expanded[token.start..token.start + token.len], &mut buffer)
                    .unwrap()
                    .0;
            decoded.push(buffer[..length].to_vec());
        }
        assert_eq!(decoded[2], br#"print("$QTEST", "~", "!42")"#);
        assert_eq!(decoded[3], b"a\"b|c");
        assert_eq!(
            env::expand_tilde(b"echo '~' ~").as_slice(),
            b"echo '~' /users/test"
        );
    }
}
