
/// Boolean value.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct boolean(u32);

impl boolean {
    fn new(value: bool) -> Self {
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

/// Unsigned integer value.
pub type uint = u32;

/// floating-point value.
pub type float = f32;

/// Double-precision floating-point value.
pub type double = f64;
