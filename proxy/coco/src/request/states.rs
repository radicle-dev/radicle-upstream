use std::{collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Serialize};

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
    fn time_out(self, kind: TimedOut) -> TimedOut {
        kind
    }
}

impl TimeOut for IsRequested {}
impl TimeOut for Found {}
impl TimeOut for Cloning {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Available,
    InProgress,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Found {
    pub(crate) peers: HashMap<PeerId, Status>,
}

// TODO(finto): Should Cloning know which PeerId it's cloning?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloning {
    pub(crate) peers: HashMap<PeerId, Status>,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloned {
    pub(crate) repo: RadUrn,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Created;
pub type IsCreated = PhantomData<Created>;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canceled;
pub type IsCanceled = PhantomData<Canceled>;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimedOut {
    Query,
    Clone,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requested;
pub type IsRequested = PhantomData<Requested>;
