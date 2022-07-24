use crate::mat::{dmat2, dmat3, dmat4, mat2, mat3, mat4};
use crate::scalar::{double, float, int, uint};
use crate::vec::{dvec2, dvec3, dvec4, ivec2, ivec3, ivec4, uvec2, uvec3, uvec4, vec2, vec3, vec4};
use glam::{
    DMat2, DMat3, DMat4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3,
    UVec4, Vec2, Vec3, Vec4,
};

macro_rules! impl_vec_from_glam {
    ($vec:ident : $glam:ident => [$type:ty; $size:tt]) => {
        impl From<$glam> for $vec {
            fn from(value: $glam) -> Self {
                let array: [$type; $size] = value.into();
                array.into()
            }
        }
    };
}

macro_rules! impl_mat_from_glam {
    ($mat:ident : $glam:ident => [$type:ty; $size:tt]) => {
        impl From<$glam> for $mat {
            fn from(value: $glam) -> Self {
                let array: [[$type; $size]; $size] = value.to_cols_array_2d();
                array.into()
            }
        }
    };
}

impl_vec_from_glam!(ivec2 : IVec2 => [int;    2]);
impl_vec_from_glam!(ivec3 : IVec3 => [int;    3]);
impl_vec_from_glam!(ivec4 : IVec4 => [int;    4]);
impl_vec_from_glam!(uvec2 : UVec2 => [uint;   2]);
impl_vec_from_glam!(uvec3 : UVec3 => [uint;   3]);
impl_vec_from_glam!(uvec4 : UVec4 => [uint;   4]);
impl_vec_from_glam!( vec2 : Vec2 => [float;   2]);
impl_vec_from_glam!( vec3 : Vec3 => [float;   3]);
impl_vec_from_glam!( vec4 : Vec4 => [float;   4]);
impl_vec_from_glam!(dvec2 : DVec2 => [double; 2]);
impl_vec_from_glam!(dvec3 : DVec3 => [double; 3]);
impl_vec_from_glam!(dvec4 : DVec4 => [double; 4]);

impl_mat_from_glam!( mat2 : Mat2 => [float;   2]);
impl_mat_from_glam!( mat3 : Mat3 => [float;   3]);
impl_mat_from_glam!( mat4 : Mat4 => [float;   4]);
impl_mat_from_glam!(dmat2 : DMat2 => [double; 2]);
impl_mat_from_glam!(dmat3 : DMat3 => [double; 3]);
impl_mat_from_glam!(dmat4 : DMat4 => [double; 4]);

#[test]
fn test_cgmath() {
    use crate::mat::mat2;
    use crate::vec::dvec3;

    let _: dvec3 = DVec3::new(1.0, 2.0, 3.0).into();
    let _: mat2 = Mat2::IDENTITY.into();
}
