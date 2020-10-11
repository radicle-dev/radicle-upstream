//! Traits for conversions between types.

/// Used to do conversions where the input maybe maps.
pub trait MaybeFrom<T>: Sized {
    /// Performs the conversion.
    fn maybe_from(_: T) -> Option<Self>;
}
