// Using `RawVec<T>`, could be replaced.
#![feature(alloc)]
#![feature(raw_vec_internals)]
// Almost certainly going to be stabilized as-is, unlikely to break anything.
#![feature(const_fn)]
// The scheduler puts a `Condvar` and `Mutex` into some statics.
#![feature(drop_types_in_const)]
// Useful when sending raw pointers between threads, could be replaced.
#![feature(ptr_internals)]

extern crate alloc;

#[macro_use]
pub mod macros;

pub mod camera;
pub mod collections;
pub mod engine;
pub mod input;
pub mod light;
pub mod mesh_renderer;
pub mod prelude;
pub mod resource;
pub mod scheduler;
pub mod time;
pub mod transform;
