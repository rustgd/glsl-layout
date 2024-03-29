use crate::mat::{dmat2, dmat3, dmat4, imat2, imat3, imat4, mat2, mat3, mat4, umat2, umat3, umat4};
use crate::scalar::{double, float, int, uint};
use crate::uniform::Uniform;
use crate::vec::{dvec2, dvec3, dvec4, ivec2, ivec3, ivec4, uvec2, uvec3, uvec4, vec2, vec3, vec4};
use cgmath::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};

macro_rules! impl_vec_from_cgmath {
    ($vec:ident : $cgmath:ident => [$type:ty; $size:tt]) => {
        impl From<$cgmath<$type>> for $vec {
            fn from(value: $cgmath<$type>) -> Self {
                let array: [$type; $size] = value.into();
                array.into()
            }
        }

        impl Uniform for $cgmath<$type> {
            type Align = <$vec as Uniform>::Align;
            type Std140 = $vec;

            fn std140(&self) -> Self::Std140 {
                Self::Std140::from(*self)
            }
        }
    };
}

macro_rules! impl_mat_from_cgmath {
    ($mat:ident : $cgmath:ident => [$type:ty; $size:tt]) => {
        impl From<$cgmath<$type>> for $mat {
            fn from(value: $cgmath<$type>) -> Self {
                let array: [[$type; $size]; $size] = value.into();
                array.into()
            }
        }

        impl Uniform for $cgmath<$type> {
            type Align = <$mat as Uniform>::Align;
            type Std140 = $mat;

            fn std140(&self) -> Self::Std140 {
                let array: [[$type; $size]; $size] = (*self).into();
                Self::Std140::from(array)
            }
        }
    };
}

impl_vec_from_cgmath!(ivec2 : Vector2 => [int;    2]);
impl_vec_from_cgmath!(ivec3 : Vector3 => [int;    3]);
impl_vec_from_cgmath!(ivec4 : Vector4 => [int;    4]);
impl_vec_from_cgmath!(uvec2 : Vector2 => [uint;   2]);
impl_vec_from_cgmath!(uvec3 : Vector3 => [uint;   3]);
impl_vec_from_cgmath!(uvec4 : Vector4 => [uint;   4]);
impl_vec_from_cgmath!( vec2 : Vector2 => [float;  2]);
impl_vec_from_cgmath!( vec3 : Vector3 => [float;  3]);
impl_vec_from_cgmath!( vec4 : Vector4 => [float;  4]);
impl_vec_from_cgmath!(dvec2 : Vector2 => [double; 2]);
impl_vec_from_cgmath!(dvec3 : Vector3 => [double; 3]);
impl_vec_from_cgmath!(dvec4 : Vector4 => [double; 4]);

impl_mat_from_cgmath!(imat2 : Matrix2 => [int;    2]);
impl_mat_from_cgmath!(imat3 : Matrix3 => [int;    3]);
impl_mat_from_cgmath!(imat4 : Matrix4 => [int;    4]);
impl_mat_from_cgmath!(umat2 : Matrix2 => [uint;   2]);
impl_mat_from_cgmath!(umat3 : Matrix3 => [uint;   3]);
impl_mat_from_cgmath!(umat4 : Matrix4 => [uint;   4]);
impl_mat_from_cgmath!( mat2 : Matrix2 => [float;  2]);
impl_mat_from_cgmath!( mat3 : Matrix3 => [float;  3]);
impl_mat_from_cgmath!( mat4 : Matrix4 => [float;  4]);
impl_mat_from_cgmath!(dmat2 : Matrix2 => [double; 2]);
impl_mat_from_cgmath!(dmat3 : Matrix3 => [double; 3]);
impl_mat_from_cgmath!(dmat4 : Matrix4 => [double; 4]);

#[test]
fn test_cgmath() {
    use crate::uniform::Std140;
    use cgmath::SquareMatrix;

    let v3: vec3 = [1.0f32, 2.0, 3.0].into();
    let gv3_to_v3: vec3 = Vector3::new(1.0f32, 2.0, 3.0).into();
    let gv3 = Vector3::new(1.0f32, 2.0, 3.0);
    assert_eq!(v3.std140().as_raw(), gv3.std140().as_raw());
    assert_eq!(gv3.std140().as_raw(), gv3_to_v3.std140().as_raw());

    let m3: mat3 = [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].into();
    let gm3_to_m3: mat3 = Matrix3::<f32>::identity().into();
    let gm3 = Matrix3::<f32>::identity();
    assert_eq!(m3.std140(), gm3.std140());
    assert_eq!(gm3.std140(), gm3_to_m3.std140());
}
