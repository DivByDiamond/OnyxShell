#![allow(
    dead_code,
    reason = "syscall wrapper library surface deliberately mirrors the full kernel ABI (same convention as onyx_init's/kernel's own syscall tables), not per-command dead weight; wire-up happens as shell features land"
)]
#![allow(
    non_upper_case_globals,
    reason = "TODO(2026-09-12): SYS_* constants mirror kernel ABI"
)]
#![allow(
    unused_imports,
    reason = "TODO(2026-09-12): re-export surface intentionally wider than current shell usage"
)]

pub mod comm;
pub mod consts;
pub mod io;

pub use consts::*;
pub use io::proc::*;
pub use io::tty::*;
pub use io::*;
