
extern crate cgmath;

// use align::{Align, Aligned, Array};
use array::GArray;
use vec::GVec;

use self::cgmath::{Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4};

macro_rules! from_vec {
    ($vec:ident => [$type:ty; $size:tt]) => {
        impl From<$vec<$type>> for GVec<[$type; $size]> {
            fn from(value: $vec<$type>) -> Self {
                let array: [$type; $size] = value.into();
                array.into()
            }
        }
    };

    ($vec:ident / $size:tt) => {
        from_vec!($vec => [i32; $size]);
        from_vec!($vec => [u32; $size]);
        from_vec!($vec => [f32; $size]);
        from_vec!($vec => [f64; $size]);
    }
}

from_vec!(Vector2 / 2);
from_vec!(Vector3 / 3);
from_vec!(Vector4 / 4);


macro_rules! from_mat {
    ($mat:ident => [$type:ty; $cols:tt * $rows:tt]) => {
        impl From<$mat<$type>> for GArray<[GVec<[$type; $cols]>; $rows]> {
            fn from(value: $mat<$type>) -> Self {
                let array: [[$type; $rows]; $cols] = value.into();
                array.into()
            }
        }
    };

    ($mat:ident / $cols:tt * $rows:tt) => {
        from_mat!($mat => [i32; $cols * $rows]);
        from_mat!($mat => [u32; $cols * $rows]);
        from_mat!($mat => [f32; $cols * $rows]);
        from_mat!($mat => [f64; $cols * $rows]);
    }
}

from_mat!(Matrix2 / 2 * 2);
from_mat!(Matrix3 / 3 * 3);
from_mat!(Matrix4 / 4 * 4);

#[test]
fn test_cgmath() {
    pub use std::mem::{align_of, size_of};
    use self::cgmath::SquareMatrix;

    let _: dvec![3] = Vector3::new(1.0, 2.0, 3.0).into();
    let _: fmat![2 * 2] = Matrix2::from_value(1.0f32).into();
}
