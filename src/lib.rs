#![allow(non_camel_case_types)]
//!
//! This crates provides data types to build structures ready to upload into UBO.
//! Data layout will match one for uniform blocks declared with `layout(std140)`.
//! See [specs](https://www.khronos.org/registry/OpenGL/specs/gl/glspec45.core.pdf#page=159) for alignment rules.
//!
//! # Examples
//!
//! ```rust
//! # use glsl_layout::{Uniform, int, mat4x4, vec3, float};
//! 
//! #[derive(Debug, Default, Clone, Copy, Uniform)]
//! struct Foo {
//!     x: int,
//!     y: vec3,
//!     z: float,
//!     w: mat4x4,
//!     a: [f32; 3],
//!     b: f32,
//! }
//!
//! type UFoo = <Foo as Uniform>::Std140;
//!
//! assert_eq!(
//!     std::mem::offset_of!(UFoo, y),
//!     round_up_to(size_of::<int>(), 16), // `vec3` has alignment of size `vec4`
//!     "Offset of field `y` must be equal of size of `x` rounded up to the alignment",
//! );
//!
//! assert_eq!(
//!     std::mem::offset_of!(UFoo, z),
//!     round_up_to(std::mem::offset_of!(UFoo, y) + size_of::<vec3>(), 4),
//!     "Field `z` must follow `y`. `y` should not have padding at the end",
//! );
//!
//! assert_eq!(
//!     std::mem::offset_of!(UFoo, b),
//!     std::mem::offset_of!(UFoo, a) + size_of::<[[f32; 4]; 3]>(),
//!     "Field `b` must follow `a`. But `a` has padding at the end.",
//! );
//! 
//! let foo_uniform = Foo {
//!     x: 2,
//!     y: [0.0; 3].into(),
//!     z: 0.0,
//!     w: [[0.0; 4]; 4].into(),
//!     a: [0.0; 3].into(),
//!     b: 0.0,
//! }.std140();
//!
//! # fn round_up_to(offset: usize, align: usize) -> usize {
//! #     if offset % align == 0 {
//! #         offset
//! #     } else {
//! #         (((offset - 1) / align) + 1) * align
//! #     }
//! # }
//! ```
//!

#[doc(hidden)]
pub mod align;
mod scalar;
mod vec;

#[macro_use]
mod array;
mod mat;
mod uniform;

#[cfg(feature = "cgmath")]
mod cgmath;

#[cfg(feature = "nalgebra")]
mod nalgebra;

#[cfg(feature = "glam")]
mod glam;

pub use array::*;
pub use mat::*;
pub use scalar::*;
pub use uniform::*;
pub use vec::*;

#[allow(unused_imports)]
#[macro_use]
extern crate glsl_layout_derive;
#[doc(hidden)]
pub use glsl_layout_derive::*;

#[test]
fn test_derive() {
    use crate as glsl_layout;
    #[derive(Copy, Clone, Uniform)]
    struct Test {
        a: [u32; 3],
        b: vec2,
        c: dmat4x3,
    }
}

#[test]
fn test_array() {
    use crate as glsl_layout;
    #[derive(Copy, Clone, Uniform)]
    struct Test {
        a: [u32; 3],
        b: vec2,
        c: [dmat4x3; 32],
    }
}
