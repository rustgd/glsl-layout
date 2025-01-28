use crate::mat::{dmat2, dmat3, dmat4, mat2, mat3, mat4};
use crate::scalar::{double, float, int, uint};
use crate::uniform::Uniform;
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

        impl Uniform for $glam {
            type Align = <$vec as Uniform>::Align;
            type Std140 = <$vec as Uniform>::Std140;

            fn std140(&self) -> Self::Std140 {
                Self::Std140::from(*self)
            }
        }
    };
}

macro_rules! impl_mat_from_glam {
    ($mat:ident : $glam:ident => [$type:ty; $size:tt]) => {
        impl From<$glam> for $mat {
            fn from(value: $glam) -> Self {
                $mat::from(value.to_cols_array_2d())
            }
        }

        impl Uniform for $glam {
            type Align = <$mat as Uniform>::Align;
            type Std140 = <$mat as Uniform>::Std140;

            fn std140(&self) -> Self::Std140 {
                Self::Std140::from(self.to_cols_array_2d())
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
pub fn test_glam() {
    use crate::uniform::Std140;

    let v3: vec3 = [1.0, 2.0, 3.0].into();
    let gv3_to_v3: vec3 = Vec3::new(1.0, 2.0, 3.0).into();
    let gv3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v3.std140(), gv3.std140());
    assert_eq!(gv3.std140(), gv3_to_v3.std140());

    let m3: mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].into();
    let gm3_to_m3: mat3 = Mat3::IDENTITY.into();
    let gm3 = Mat3::IDENTITY;
    assert_eq!(m3.std140(), gm3.std140());
    assert_eq!(gm3.std140(), gm3_to_m3.std140());
}
