#![no_std]
#![no_main]
#![allow(
    non_snake_case,
    reason = "SYS_* constants (syscalls/consts.rs) mirror Linux/kernel ABI syscall-name casing"
)]
#![allow(
    clippy::missing_safety_doc,
    reason = "TODO(2026-09-13): crate-wide backstop for older unsafe fns predating the per-fn `# Safety` convention; burn down opportunistically"
)]
#![allow(
    static_mut_refs,
    reason = "bare-static shell state (cwd, job table) is pervasive in this no_std binary; per-site migration to safe wrappers is tracked separately, not a stub"
)]

mod commands;
mod eval;
mod features;
mod io;
mod path;
mod pipeline;
mod repl;
mod syntax;
mod syscalls;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start(argc: usize, argv: *const *const u8) -> ! {
    unsafe {
        if argc > 1 && !argv.is_null() {
            let script_ptr = *argv.add(1);
            if !script_ptr.is_null() {
                let mut path = [0u8; 256];
                let mut i = 0;
                while *script_ptr.add(i) != 0 && i < 255 {
                    path[i] = *script_ptr.add(i);
                    i += 1;
                }
                commands::do_script(&path[..i]);
            }
            syscalls::exit(0);
        }

        syscalls::write(1, repl::VERSION_BANNER.as_ptr(), repl::VERSION_BANNER.len());
        features::env_init();

        let raw_ok = syscalls::enable_raw_mode() == 0;
        if raw_ok {
            repl::raw::raw_mode_repl();
        } else {
            repl::cooked::cooked_mode_repl();
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let _ = syscalls::disable_raw_mode();
        io::write_str("osh: internal panic — halting\n");
        syscalls::exit(101);
    }
}
