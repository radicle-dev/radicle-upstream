//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use crate::identity;

/// Container for all local state.
pub struct Session {
    /// The currently used [`identity::Identity`].
    pub identity: Option<identity::Identity>,
}
