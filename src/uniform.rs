#[cfg(feature = "gfx")]
use gfx_core::memory::Pod;

/// Special marker trait implemented only for `std140` types.
#[cfg(feature = "gfx")]
pub unsafe trait Std140: Sized + Uniform<Std140 = Self> + Pod {}

/// Special marker trait implemented only for `std140` types.
#[cfg(not(feature = "gfx"))]
pub unsafe trait Std140: Sized + Uniform<Std140 = Self> {}

/// Structure to transform data from rust's structure to the raw data ready to upload to UBO.
/// Users should prepfer to use `derive(Uniform)` instead of implementing this manually.
pub trait Uniform: Copy {
    /// ZST that enforces alignment required for this type.
    type Align: Copy;

    /// The value of the `Align`.
    fn align() -> Self::Align;

    /// Type that contain same data with memory layout matching glsl's `layout(std140)`.
    type Std140: Std140;

    /// Get aligned data from structure.
    fn std140(&self) -> Self::Std140;
}
