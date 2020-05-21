// Using `RawVec<T>`, could be replaced.
#![feature(alloc)]
// Almost certainly going to be stabilized as-is, unlikely to break anything.
#![feature(const_fn)]
// The scheduler puts a `Condvar` and `Mutex` into some statics.
#![feature(drop_types_in_const)]
// Used by the scheduler for handling work. We might be able to remove that with some unsafe magic,
// but even then being able to box a `FnOnce()` is valuable, so this is unlikely to go away.
#![feature(fnbox)]
// Useful when sending raw pointers between threads, could be replaced.
#![feature(unique)]

//extern crate bootstrap_audio as bs_audio;
use bootstrap_rs as bootstrap;
use parse_obj as obj;
use polygon_math as math;

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
