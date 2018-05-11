
/// Structure to transform data from rust's structure to the raw data ready to upload to UBO.
/// Users should prepfer to use `derive(Uniform)` instead of implementing this manually.
pub trait Uniform {
    /// ZST that enforces alignment required for this type.
    type Align;

    /// Type that contain same data with memory layout matching glsl's `layout(std140)`.
    type Std140;

    /// Get aligned data from structure.
    fn std140(&self) -> Self::Std140;
}
