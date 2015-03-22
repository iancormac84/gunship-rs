use std::cmp::{PartialEq, Eq};
use std::ops::{Index, IndexMut};

/// A 4x4 matrix that can be used to transform 3D points and vectors.
///
/// Matrices are row-major.
#[repr(C)] #[derive(Copy)]
pub struct Matrix4 {
    data: [f32; 16]
}

impl Matrix4 {

    /// Create a new identity matrix.
    pub fn new() -> Matrix4 {
        Matrix4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    /// Create a new translation matrix.
    pub fn from_translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {
            data: [
                1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    /// Get the matrix data as a raw array.
    ///
    /// This is meant to be used for ffi and when passing
    /// matrix data to the graphics card, it should not
    /// be used to directly manipulate the contents of the matrix.
    pub unsafe fn raw_data(&self) -> *const f32
    {
        &self.data[0]
    }
}

impl PartialEq for Matrix4 {
    fn ne(&self, other: &Matrix4) -> bool {
        for (&ours, &theirs) in self.data.iter().zip(other.data.iter()) {
            if ours != theirs {
                return true
            }
        }
        false
    }

    fn eq(&self, other: &Matrix4) -> bool {
        !(self != other)
    }
}

impl Eq for Matrix4 {}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;

    fn index<'a>(&'a self, index: &(usize, usize)) -> &'a f32 {
        let &(row, col) = index;
        assert!(row < 4 && col < 4);
        &self.data[row * 4 + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut<'a>(&'a mut self, index: &(usize, usize)) -> &'a mut f32 {
        let &(row, col) = index;
        assert!(row < 4 && col < 4);
        &mut self.data[row * 4 + col]
    }
}