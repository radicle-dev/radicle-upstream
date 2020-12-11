//! The state types and traits that we can use for [`super::Request`]'s `S` parameter.

use std::{
    collections::HashMap,
    fmt,
    ops::{Add, AddAssign, Deref},
};

use serde::{Deserialize, Serialize};

use librad::peer::PeerId;

use super::sealed;

impl sealed::Sealed for Created {}
impl sealed::Sealed for Requested {}
impl sealed::Sealed for Found {}
impl sealed::Sealed for Cloning {}
impl sealed::Sealed for Cancelled {}

// State Types

/// An enumeration of the different states a `Request` can be in. This is useful if we want to
/// convey the state information without any of the other state data.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestState {
    /// The initial state where the `Request` has been created.
    Created,

    /// The state where the `Request` has been requested.
    Requested,

    /// The state where the `Request` has found at least one peer.
    Found,

    /// The state where the `Request` has is cloning from a peer.
    Cloning,

    /// The state where the `Request` has successfully cloned from a peer.
    Cloned,

    /// The state where the `Request` has been cancelled.
    Cancelled,

    /// The state where the `Request` has timed out.
    TimedOut,
}

impl fmt::Display for RequestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The initial state for a `Request`. It has simply been created.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Created {}

/// The state signifying that the `Request` has been kicked-off.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requested {
    /// A set of found peers and the lifecycle of clone attempts made on those peers.
    pub(super) peers: HashMap<PeerId, Status>,
}

/// `Status` represents the lifecycle of some action on data. The data could be available to take
/// the action on, the action could be in progress, or it could have failed.
///
/// Note that the related data isn't in the `Status` variants. The status is free to be associated
/// with any external data, e.g. using it as a value in a `HashMap`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// The status of the related data is: ready to go!
    Available,
    /// The status of the related data is: in progress!
    InProgress,
    /// The status of the related data is: failed :(
    Failed,
}

impl Status {
    /// Joining two `Status` favours `Failed` over any other `Status`, then `InProgress`, and
    /// finally `Available`.
    ///
    /// This translates to the fact that if something has `Failed` then that's it, there's no going
    /// back.
    /// If it's `InProgress`, it doesn't matter that the other `Status` is saying its `Available`,
    /// because you know what? It's actually in progress.
    /// And finally, the last case is that both `Status`es agree the `Status` is `Available`.
    #[must_use]
    pub const fn join(self, other: Self) -> Self {
        match (self, other) {
            (Self::Failed, _) | (_, Self::Failed) => Self::Failed,
            (Self::InProgress, _) | (_, Self::InProgress) => Self::InProgress,
            (Self::Available, Self::Available) => Self::Available,
        }
    }
}

/// The `Found` state means that we have found at least one peer and can possibly find more.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Found {
    /// A set of found peers and the lifecycle of clone attempts made on those peers.
    pub(crate) peers: HashMap<PeerId, Status>,
}

impl Deref for Found {
    type Target = HashMap<PeerId, Status>;

    fn deref(&self) -> &Self::Target {
        &self.peers
    }
}

/// The `Cloning` state means that we have found at least one peer and we are attempting a clone on
/// one of them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloning {
    /// A set of found peers and the lifecycle of clone attempts made on those peers.
    pub(crate) peers: HashMap<PeerId, Status>,
}

impl Deref for Cloning {
    type Target = HashMap<PeerId, Status>;

    fn deref(&self) -> &Self::Target {
        &self.peers
    }
}

/// The `Cloned` state means that we have successfully cloned the desired identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloned {
    /// The identity that we were attempting to find and the peer that we found it from.
    pub(crate) remote_peer: PeerId,
}
/// One of the terminal states for a `Request` where the task has been cancelled.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancelled {}

/// One of the terminal states for a `Request` where the task made too many attempts.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimedOut {
    /// The `Request` made too many query attempts.
    Query,
    /// The `Request` made too many clone attempts.
    Clone,
}

impl fmt::Display for TimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Query => write!(f, "query"),
            Self::Clone => write!(f, "clone"),
        }
    }
}

/// `Queries` is a wrapper around `usize` so that we can differentiate it from [`Clones`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Queries {
    /// The max number of queries allowed per request.
    Max(usize),
    /// The max number is infinite, and so we allow the request to never time out.
    Infinite,
}

impl Queries {
    /// Create a new `Queries` wrapping around `n`.
    #[must_use]
    pub const fn new(n: usize) -> Self {
        Self::Max(n)
    }
}

impl From<Queries> for Option<usize> {
    fn from(other: Queries) -> Self {
        match other {
            Queries::Max(i) => Some(i),
            Queries::Infinite => None,
        }
    }
}

impl Add<usize> for Queries {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        match self {
            Self::Max(i) => Self::Max(i + other),
            Self::Infinite => Self::Infinite,
        }
    }
}

impl AddAssign<usize> for Queries {
    fn add_assign(&mut self, other: usize) {
        match self {
            Self::Max(i) => *i += other,
            Self::Infinite => {},
        }
    }
}

/// `Clones` is a wrapper around `usize` so that we can differentiate it from [`Queries`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Clones {
    /// The max number of clones allowed per request.
    Max(usize),
    /// The max number is infinite, and so we allow the request to never time out.
    Infinite,
}

impl Clones {
    /// Create a new `Clones` wrapping around `n`.
    #[must_use]
    pub const fn new(n: usize) -> Self {
        Self::Max(n)
    }
}

impl From<Clones> for Option<usize> {
    fn from(other: Clones) -> Self {
        match other {
            Clones::Max(i) => Some(i),
            Clones::Infinite => None,
        }
    }
}

impl Add<usize> for Clones {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        match self {
            Self::Max(i) => Self::Max(i + other),
            Self::Infinite => Self::Infinite,
        }
    }
}

impl AddAssign<usize> for Clones {
    fn add_assign(&mut self, other: usize) {
        match self {
            Self::Max(i) => *i += other,
            Self::Infinite => {},
        }
    }
}

/// The number of different attempts a `Request` has made during its lifetime.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attempts {
    /// The number of query attempts we have made.
    pub(super) queries: Queries,
    /// The number of clone attempts we have made.
    pub(super) clones: Clones,
}

impl Attempts {
    /// Get a new `Attempts` where the number of queires and clones is initially `0`.
    #[must_use]
    pub const fn new() -> Self {
        Attempts {
            queries: Queries::Max(0),
            clones: Clones::Max(0),
        }
    }

    /// Construct an `Attempts` where the number of queries and clones is `Infinite`.
    #[must_use]
    pub const fn infinite() -> Self {
        Attempts {
            queries: Queries::Infinite,
            clones: Clones::Infinite,
        }
    }
}

impl Default for Attempts {
    fn default() -> Self {
        Attempts::new()
    }
}

impl sealed::Sealed for Attempts {}

// State Traits

/// If a state type implements this trait then it means that the type is allowed to increment the
/// query attempts in a `Request`.
///
/// The trait is sealed internally, so we do not expect end-users to implement it.
pub trait QueryAttempt: sealed::Sealed {}
impl QueryAttempt for Requested {}

/// If a state type implements this trait then it means that the type holds a `HashMap` of peers and
/// their status of cloning.
///
/// The trait is sealed internally, so we do not expect end-users to implement it.
pub trait HasPeers: sealed::Sealed
where
    Self: Sized + Deref<Target = HashMap<PeerId, Status>>,
{
    /// Give back the underlying `HashMap` of peers that is contained in `Self`.
    fn peers(&mut self) -> &mut HashMap<PeerId, Status>;

    /// Returns `false` if the `peers` are empty or if any of them are `Status::Available`
    /// or `Status::InProgress`.
    ///
    /// Otherwise, if all are in the `Status::Failed` state, then we return `true`.
    fn all_failed(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.iter().all(|(_, status)| *status == Status::Failed)
    }
}

impl HasPeers for Found {
    fn peers(&mut self) -> &mut HashMap<PeerId, Status> {
        &mut self.peers
    }
}

impl HasPeers for Cloning {
    fn peers(&mut self) -> &mut HashMap<PeerId, Status> {
        &mut self.peers
    }
}

/// If a state type implements this trait it means that there is a valid transition from that state
/// to the `Cancelled` state.
///
/// The trait is sealed internally, so we do not expect end-users to implement it.
pub trait Cancel: sealed::Sealed
where
    Self: Sized,
{
    /// Transition the state into `Cancelled`. This ignores whatever state we were in and defaults
    /// by returning the `Cancelled` state.
    fn cancel(self) -> Cancelled {
        Cancelled {}
    }
}

impl Cancel for Created {}
impl Cancel for Requested {}
impl Cancel for Found {}
impl Cancel for Cloning {}
impl Cancel for Cancelled {}

/// If a state type implements this trait it means that there is a valid transition from that state
/// to the `TimedOut` state.
///
/// The trait is sealed internally, so we do not expect end-users to implement it.
pub trait TimeOut: sealed::Sealed
where
    Self: Sized,
{
    /// Transition the state into `TimedOut`. This ignores whatever state we were in and defaults
    /// by returning the `TimedOut` state by returning the `kind` of timeout.
    fn time_out(self, kind: TimedOut) -> TimedOut {
        kind
    }
}

impl TimeOut for Requested {}
impl TimeOut for Found {}
impl TimeOut for Cloning {}
