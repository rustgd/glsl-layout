#![allow(non_camel_case_types)]
//!
//! This crates provides data types to build structures ready to upload into UBO.
//! Data layout will match one for uniform blocks declared with `layout(std140)`.
//!

/// Vector of 2, 3 or 4 values.
#[macro_export]
macro_rules! vector {
    ($type:ty; $size:tt) => {
        $crate::vec::GVec<[$type; $size]>
    }
}

/// Vector of 2, 3 or 4 boolean values.
/// `foo: bvec![N]` is equivalent to glsl's `bvecN foo;`
/// 
/// `bvec![N]` implements `From<[boolean; N]>`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: bvec![2] = [true.into(), false.into()].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! bvec {
    ($size:tt) => {vector![$crate::scalar::boolean; $size]}
}

/// Vector of 2, 3 or 4 integer values.
/// `foo: ivec![N]` is equivalent to glsl's `ivecN foo;`
///
/// `ivec![N]` implements `From<[int; N]>`.
/// With 'cgmath' feature `ivec![N]` also implements `From<cgmath::VectorN<int>>` where N is 2, 3 or 4.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: ivec![2] = [1i32, 2i32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! ivec {
    ($size:tt) => {vector![$crate::scalar::int; $size]}
}

/// Vector of 2, 3 or 4 unsigned integer values.
/// `foo: uvec![N]` is equivalent to glsl's `uvecN foo;`
///
/// `uvec![N]` implements `From<[uint; N]>`.
/// With 'cgmath' feature `uvec![N]` also implements `From<cgmath::VectorN<uint>>` where N is 2, 3 or 4.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: uvec![2] = [1u32, 2u32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! uvec {
    ($size:tt) => {vector![$crate::scalar::uint; $size]}
}

/// Vector of 2, 3 or 4 floating point values.
/// `foo: fvec![N]` is equivalent to glsl's `vecN foo;`
///
/// `fvec![N]` implements `From<[float; N]>`.
/// With 'cgmath' feature `fvec![N]` also implements `From<cgmath::VectorN<float>>` where N is 2, 3 or 4.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: fvec![2] = [1.0f32, 2.0f32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! fvec {
    ($size:tt) => {vector![$crate::scalar::float; $size]}
}

/// Vector of 2, 3 or 4 double-precision floating point values.
/// `foo: dvec![N]` is equivalent to glsl's `dvecN foo;`
///
/// `dvec![N]` implements `From<[double; N]>`.
/// With 'cgmath' feature `dvec![N]` also implements `From<cgmath::VectorN<double>>` where N is 2, 3 or 4.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: dvec![2] = [1.0f64, 2.0f64].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! dvec {
    ($size:tt) => {vector![$crate::scalar::double; $size]}
}

/// Array storage.
/// `foo: array![float; N]` is equivalent to glsl's `float foo[N];`.
///
/// `array![float; N]` implements `From<[float; N]`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # use glsl_std140::scalar::float;
/// # fn main() {
/// let x: array![float; 3] = [1.0f32, 2.0f32, 3.0f32].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! array {
    ($type:ty; $size:tt) => {
        $crate::array::GArray<[$type; $size]>
    }
}

/// Matrix of N * M values.
#[macro_export]
macro_rules! matrix {
    ($type:ty; $cols:tt * $rows:tt) => {
        array![vector![$type; $rows]; $cols]
    }
}

/// Matrix of N * M floating point values.
/// `foo: fmat![N * M]` is equivalent to glsl's `matNxM foo;`.
/// 
/// `fmat![N * M]` implements `From<[[float; M]; N]>`;
/// With 'cgmath' feature `fmat![N * N]` also implements `From<cgmath::MatrixN<float>>` where N is 2, 3 or 4.
///
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: fmat![2 * 3] = [[2.0f32; 3]; 2].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! fmat {
    ($cols:tt * $rows:tt) => {
        matrix!($crate::scalar::float; $cols * $rows)
    }
}

/// Matrix of N * M double-precision floating point values.
/// `foo: dmat![N * M]` is equivalent to glsl's `dmatNxM foo;`.
///
/// `dmat![N * M]` implements `From<[[double; M]; N]>`;
/// With 'cgmath' feature `dmat![N * N]` also implements `From<cgmath::MatrixN<double>>` where N is 2, 3 or 4.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # fn main() {
/// let x: dmat![2 * 3] = [[2.0f64; 3]; 2].into();
/// # }
/// ```
///
#[macro_export]
macro_rules! dmat {
    ($cols:tt * $rows:tt) => {
        matrix!($crate::scalar::double; $cols * $rows)
    }
}

/// Define structure to use upload to UBO.
///
/// ```rust
/// # #[macro_use] extern crate glsl_std140;
/// # use glsl_std140::scalar::int;
/// # fn main() {
/// uniform! { struct Foo {
///     x: int,
///     y: dvec![3],
///     z: fmat![4*4],
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

pub mod align;
pub mod scalar;
pub mod vec;
pub mod array;

#[cfg(test)]
mod tests;

#[cfg(feature="cgmath")]
mod cgmath;
