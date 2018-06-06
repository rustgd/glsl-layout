
/// Special marker trait implemented only for `std140` types.
pub unsafe trait Std140: Sized + Uniform<Std140 = Self> {
    fn as_raw(&self) -> &[u8] {
        use std::{slice::from_raw_parts, mem::size_of};
        unsafe {
            from_raw_parts(self as *const Self as *const u8, size_of::<Self>())
        }
    }
}

/// Structure to transform data from rust's structure to the raw data ready to upload to UBO.
/// Users should prepfer to use `derive(Uniform)` instead of implementing this manually.
pub trait Uniform: Copy {
    /// ZST that enforces alignment required for this type.
    type Align: Copy + Default;

    /// Type that contain same data with memory layout matching glsl's `layout(std140)`.
    type Std140: Std140;

    /// Get aligned data from structure.
    fn std140(&self) -> Self::Std140;
}
