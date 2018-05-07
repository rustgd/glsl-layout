
use std::fmt::Debug;

pub trait Align: Copy + Debug {
    const SELF: Self;
}

// pub trait OverAlign<A>: Align {} 

pub trait Aligned: Copy + Sized + Debug {
    const SIZE: usize;
    type Align: Align;
}

macro_rules! define_align {
    ($name:ident: $value:tt) => {
        #[repr(align($value))]
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl Align for $name {
            const SELF: Self = $name;
        }
    }
}

define_align!(Align4:4);
define_align!(Align8:8);
define_align!(Align16:16);
define_align!(Align32:32);

// impl OverAlign<Align4> for Align4 {}
// impl OverAlign<Align4> for Align8 {}
// impl OverAlign<Align4> for Align16 {}
// impl OverAlign<Align4> for Align32 {}

// impl OverAlign<Align8> for Align8 {}
// impl OverAlign<Align8> for Align16 {}
// impl OverAlign<Align8> for Align32 {}

// impl OverAlign<Align16> for Align16 {}
// impl OverAlign<Align16> for Align32 {}

// impl OverAlign<Align32> for Align32 {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WithAlign<T, A>(pub T, pub A);

impl<T, A> From<T> for WithAlign<T, A>
where
    T: Copy,
    A: Align,
{
    fn from(value: T) -> Self {
        WithAlign(value.clone(), A::SELF)
    }
}

impl<T, A> AsRef<T> for WithAlign<T, A> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, A> AsMut<T> for WithAlign<T, A> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}