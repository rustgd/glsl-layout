
use align::{Aligned, Align4, Align8};

pub trait Scalar: Aligned {
    const ZERO: Self;
}

/// Boolean value.
#[derive(Clone, Copy, Debug)]
pub struct boolean(u32);

impl<T> From<T> for boolean
where
    T: Into<bool>,
{
    fn from(value: T) -> Self {
        boolean(value.into() as u32)
    }
}

impl Aligned for boolean {
    const SIZE: usize = 4;
    type Align = Align4;
}

impl Scalar for boolean {
    const ZERO: Self = boolean(0);
}

/// Integer value.
pub type int = i32;

impl Aligned for int {
    const SIZE: usize = 4;
    type Align = Align4;
}

impl Scalar for int {
    const ZERO: Self = 0;
}

/// Unsigned integer value.
pub type uint = u32;

impl Aligned for uint {
    const SIZE: usize = 4;
    type Align = Align4;
}

impl Scalar for uint {
    const ZERO: Self = 0;
}

/// Floating point value.
pub type float = f32;

impl Aligned for float {
    const SIZE: usize = 4;
    type Align = Align4;
}

impl Scalar for float {
    const ZERO: Self = 0.0;
}

/// Double-precision floating point value.
pub type double = f64;

impl Aligned for double {
    const SIZE: usize = 8;
    type Align = Align8;
}

impl Scalar for double {
    const ZERO: Self = 0.0;
}
