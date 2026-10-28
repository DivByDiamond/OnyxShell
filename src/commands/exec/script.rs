use crate::features;
use crate::io;
use crate::path;
use crate::syscalls;

use super::parse_job_id;

const SCRIPT_DEPTH_MAX: u8 = 8;
static mut SCRIPT_DEPTH: u8 = 0;

pub(crate) fn cmd_source(args: &[&[u8]]) {
    if args.is_empty() {
        io::write_error("source: missing file operand (try 'help')");
        return;
    }
    do_script(args[0]);
}

pub fn do_script(input: &[u8]) {
    unsafe {
        if SCRIPT_DEPTH >= SCRIPT_DEPTH_MAX {
            io::write_error("source: max recursion depth exceeded");
            return;
        }
        SCRIPT_DEPTH += 1;
    }
    do_script_inner(input);
    unsafe {
        SCRIPT_DEPTH -= 1;
    }
}

fn do_script_inner(input: &[u8]) {
    let mut abs = [0u8; path::PATH_MAX];
    let len = path::resolve(input, &mut abs);
    if len == 0 {
        io::write_error("source: path too long");
        return;
    }

    let fd = unsafe { syscalls::open(abs.as_ptr(), syscalls::O_RDONLY as u64, 0) };
    if fd < 0 {
        io::write_error_errno("source", fd);
        return;
    }

    let mut line_buf = [0u8; crate::commands::LINE_MAX];
    let mut line_pos = 0usize;
    loop {
        let mut c = [0u8; 1];
        let n = unsafe { syscalls::read_fd(fd as u64, c.as_mut_ptr(), 1) };
        if n <= 0 {
            break;
        }
        if c[0] == b'\n' {
            if line_pos > 0 && line_buf[0] != b'#' {
                execute_line(&line_buf[..line_pos]);
            }
            line_pos = 0;
        } else if c[0] != b'\r' && line_pos < crate::commands::LINE_MAX - 1 {
            line_buf[line_pos] = c[0];
            line_pos += 1;
        }
    }
    if line_pos > 0 && line_buf[0] != b'#' {
        execute_line(&line_buf[..line_pos]);
    }

    unsafe {
        syscalls::close(fd as u64);
    }
}

fn execute_line(line: &[u8]) {
    unsafe { crate::eval::eval_script_line(line); }
}

pub(crate) fn cmd_jobs(_args: &[&[u8]]) {
    unsafe {
        features::job_list();
    }
}

pub(crate) fn cmd_fg(args: &[&[u8]]) {
    if args.is_empty() {
        io::write_error("fg: usage: fg %<jobid>");
        return;
    }
    let job_id = parse_job_id(args[0]);
    if job_id == 0 {
        io::write_error("fg: invalid job specifier (use %<number>)");
        return;
    }
    unsafe {
        if let Some((id, pid, _running)) = features::job_find_by_id(job_id) {
            let mut status: i32 = 0;
            syscalls::waitpid(pid as u64, &mut status, 0);
            io::write_raw(b"[");
            io::write_u64(id as u64);
            io::write_raw(b"] ");
            io::write_i64(pid as i64);
            io::write_raw(b" Done\n");
            features::job_remove_by_id(id);
        } else {
            io::write_error("fg: job not found");
        }
    }
}

pub(crate) fn cmd_bg(args: &[&[u8]]) {
    if args.is_empty() {
        io::write_error("bg: usage: bg %<jobid>");
        return;
    }
    let job_id = parse_job_id(args[0]);
    if job_id == 0 {
        io::write_error("bg: invalid job specifier (use %<number>)");
        return;
    }
    unsafe {
        if let Some((id, pid, running)) = features::job_find_by_id(job_id) {
            if !running {
                syscalls::kill(pid, syscalls::SIGCONT);
                features::job_set_running(id, true);
            }
            io::write_raw(b"[");
            io::write_u64(id as u64);
            io::write_raw(b"] ");
            io::write_i64(pid as i64);
            io::newline();
        } else {
            io::write_error("bg: job not found");
        }
    }
}
