use align::{Align4, Align8};
use uniform::{Std140, Uniform};

macro_rules! impl_scalar {
    ($type:ty : $align:tt) => {
        unsafe impl Std140 for $type {}

        impl Uniform for $type {
            type Align = $align;
            type Std140 = $type;

            fn std140(&self) -> $type {
                *self
            }
        }
    }
}

/// Boolean value.
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct boolean(u32);
impl_scalar!(boolean : Align4);

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

/// Signed integer value.
pub type int = i32;
impl_scalar!(int : Align4);

/// Unsigned integer value.
pub type uint = u32;
impl_scalar!(uint : Align4);

/// floating-point value.
pub type float = f32;
impl_scalar!(float : Align4);

/// Double-precision floating-point value.
pub type double = f64;
impl_scalar!(double : Align8);
