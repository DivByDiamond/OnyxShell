# Shell quoting host tests

From the OnyxShell root:

```sh
rustc --edition=2024 -A static_mut_refs --test tests/quoting/main.rs \
  -o /tmp/onyx-shell-quoting-tests
/tmp/onyx-shell-quoting-tests
bash build.sh
```

The lint setting mirrors the shell crate's existing policy for its global
environment storage. Tests import the production scanner, decoder, pipeline
parser and variable-expansion module; host I/O is replaced with test-only
helpers. They do not exercise kernel process execution or redirection I/O.

The parser supports single/double quotes, escaped characters, empty quoted
words and quote-aware `|`, `<`, `>` and trailing `&`. Quoted or escaped
words bypass globbing; single-quoted content bypasses variable/history and
tilde expansion. Adjacent quoted/unquoted portions form one argument.

This is not a complete POSIX shell: existing fixed line/argument limits
remain, any quoted portion suppresses globbing for the whole word, and
features such as command substitution, `&&`, `||` and `>>` are unsupported.
