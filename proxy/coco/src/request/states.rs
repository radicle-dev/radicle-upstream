use std::{collections::HashMap, marker::PhantomData};

use librad::peer::PeerId;
use librad::uri::RadUrn;

use super::sealed;

impl sealed::Sealed for IsCreated {}
impl sealed::Sealed for IsRequested {}
impl sealed::Sealed for Found {}
impl sealed::Sealed for Cloning {}
impl sealed::Sealed for IsCanceled {}

pub trait HasPeers: sealed::Sealed
where
    Self: Sized,
{
    fn peers(&mut self) -> &mut HashMap<PeerId, Status>;
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

pub trait Cancel: sealed::Sealed
where
    Self: Sized,
{
    fn cancel(self) -> IsCanceled {
        PhantomData
    }
}

impl Cancel for IsCreated {}
impl Cancel for IsRequested {}
impl Cancel for Found {}
impl Cancel for Cloning {}
impl Cancel for IsCanceled {}

pub trait TimeOut: sealed::Sealed
where
    Self: Sized,
{
    fn time_out(self, kind: Kind) -> TimedOut {
        TimedOut { kind }
    }
}

impl TimeOut for IsRequested {}
impl TimeOut for Found {}
impl TimeOut for Cloning {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Available,
    InProgress,
    Failed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Found {
    pub(crate) peers: HashMap<PeerId, Status>,
}

// TODO(finto): Should Cloning know which PeerId it's cloning?
#[derive(Clone, Debug, PartialEq)]
pub struct Cloning {
    pub(crate) peers: HashMap<PeerId, Status>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Cloned {
    pub(crate) repo: RadUrn,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Created;
pub type IsCreated = PhantomData<Created>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Canceled;
pub type IsCanceled = PhantomData<Canceled>;

// TODO(finto): Better naming to please the people who will inevitably give out about it.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Kind {
    Query,
    Clone,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TimedOut {
    pub(crate) kind: Kind,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Requested;
pub type IsRequested = PhantomData<Requested>;
