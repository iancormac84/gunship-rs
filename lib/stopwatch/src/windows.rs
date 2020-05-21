use std::{mem, sync::atomic::{AtomicUsize, Ordering::SeqCst}};
use winapi::um::{processthreadsapi::GetCurrentThreadId, profileapi::{QueryPerformanceFrequency, QueryPerformanceCounter}, winnt::LARGE_INTEGER};

pub trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ($($t:ident)*) => ($(impl IsZero for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }
    })*)
}

impl_is_zero! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

pub fn cvt<I: IsZero>(i: I) -> std::io::Result<I> {
    if i.is_zero() { Err(std::io::Error::last_os_error()) } else { Ok(i) }
}

fn frequency() -> LARGE_INTEGER {
        static mut FREQUENCY: LARGE_INTEGER = unsafe { mem::zeroed() };
        static STATE: AtomicUsize = AtomicUsize::new(0);

        unsafe {
            // If a previous thread has filled in this global state, use that.
            if STATE.load(SeqCst) == 2 {
                return FREQUENCY;
            }

            // ... otherwise learn for ourselves ...
            let mut frequency = mem::zeroed();
            cvt(QueryPerformanceFrequency(&mut frequency)).unwrap();

            // ... and attempt to be the one thread that stores it globally for
            // all other threads
            if STATE.compare_exchange(0, 1, SeqCst, SeqCst).is_ok() {
                FREQUENCY = frequency;
                STATE.store(2, SeqCst);
            }
            frequency
        }
    }

    fn query() -> LARGE_INTEGER {
        let mut qpc_value: LARGE_INTEGER = unsafe { mem::zeroed() };
        cvt(unsafe { QueryPerformanceCounter(&mut qpc_value) }).unwrap();
        qpc_value
    }

/// Gets the current timestamp in microseconds.
pub fn timestamp() -> i64 {
    let frequency = frequency();
    let qpc_value = query();

    unsafe {
        *qpc_value.QuadPart() * 1_000_000 / *frequency.QuadPart()
    }
}

pub fn thread_id() -> usize {
    unsafe { GetCurrentThreadId() as usize }
}
