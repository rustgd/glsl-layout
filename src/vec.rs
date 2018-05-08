
use array::ArrayFrom;
use scalar::{boolean, int, uint, float, double};

macro_rules! define_vec {
    ($(#[$($meta:meta),+])* $vec:ident => [$type:ty ; $size:tt] : $a:tt) => {
        #[repr(C, align($a))]
        #[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
        $(#[$($meta),+])*
        pub struct $vec([$type; $size]);

        impl<T> From<[T; $size]> for $vec
        where
            T: Into<$type> + 'static,
        {
            fn from(values: [T; $size]) -> Self {
                $vec(ArrayFrom::array_from(values))
            }
        }

        impl AsRef<[$type; $size]> for $vec {
            fn as_ref(&self) -> &[$type; $size] {
                &self.0
            }
        }

        impl AsMut<[$type; $size]> for $vec {
            fn as_mut(&mut self) -> &mut [$type; $size] {
                &mut self.0
            }
        }

        impl AsRef<[$type]> for $vec {
            fn as_ref(&self) -> &[$type] {
                &self.0
            }
        }

        impl AsMut<[$type]> for $vec {
            fn as_mut(&mut self) -> &mut [$type] {
                &mut self.0
            }
        }
    }
}

/// Vector of 2 boolean values.
/// `foo: bvec2` is equivalent to glsl's `bvec2 foo;`
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// use glsl_layout_std140::
/// # fn main() {
/// let x: bvec2 = [true.into(), false.into()].into();
/// # }
/// ```
///
define_vec!(#[derive(Ord, Eq, Hash)] bvec2 => [boolean;  2] : 8);
define_vec!(#[derive(Ord, Eq, Hash)] bvec3 => [boolean;  3] : 16);
define_vec!(#[derive(Ord, Eq, Hash)] bvec4 => [boolean;  4] : 16);



/// Vector of 2 integer values.
/// `foo: ivec2` is equivalent to glsl's `ivec2 foo;`
///
/// With 'cgmath' feature `ivec2` also implements `From<cgmath::Vector2<int>>`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// use glsl_layout_std140::
/// # fn main() {
/// let x: ivec2 = [1i32, 2i32].into();
/// # }
/// ```
///
define_vec!(#[derive(Ord, Eq, Hash)] ivec2 => [int;      2] : 8);
define_vec!(#[derive(Ord, Eq, Hash)] ivec3 => [int;      3] : 16);
define_vec!(#[derive(Ord, Eq, Hash)] ivec4 => [int;      4] : 16);



/// Vector of 2 unsigned integer values.
/// `foo: uvec2` is equivalent to glsl's `uvec2 foo;`
///
/// With 'cgmath' feature `uvec2` also implements `From<cgmath::Vector2<uint>>`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// use glsl_layout_std140::
/// # fn main() {
/// let x: uvec2 = [1u32, 2u32].into();
/// # }
/// ```
///
define_vec!(#[derive(Ord, Eq, Hash)] uvec2 => [uint;     2] : 8);
define_vec!(#[derive(Ord, Eq, Hash)] uvec3 => [uint;     3] : 16);
define_vec!(#[derive(Ord, Eq, Hash)] uvec4 => [uint;     4] : 16);




/// Vector of 2 floating point values.
/// `foo: vec2` is equivalent to glsl's `vec2 foo;`
///
/// With 'cgmath' feature `vec2` also implements `From<cgmath::Vector2<float>>`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// use glsl_layout_std140::
/// # fn main() {
/// let x: vec2 = [1.0f32, 2.0f32].into();
/// # }
/// ```
///
define_vec!( vec2 => [float;    2] : 8);
define_vec!( vec3 => [float;    3] : 16);
define_vec!( vec4 => [float;    4] : 16);


/// Vector of 2 double-precision floating point values.
/// `foo: dvec2` is equivalent to glsl's `dvec2 foo;`
///
/// With 'cgmath' feature `dvec2` also implements `From<cgmath::Vector2<double>>`.
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout_std140;
/// use glsl_layout_std140::
/// # fn main() {
/// let x: dvec2 = [1.0f64, 2.0f64].into();
/// # }
/// ```
///
define_vec!(dvec2 => [double;   2] : 16);
define_vec!(dvec3 => [double;   3] : 32);
define_vec!(dvec4 => [double;   4] : 32);
