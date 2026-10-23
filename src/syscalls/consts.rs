#![allow(
    dead_code,
    reason = "syscall-number table deliberately mirrors the full kernel ABI (same convention as onyx_init's/kernel's own syscall tables), not per-command dead weight; wire-up happens as shell features land"
)]
#![allow(
    non_upper_case_globals,
    reason = "TODO(2026-09-12): SYS_* constants mirror Linux/Kernel ABI naming, deliberately non-upper-case by convention"
)]

include!("numbers.rs");

pub const TIOCSRAW: u64 = 0x5421;
pub const TIOCRRAW: u64 = 0x5422;
pub const TIOCGRAW: u64 = 0x5423;

pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 1 << 6;
pub const O_TRUNC: u32 = 1 << 9;
pub const O_APPEND: u32 = 1 << 10;

pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;

pub const WNOHANG: u32 = 1;
pub const SIGCONT: i32 = 18;

// POSIX errno values, matching the kernel's syscall-boundary translation
// (onyx_core::errno::Errno::to_posix / libonyxc/include/io/errno.h) — NOT
// the kernel's internal compact ordinals. A raw syscall return is already
// POSIX-numbered by the time it reaches userspace.
pub const EPERM: i64 = -1;
pub const ENOENT: i64 = -2;
pub const EIO: i64 = -5;
pub const EBADF: i64 = -9;
pub const ECHILD: i64 = -10;
pub const EAGAIN: i64 = -11;
pub const ENOMEM: i64 = -12;
pub const EFAULT: i64 = -14;
pub const EBUSY: i64 = -16;
pub const EEXIST: i64 = -17;
pub const ENOTDIR: i64 = -20;
pub const EISDIR: i64 = -21;
pub const EINVAL: i64 = -22;
pub const ENOSPC: i64 = -28;
pub const EPIPE: i64 = -32;
pub const ERANGE: i64 = -34;
pub const ENOSYS: i64 = -38;
pub const ENOTEMPTY: i64 = -39;
pub const ELOOP: i64 = -40;
pub const EOVERFLOW: i64 = -75;

pub fn errno_str(ret: i64) -> &'static [u8] {
    match ret {
        EPERM => b"Permission denied",
        ENOENT => b"No such file or directory",
        EIO => b"I/O error",
        EBADF => b"Bad file descriptor",
        ECHILD => b"No child processes",
        EAGAIN => b"Resource temporarily unavailable",
        ENOMEM => b"Out of memory",
        EFAULT => b"Bad address",
        EBUSY => b"Device or resource busy",
        EEXIST => b"File exists",
        ENOTDIR => b"Not a directory",
        EISDIR => b"Is a directory",
        EINVAL => b"Invalid argument",
        ENOSPC => b"No space left on device",
        EPIPE => b"Broken pipe",
        ERANGE => b"Out of range",
        ENOSYS => b"Function not implemented",
        ENOTEMPTY => b"Directory not empty",
        ELOOP => b"Too many levels of symbolic links",
        EOVERFLOW => b"Value too large",
        _ => b"Unknown error",
    }
}
