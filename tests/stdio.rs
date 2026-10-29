//! Run with: rustc --test tests/stdio.rs -o /tmp/onyx-stdio-test
use std::sync::Mutex;
static STATE: Mutex<State> = Mutex::new(State {
    saves: vec![],
    bindings: vec![],
    closed: vec![],
    fail: None,
    pos: [0; 2],
    saved_pos: [0; 2],
});
struct State {
    saves: Vec<u64>,
    bindings: Vec<(u64, u64)>,
    closed: Vec<u64>,
    fail: Option<u64>,
    pos: [u64; 2],
    saved_pos: [u64; 2],
}
mod syscalls {
    pub unsafe fn save_stdio(target: u64) -> Result<u64, i64> {
        let mut s = super::STATE.lock().unwrap();
        s.saves.push(target);
        s.saved_pos[target as usize] = s.pos[target as usize];
        if s.fail == Some(target) {
            Err(-9)
        } else {
            Ok(100 + target)
        }
    }
    pub unsafe fn bind_stdio(source: u64, target: u64) -> i64 {
        let mut s = super::STATE.lock().unwrap();
        s.bindings.push((source, target));
        if source == 100 + target {
            s.pos[target as usize] = s.saved_pos[target as usize];
        }
        0
    }
    pub unsafe fn close(fd: u64) -> i64 {
        super::STATE.lock().unwrap().closed.push(fd);
        0
    }
}
#[path = "../src/pipeline/stdio.rs"]
mod stdio;
#[test]
fn redirects_only_changed_streams_and_preserves_bindings_when_save_fails() {
    // Input-only redirection must not snapshot/rewind inherited file stdout.
    for _ in 0..2 {
        let _scope = unsafe { stdio::Redirect::new(20, 1) }.unwrap();
        STATE.lock().unwrap().pos[1] += 2; // pwd writes two bytes to inherited stdout.
    }
    {
        let mut s = STATE.lock().unwrap();
        assert_eq!(s.pos[1], 4); // No position rollback/overwriting the previous pwd.
        assert_eq!(s.saves, vec![0, 0]);
        assert_eq!(s.bindings, vec![(20, 0), (100, 0), (20, 0), (100, 0)]);
        assert_eq!(s.closed, vec![100, 100]);
        s.saves.clear();
        s.bindings.clear();
        s.closed.clear();
    }
    // Output-only redirection must not snapshot/rewind inherited file stdin.
    {
        let _scope = unsafe { stdio::Redirect::new(0, 21) }.unwrap();
    }
    {
        let mut s = STATE.lock().unwrap();
        assert_eq!(s.saves, vec![1]);
        assert_eq!(s.bindings, vec![(21, 1), (101, 1)]);
        s.saves.clear();
        s.bindings.clear();
        s.closed.clear();
        s.fail = Some(1);
    }
    // Allocation failure is an error, never permission to reset stdout to UART.
    assert!(unsafe { stdio::Redirect::new(0, 21) }.is_err());
    {
        let s = STATE.lock().unwrap();
        assert!(s.bindings.is_empty());
        assert!(s.closed.is_empty());
    }
    // Failure on the second stream unwinds a successful first binding.
    assert!(unsafe { stdio::Redirect::new(20, 21) }.is_err());
    {
        let s = STATE.lock().unwrap();
        assert_eq!(s.bindings, vec![(20, 0), (100, 0)]);
    }
}
