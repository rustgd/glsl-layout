#![allow(non_camel_case_types)]
//!
//! This crates provides data types to build structures ready to upload into UBO.
//! Data layout will match one for uniform blocks declared with `layout(std140)`.
//!

/// Array storage.
/// `foo: array![float; N]` is equivalent to glsl's `float foo[N];`.
///
/// `array![float; N]` implements `From<[float; N]`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// # use glsl_layout_std140::scalar::float;
/// # fn main() {
/// let x: array![float; 3] = [1.0f32, 2.0f32, 3.0f32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! array {
    ($type:ty; $size:tt) => {
        $crate::array::Array<[$crate::array::Element<$type>; $size]>
    }
}

/// Define structure to use upload to UBO.
///
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// # use glsl_layout_std140::scalar::int;
/// # use glsl_layout_std140::vec::dvec3;
/// # use glsl_layout_std140::mat::mat4x4;
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

pub mod scalar;
pub mod vec;
pub mod array;
pub mod mat;

#[cfg(test)]
mod tests;

#[cfg(feature="cgmath")]
mod cgmath;
