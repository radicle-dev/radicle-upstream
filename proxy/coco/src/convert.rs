//! Traits for conversions between types.

use std::convert::TryFrom;

/// Used to do conversions where the input maybe maps.
pub trait MaybeFrom<T>: Sized {
    /// Performs the conversion.
    fn maybe_from(_: T) -> Option<Self>;
}

impl<T, U: TryFrom<T>> MaybeFrom<T> for U {
    fn maybe_from(t: T) -> Option<Self> {
        Self::try_from(t).ok()
    }
}
