#![allow(
    dead_code,
    reason = "TODO(2026-09-12): shell syscall wrappers re-exported per-command; per-bin unused items live in other shell build contexts"
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
