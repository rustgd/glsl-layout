
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct boolean(u32);

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

pub type int = i32;

pub type uint = u32;

pub type float = f32;

pub type double = f64;
