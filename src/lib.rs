#![allow(non_camel_case_types)]
//!
//! This crates provides data types to build structures ready to upload into UBO.
//! Data layout will match one for uniform blocks declared with `layout(std140)`.
//! See [https://www.khronos.org/registry/OpenGL/specs/gl/glspec45.core.pdf#page=159](specs) for alignment rules.
//!

/// Array storage.
/// `foo: array![float; N]` is equivalent to glsl's `float foo[N];`.
///
/// `array![float; N]` implements `From<[float; N]`.
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::float;
/// # fn main() {
/// let x: array![float; 3] = [1.0f32, 2.0f32, 3.0f32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! array {
    ($type:ty; $size:tt) => {
        $crate::Array<[$crate::Element<$type>; $size]>
    }
}

/// Define structure to use upload to UBO.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::{int, dvec3, mat4x4};
/// # fn main() {
/// uniform! { struct Foo {
///     x: int,
///     y: dvec3,
///     z: mat4x4,
/// }};
/// # }
/// ```
#[macro_export]
macro_rules! uniform {
    ($(#[$($meta:meta),*])* struct $name:ident { $($fname:ident: $ftype:ty),* $(,)* }) => {
        $(#[$($meta),*])*
        #[repr(C, align(16))]
        struct $name {
            $(
                $fname: $ftype,
            )*
        }
    }
}

mod scalar;
mod vec;
mod array;
mod mat;

#[cfg(test)]
mod tests;

#[cfg(feature="cgmath")]
mod cgmath;

pub use scalar::*;
pub use vec::*;
pub use array::*;
pub use mat::*;
