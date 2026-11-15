use super::super::consts::*;
use core::arch::asm;

pub unsafe fn exit(code: u64) -> ! {
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_EXIT,
            in("a0") code,
        );
    }
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

pub unsafe fn yield_cpu() {
    let _ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_YIELD,
            lateout("a0") _ret,
        );
    }
}

pub unsafe fn getring() -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_GETRING,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn getuid() -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_GETUID,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn fork() -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_FORK,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn kill(pid: i32, sig: i32) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_KILL,
            in("a0") pid as usize,
            in("a1") sig as usize,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn execve(path: *const u8, argv: *const u64, envp: *const u64) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_EXECVE,
            in("a0") path as usize,
            in("a1") argv as usize,
            in("a2") envp as usize,
            lateout("a0") ret,
        );
    }
    ret
}

/// `SYS_REBOOT(cmd)`: 0 = power off, 1 = restart. Root only (the kernel ACL
/// requires ring <= 1); never returns on success, so any return is an error.
pub unsafe fn reboot(cmd: u64) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_REBOOT,
            in("a0") cmd as usize,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn spawn(path: *const u8, argv: *const u64, ring_hint: u8) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_SPAWN,
            in("a0") path as usize,
            in("a1") argv as usize,
            in("a2") ring_hint as usize,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn wait(status_out: *mut i32) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_WAIT,
            in("a0") status_out as usize,
            lateout("a0") ret,
        );
    }
    ret
}

pub unsafe fn waitpid(pid: u64, status_out: *mut i32, options: u32) -> i64 {
    let ret: i64;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_WAITPID,
            in("a0") pid,
            in("a1") status_out as usize,
            in("a2") options as usize,
            lateout("a0") ret,
        );
    }
    ret
}
