//! External-command fallback: when a tokenized command line does not
//! match any osh builtin, try to spawn `/bin/<cmd>` as an .onx binary
//! before printing "command not found".
//!
//! This is what makes `obrowse <url>` work from osh: obrowse is not a
//! shell builtin, it is a separate .onx in /bin (dropped there by
//! OnyxOS/scripts/mk-onyxfs-disk.sh). Without this fallback the user
//! would have to type `run /bin/obrowse <url>` instead. The same path
//! applies to any other OnyxApps .onx in /bin (osnake, otop, ohttp,
//! vim, ...).
//!
//! Returns true if a `/bin/<cmd>` file exists (whether or not spawn
//! succeeds - a missing file is a "not found" answer, but a spawn
//! error is reported like `run` does). Returns false if no
//! `/bin/<cmd>` exists, so the caller can print the canonical
//! "command not found" message.

use crate::io;
use crate::path;
use crate::syscalls;

use super::build_argv;

pub(crate) fn try_external(args: &[&[u8]]) -> bool {
    if args.is_empty() {
        return false;
    }
    let cmd = args[0];
    if cmd.is_empty() || cmd.contains(&b'/') {
        return false;
    }
    let mut full = [0u8; path::PATH_MAX];
    let prefix = b"/bin/";
    let plen = prefix.len();
    if plen + cmd.len() + 1 > full.len() {
        return false;
    }
    full[..plen].copy_from_slice(prefix);
    full[plen..plen + cmd.len()].copy_from_slice(cmd);
    full[plen + cmd.len()] = 0;
    // Stat the candidate path: if it does not exist, fall through to
    // "command not found". If it does, spawn + wait, mirroring `run`.
    let mut st = [0u8; 256];
    if unsafe { syscalls::stat(full.as_ptr(), st.as_mut_ptr()) } < 0 {
        return false;
    }
    let mut argv_strs: [[u8; path::PATH_MAX]; super::super::MAX_ARGS] =
        [[0; path::PATH_MAX]; super::super::MAX_ARGS];
    let mut argv_ptrs = [0u64; super::super::MAX_ARGS + 1];
    let argc = build_argv(args, &mut argv_strs, &mut argv_ptrs);
    if argc == 0 {
        io::write_error("try_external: argument too long");
        return true;
    }
    let pid = unsafe { syscalls::spawn(full.as_ptr(), argv_ptrs.as_ptr(), 0) };
    if pid < 0 {
        io::write_error_errno("spawn", pid);
        return true;
    }
    let mut status: i32 = 0;
    let waited = unsafe { syscalls::wait(&mut status) };
    if waited < 0 {
        io::write_error_errno("wait", waited);
        return true;
    }
    if status != 0 {
        io::write_raw(b"osh: ");
        io::write_raw(cmd);
        io::write_raw(b": exited with code ");
        io::write_i64(status as i64);
        io::newline();
    }
    true
}
