
use std::fmt::Debug;
use align::{Aligned, Align4, Align8, Align16, Align32, WithAlign};
use scalar::{Scalar, double};

pub trait VecStorage: Sized {
    type Storage: Copy + Debug;
}

#[derive(Clone, Copy, Debug)]
pub struct GVec<T: VecStorage>(T::Storage);

impl<T> VecStorage for [T; 2]
where
    T: Scalar<Align=Align4>,
{
    type Storage = WithAlign<[T; 2], Align8>;
}
impl<T> Aligned for GVec<[T; 2]>
where
    T: Scalar<Align=Align4>,
{
    const SIZE: usize = T::SIZE * 2;
    type Align = Align8;
}
impl<T> From<[T; 2]> for GVec<[T; 2]>
where
    T: Scalar<Align=Align4>,
{
    fn from(values: [T; 2]) -> Self {
        GVec(WithAlign(values, Align8))
    }
}

impl<T> VecStorage for [T; 3]
where
    T: Scalar<Align=Align4>,
{
    type Storage = WithAlign<[T; 3], Align16>;
}
impl<T> Aligned for GVec<[T; 3]>
where
    T: Scalar<Align=Align4>,
{
    const SIZE: usize = T::SIZE * 3;
    type Align = Align16;
}
impl<T> From<[T; 3]> for GVec<[T; 3]>
where
    T: Scalar<Align=Align4>,
{
    fn from(values: [T; 3]) -> Self {
        GVec(WithAlign(values, Align16))
    }
}

impl<T> VecStorage for [T; 4]
where
    T: Scalar<Align=Align4>,
{
    type Storage = WithAlign<[T; 4], Align16>;
}
impl<T> Aligned for GVec<[T; 4]>
where
    T: Scalar<Align=Align4>,
{
    const SIZE: usize = T::SIZE * 4;
    type Align = Align16;
}
impl<T> From<[T; 4]> for GVec<[T; 4]>
where
    T: Scalar<Align=Align4>,
{
    fn from(values: [T; 4]) -> Self {
        GVec(WithAlign(values, Align16))
    }
}

impl VecStorage for [double; 2] {
    type Storage = WithAlign<[double; 2], Align16>;
}
impl Aligned for GVec<[double; 2]> {
    const SIZE: usize = double::SIZE * 2;
    type Align = Align16;
}
impl From<[double; 2]> for GVec<[double; 2]> {
    fn from(values: [double; 2]) -> Self {
        GVec(WithAlign(values, Align16))
    }
}

impl VecStorage for [double; 3] {
    type Storage = WithAlign<[double; 3], Align32>;
}
impl Aligned for GVec<[double; 3]> {
    const SIZE: usize = double::SIZE * 3;
    type Align = Align32;
}
impl From<[double; 3]> for GVec<[double; 3]> {
    fn from(values: [double; 3]) -> Self {
        GVec(WithAlign(values, Align32))
    }
}

impl VecStorage for [double; 4] {
    type Storage = WithAlign<[double; 4], Align32>;
}
impl Aligned for GVec<[double; 4]> {
    const SIZE: usize = double::SIZE * 4;
    type Align = Align32;
}
impl From<[double; 4]> for GVec<[double; 4]> {
    fn from(values: [double; 4]) -> Self {
        GVec(WithAlign(values, Align32))
    }
}
