#![no_std]
#![no_main]

mod commands;
mod eval;
mod features;
mod io;
mod path;
mod pipeline;
mod repl;
mod syntax;
mod syscalls;

/// Enter the shell with the kernel-provided process arguments.
///
/// # Safety
/// `argv` must contain `argc` readable pointers to NUL-terminated strings.
/// The kernel must enter once, on the process's sole thread, with a valid stack.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start(argc: usize, argv: *const *const u8) -> ! {
    // SAFETY: The kernel entry contract supplies valid process arguments.
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
    // SAFETY: Panic cleanup runs on the shell thread and exits the process.
    unsafe {
        let _ = syscalls::disable_raw_mode();
        io::write_str("osh: internal panic — halting\n");
        syscalls::exit(101);
    }
}
