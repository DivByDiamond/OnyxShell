// features/keymap.rs — Cyrillic (ЙЦУКЕН) keyboard layout mapping.
//
// Maps Latin characters to their Russian keyboard layout equivalents.
// The mapping is applied in raw mode when the user types Latin letters,
// producing Cyrillic output. This allows typing Russian text in the shell.

/// Map a Latin ASCII character to its Russian keyboard layout equivalent.
/// Returns the Cyrillic UTF-8 bytes (up to 2 bytes) and their count.
/// Returns (0, 0) if the character has no Russian mapping.
pub fn latin_to_cyrillic(c: u8) -> (u8, u8) {
    match c {
        // Row 1: q→й w→ц e→у r→к t→е y→н u→г i→ш o→щ p→з [→х ]→ъ
        b'q' => (0xD0, 0xB9), // й
        b'w' => (0xD1, 0x86), // ц
        b'e' => (0xD1, 0x83), // у
        b'r' => (0xD0, 0xBA), // к
        b't' => (0xD0, 0xB5), // е
        b'y' => (0xD0, 0xBD), // н
        b'u' => (0xD0, 0xB3), // г
        b'i' => (0xD1, 0x88), // ш
        b'o' => (0xD1, 0x89), // щ
        b'p' => (0xD0, 0xB7), // з
        b'[' => (0xD1, 0x85), // х
        b']' => (0xD1, 0x8A), // ъ
        // Row 2: a→ф s→ы d→в f→а g→п h→р j→о k→л l→д ;→ж '→э
        b'a' => (0xD1, 0x84),  // ф
        b's' => (0xD1, 0x8B),  // ы
        b'd' => (0xD0, 0xB2),  // в
        b'f' => (0xD0, 0xB0),  // а
        b'g' => (0xD0, 0xBF),  // п
        b'h' => (0xD1, 0x80),  // р
        b'j' => (0xD0, 0xBE),  // о
        b'k' => (0xD0, 0xBB),  // л
        b'l' => (0xD0, 0xB4),  // д
        b';' => (0xD0, 0xB6),  // ж
        b'\'' => (0xD1, 0x8D), // э
        // Row 3: z→я x→ч c→с v→м b→и n→т m→ь ,→б .→ю
        b'z' => (0xD1, 0x8F), // я
        b'x' => (0xD1, 0x87), // ч
        b'c' => (0xD1, 0x81), // с
        b'v' => (0xD0, 0xBC), // м
        b'b' => (0xD0, 0xB8), // и
        b'n' => (0xD1, 0x82), // т
        b'm' => (0xD1, 0x8C), // ь
        b',' => (0xD0, 0xB1), // б
        b'.' => (0xD1, 0x8E), // ю
        // Special: `→ё
        b'`' => (0xD1, 0x91), // ё
        _ => (0, 0),
    }
}

/// Check if a byte is the first byte of a 2-byte Cyrillic UTF-8 sequence.
pub fn is_cyrillic_first(b: u8) -> bool {
    b >= 0xD0 && b <= 0xD1
}
