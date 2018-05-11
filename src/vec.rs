
use align::{Align8, Align16, Align32};
use array::ArrayFrom;
use scalar::{boolean, int, uint, float, double};
use uniform::Uniform;

macro_rules! implement_vec {
    ($vec:ident => [$type:ty ; $size:tt] : $align:ty) => {
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

        unsafe impl Uniform for $vec {
            type Align = $align;
            type Std140 = $vec;
            fn std140(&self) -> $vec {
                *self
            }
        }
    }
}


/// Vector of 2 `boolean` values.
/// `foo: bvec2` is equivalent to glsl's `bvec2 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::{boolean, bvec2};
/// # fn main() {
/// let x: bvec2 = [boolean::from(true); 2].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct bvec2([boolean; 2]);
implement_vec!(bvec2 => [boolean; 2] : Align8);

/// Vector of 3 `boolean` values.
/// `foo: bvec3` is equivalent to glsl's `bvec3 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::{boolean, bvec3};
/// # fn main() {
/// let x: bvec3 = [boolean::from(true); 3].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct bvec3([boolean; 3]);
implement_vec!(bvec3 => [boolean; 3] : Align16);

/// Vector of 4 `boolean` values.
/// `foo: bvec4` is equivalent to glsl's `bvec4 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::{boolean, bvec4};
/// # fn main() {
/// let x: bvec4 = [boolean::from(true); 4].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct bvec4([boolean; 4]);
implement_vec!(bvec4 => [boolean; 4] : Align16);


/// Vector of 2 `int` values.
/// `foo: ivec2` is equivalent to glsl's `ivec2 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::ivec2;
/// # fn main() {
/// let x: ivec2 = [1i32; 2].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ivec2([int; 2]);
implement_vec!(ivec2 => [int; 2] : Align8);

/// Vector of 3 `int` values.
/// `foo: ivec3` is equivalent to glsl's `ivec3 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::ivec3;
/// # fn main() {
/// let x: ivec3 = [1i32; 3].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ivec3([int; 3]);
implement_vec!(ivec3 => [int; 3] : Align16);

/// Vector of 4 `int` values.
/// `foo: ivec4` is equivalent to glsl's `ivec4 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::ivec4;
/// # fn main() {
/// let x: ivec4 = [1i32; 4].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ivec4([int; 4]);
implement_vec!(ivec4 => [int; 4] : Align16);


/// Vector of 2 `uint` values.
/// `foo: uvec2` is equivalent to glsl's `uvec2 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::uvec2;
/// # fn main() {
/// let x: uvec2 = [1u32; 2].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct uvec2([uint; 2]);
implement_vec!(uvec2 => [uint; 2] : Align8);

/// Vector of 3 `uint` values.
/// `foo: uvec3` is equivalent to glsl's `uvec3 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::uvec3;
/// # fn main() {
/// let x: uvec3 = [1u32; 3].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct uvec3([uint; 3]);
implement_vec!(uvec3 => [uint; 3] : Align16);

/// Vector of 4 `uint` values.
/// `foo: uvec4` is equivalent to glsl's `uvec4 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::uvec4;
/// # fn main() {
/// let x: uvec4 = [1u32; 4].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct uvec4([uint; 4]);
implement_vec!(uvec4 => [uint; 4] : Align16);


/// Vector of 2 `float` values.
/// `foo: vec2` is equivalent to glsl's `vec2 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::vec2;
/// # fn main() {
/// let x: vec2 = [1f32; 2].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct vec2([float; 2]);
implement_vec!(vec2 => [float; 2] : Align8);

/// Vector of 3 `float` values.
/// `foo: vec3` is equivalent to glsl's `vec3 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::vec3;
/// # fn main() {
/// let x: vec3 = [1f32; 3].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct vec3([float; 3]);
implement_vec!(vec3 => [float; 3] : Align16);


/// Vector of 4 `float` values.
/// `foo: vec4` is equivalent to glsl's `vec4 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::vec4;
/// # fn main() {
/// let x: vec4 = [1f32; 4].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct vec4([float; 4]);
implement_vec!(vec4 => [float; 4] : Align16);


/// Vector of 2 `double` value.
/// `foo: dvec2` is equivalent to glsl's `dvec2 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::dvec2;
/// # fn main() {
/// let x: dvec2 = [1f32; 2].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct dvec2([double; 2]);
implement_vec!(dvec2 => [double; 2] : Align16);

/// Vector of 3 `double` value.
/// `foo: dvec3` is equivalent to glsl's `dvec3 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::dvec3;
/// # fn main() {
/// let x: dvec3 = [1f32; 3].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct dvec3([double; 3]);
implement_vec!(dvec3 => [double; 3] : Align32);


/// Vector of 4 `double` value.
/// `foo: dvec4` is equivalent to glsl's `dvec4 foo;`
///
/// # Examples
/// 
/// ```rust
/// # #[macro_use] extern crate glsl_layout;
/// # use glsl_layout::dvec4;
/// # fn main() {
/// let x: dvec4 = [1f32; 4].into();
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct dvec4([double; 4]);
implement_vec!(dvec4 => [double; 4] : Align32);
