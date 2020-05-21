use winapi::um::{processthreadsapi::GetCurrentThreadId, profileapi::{QueryPerformanceFrequency, QueryPerformanceCounter}};

/// Gets the current timestamp in microseconds.
pub fn timestamp() -> i64 {
    let mut frequency = 0;
    if unsafe { QueryPerformanceFrequency(*mut frequency) } == 0 {
        panic!("Failed to query performance frequency");
    }

    let mut counter = 0;
    if unsafe { QueryPerformanceCounter(*mut counter) } == 0 {
        panic!("Failed to query performance counter");
    }

    counter * 1_000_000 / frequency
}

pub fn thread_id() -> usize {
    unsafe { GetCurrentThreadId() as usize }
}
