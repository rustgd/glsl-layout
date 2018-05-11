
/// Structure to transform data from rust's structure to the raw data ready to upload to UBO.
/// Users should prepfer to use `derive(Uniform)` instead of implementing this manually.
pub unsafe trait Uniform: Copy {
    /// ZST that enforces alignment required for this type.
    type Align;

    /// Type that contain same data with memory layout matching glsl's `layout(std140)`.
    type Std140;

    /// Get aligned data from structure.
    fn std140(&self) -> Self::Std140;

    /// Convert this type's `Std140` to bytes.
    fn raw(std140: &Self::Std140) -> &[u8] {
        unsafe { ::as_ref_u8(std140) }
    }
}

