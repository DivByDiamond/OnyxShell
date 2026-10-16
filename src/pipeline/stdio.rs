//! Restore only streams this command actually redirects.
use crate::syscalls;

pub struct Redirect { saved: [Option<u64>; 2] }
impl Redirect {
    pub unsafe fn new(input: i64, output: i64) -> Result<Self, i64> {
        let mut scope = Self { saved: [None; 2] };
        for (target, source) in [(0, input), (1, output)] {
            if source == target as i64 { continue; }
            let saved = unsafe { syscalls::save_stdio(target as u64) }?;
            scope.saved[target] = Some(saved);
            let result = unsafe { syscalls::bind_stdio(source as u64, target as u64) };
            if result < 0 { return Err(result); }
        }
        Ok(scope)
    }
}
impl Drop for Redirect {
    fn drop(&mut self) {
        for (target, saved) in self.saved.iter().enumerate() {
            if let Some(saved) = *saved { unsafe {
                syscalls::bind_stdio(saved, target as u64);
                if saved != u64::MAX { syscalls::close(saved); }
            } }
        }
    }
}
