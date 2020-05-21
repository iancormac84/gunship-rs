// Using `RawVec<T>`, could be replaced.
#![feature(raw_vec_internals)]
// Almost certainly going to be stabilized as-is, unlikely to break anything.
#![feature(const_fn)]
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

pub mod math {
    pub use polygon_math::*;
}

pub mod stopwatch {
    pub use stopwatch::*;
}
