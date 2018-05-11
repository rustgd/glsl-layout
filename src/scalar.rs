
use align::{Align4, Align8};
use uniform::Uniform;

/// Boolean value.
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct boolean(u32);

impl boolean {
    /// Create `boolean` from `bool`.
    pub fn new(value: bool) -> Self {
        value.into()
    }
}

impl From<bool> for boolean {
    fn from(value: bool) -> Self {
        boolean(value as u32)
    }
}

impl From<boolean> for bool {
    fn from(value: boolean) -> Self {
        if value.0 == 0 {
            false
        } else {
            true
        }
    }
}

unsafe impl Uniform for boolean {
    type Align = Align4;
    type Std140 = boolean;
    fn std140(&self) -> boolean {
        *self
    }
}

/// Signed integer value.
pub type int = i32;

unsafe impl Uniform for int {
    type Align = Align4;
    type Std140 = int;
    fn std140(&self) -> int {
        *self
    }
}

/// Unsigned integer value.
pub type uint = u32;

unsafe impl Uniform for uint {
    type Align = Align4;
    type Std140 = uint;
    fn std140(&self) -> uint {
        *self
    }
}

/// floating-point value.
pub type float = f32;

unsafe impl Uniform for float {
    type Align = Align4;
    type Std140 = float;
    fn std140(&self) -> float {
        *self
    }
}

/// Double-precision floating-point value.
pub type double = f64;

unsafe impl Uniform for double {
    type Align = Align8;
    type Std140 = double;
    fn std140(&self) -> double {
        *self
    }
}
