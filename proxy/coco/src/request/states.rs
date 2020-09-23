//! The state types and traits that we can use for [`super::Request`]'s `S` parameter.

use std::{
    collections::HashMap,
    fmt,
    marker::PhantomData,
    ops::{Add, AddAssign, Deref},
};

use serde::{Deserialize, Serialize};

use librad::{peer::PeerId, uri::RadUrn};

use super::sealed;

impl sealed::Sealed for IsCreated {}
impl sealed::Sealed for IsRequested {}
impl sealed::Sealed for Found {}
impl sealed::Sealed for Cloning {}
impl sealed::Sealed for IsCanceled {}

// State Types

/// An enumeration of the different states a `Request` can be in. This is useful if we want to
/// convey the state information without any of the other state data.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestKind {
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
    Canceled,

    /// The state where the `Request` has timed out.
    TimedOut,
}

impl fmt::Display for RequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The initial state for a `Request`. It has simply been created.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Created;
/// The initial state for a `Request`. It has simply been created.
pub type IsCreated = PhantomData<Created>;

/// The state signifying that the `Request` has been kicked-off.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requested;
/// The state signifying that the `Request` has been kicked-off.
pub type IsRequested = PhantomData<Requested>;

/// `Status` represents the lifecycle of a clone attempt, when paired with a `PeerId`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// The `PeerId` is available for cloning, and an attempt has not been made yet.
    Available,
    /// An attempt to clone from the `PeerId` is currently being made.
    InProgress,
    /// The attempt to clone from the `PeerId` has failed.
    Failed,
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
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloned {
    /// The identity that we were attempting to find.
    pub(crate) repo: RadUrn,
}
/// One of the terminal states for a `Request` where the task has been cancelled.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canceled;
/// One of the terminal states for a `Request` where the task has been cancelled.
pub type IsCanceled = PhantomData<Canceled>;

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
pub struct Queries(usize);

impl Queries {
    /// Create a new `Queries` wrapping around `n`.
    #[must_use]
    pub const fn new(n: usize) -> Self {
        Self(n)
    }
}

impl From<Queries> for usize {
    fn from(other: Queries) -> Self {
        other.0
    }
}

impl Add<usize> for Queries {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl AddAssign<usize> for Queries {
    fn add_assign(&mut self, other: usize) {
        *self = Self(self.0 + other)
    }
}

/// `Clones` is a wrapper around `usize` so that we can differentiate it from [`Queries`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Clones(usize);

impl Clones {
    /// Create a new `Clones` wrapping around `n`.
    #[must_use]
    pub const fn new(n: usize) -> Self {
        Self(n)
    }
}

impl From<Clones> for usize {
    fn from(other: Clones) -> Self {
        other.0
    }
}

impl Add<usize> for Clones {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl AddAssign<usize> for Clones {
    fn add_assign(&mut self, other: usize) {
        *self = Self(self.0 + other)
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
            queries: Queries(0),
            clones: Clones(0),
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
impl QueryAttempt for IsRequested {}

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
/// to the `IsCanceled` state.
///
/// The trait is sealed internally, so we do not expect end-users to implement it.
pub trait Cancel: sealed::Sealed
where
    Self: Sized,
{
    /// Transition the state into `IsCanceled`. This ignores whatever state we were in and defaults
    /// by returning the `IsCanceled` state.
    fn cancel(self) -> IsCanceled {
        PhantomData
    }
}

impl Cancel for IsCreated {}
impl Cancel for IsRequested {}
impl Cancel for Found {}
impl Cancel for Cloning {}
impl Cancel for IsCanceled {}

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

impl TimeOut for IsRequested {}
impl TimeOut for Found {}
impl TimeOut for Cloning {}
