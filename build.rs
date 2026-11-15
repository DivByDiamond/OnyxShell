use std::{collections::BTreeSet, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    if std::env::var("TARGET")? == "riscv64gc-unknown-none-elf" {
        println!(
            "cargo:rustc-link-arg-bin=onyx-osh=-T{}",
            manifest.join("linker.ld").display()
        );
    }
    println!("cargo:rerun-if-changed=linker.ld");

    // The checked-in snapshot stays byte-for-byte compatible with the kernel
    // generator. Build idiomatic constants only for the shell's actual wrappers.
    let mut used = BTreeSet::new();
    for file in ["mod.rs", "proc.rs", "tty.rs"] {
        let path = manifest.join("src/syscalls/io").join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        let source = fs::read_to_string(path)?;
        for token in source.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
            if token.starts_with("SYS_") {
                used.insert(token.to_owned());
            }
        }
    }
    println!("cargo:rerun-if-changed=src/syscalls/numbers.rs");
    let snapshot = fs::read_to_string(manifest.join("src/syscalls/numbers.rs"))?;
    let mut constants = String::new();
    for line in snapshot
        .lines()
        .filter(|line| line.starts_with("pub const "))
    {
        let name = line
            .split_whitespace()
            .nth(2)
            .ok_or("invalid syscall constant")?
            .trim_end_matches(':');
        let upper = name.to_ascii_uppercase();
        if used.remove(&upper) {
            constants.push_str(&line.replace(name, &upper));
            constants.push('\n');
        }
    }
    if !used.is_empty() {
        return Err(format!("syscalls missing from ABI snapshot: {used:?}").into());
    }
    fs::write(
        PathBuf::from(std::env::var("OUT_DIR")?).join("syscall_numbers.rs"),
        constants,
    )?;
    Ok(())
}
