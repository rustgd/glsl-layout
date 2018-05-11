#![allow(non_camel_case_types)]
//!
//! This crates provides data types to build structures ready to upload into UBO.
//! Data layout will match one for uniform blocks declared with `layout(std140)`.
//! See [specs](https://www.khronos.org/registry/OpenGL/specs/gl/glspec45.core.pdf#page=159) for alignment rules.
//!
//! # Examples
//!
//! ```rust
//! # #[macro_use]
//! # extern crate glsl_layout;
//! # use glsl_layout::*;
//! # fn main() {
//! #[derive(Copy, Clone, Debug, Uniform)]    
//! struct Foo {
//!     x: int,
//!     y: dvec3,
//!     z: mat4x4,
//! }
//! let foo = Foo { x: 4, y: [0.0; 3].into(), z: [[0.0; 4]; 4].into() };
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

#[cfg(feature="cgmath")]
mod cgmath;

pub use scalar::*;
pub use vec::*;
pub use array::*;
pub use mat::*;
pub use uniform::*;

#[allow(unused_imports)]
#[macro_use]
extern crate glsl_layout_derive;
#[doc(hidden)]
pub use glsl_layout_derive::*;
