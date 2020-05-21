use std::mem;
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
    unsafe {
        let mut frequency = mem::zeroed();
        cvt(QueryPerformanceFrequency(&mut frequency)).unwrap();
        frequency
    }
}

fn query() -> LARGE_INTEGER {
    unsafe {
    let mut qpc_value: LARGE_INTEGER = mem::zeroed();
    cvt(QueryPerformanceCounter(&mut qpc_value)).unwrap();
    qpc_value
    }
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
