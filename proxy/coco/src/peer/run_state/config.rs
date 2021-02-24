//! Configuration types for configuring the `RunState`.

use std::time::Duration;

/// Default time to wait between announcement subroutine runs.
pub(super) const DEFAULT_ANNOUNCE_INTERVAL: Duration = std::time::Duration::from_secs(1);

pub(super) const DEFAULT_STATS_INTERVAL: Duration = Duration::from_millis(100);

/// Default number of peers a full sync is attempting with up on startup.
/// TODO(xla): Revise number.
pub(super) const DEFAULT_SYNC_MAX_PEERS: usize = 5;

/// Default Duration until the local peer goes online regardless if and how many syncs have
/// succeeded.
// TODO(xla): Review duration.
pub(super) const DEFAULT_SYNC_PERIOD: Duration = Duration::from_secs(5);

/// Default period at which we query the waiting room.
pub(super) const DEFAULT_WAITING_ROOM_INTERVAL: Duration = Duration::from_millis(500);

/// Default period to consider until a query has timed out.
pub(crate) const DEFAULT_WAITING_ROOM_TIMEOUT: Duration = Duration::from_secs(10);

/// Set of knobs to change the behaviour of the `RunState`.
#[derive(Default)]
pub struct Config {
    /// Set of knobs to alter announce behaviour.
    pub announce: Announce,
    pub stats: Stats,
    /// Set of knobs to alter sync behaviour.
    pub sync: Sync,
    /// Set of knobs to alter [`WaitingRoom`] behaviour.
    pub waiting_room: WaitingRoom,
}

/// Set of knobs to alter announce behaviour.
pub struct Announce {
    /// Determines how often the announcement subroutine should be run.
    pub interval: Duration,
}

impl Default for Announce {
    fn default() -> Self {
        Self {
            interval: DEFAULT_ANNOUNCE_INTERVAL,
        }
    }
}

pub struct Stats {
    pub interval: Duration,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            interval: DEFAULT_STATS_INTERVAL,
        }
    }
}

/// Set of knobs to alter sync behaviour.
pub struct Sync {
    /// Number of peers that a full sync is attempted with upon startup.
    pub max_peers: usize,
    /// Enables the syncing stage when coming online.
    pub on_startup: bool,
    /// Duration until the local peer goes online regardless if and how many syncs have succeeded.
    pub period: Duration,
}

impl Default for Sync {
    fn default() -> Self {
        Self {
            max_peers: DEFAULT_SYNC_MAX_PEERS,
            on_startup: false,
            period: DEFAULT_SYNC_PERIOD,
        }
    }
}

/// Set of knobs to alter the [`crate::request::waiting_room::WaitingRoom`] behvaviour.
#[derive(Clone, Debug)]
pub struct WaitingRoom {
    /// Interval at which to query the [`crate::request::waiting_room::WaitingRoom`] for ready
    /// requests.
    pub interval: Duration,
}

impl Default for WaitingRoom {
    fn default() -> Self {
        Self {
            interval: DEFAULT_WAITING_ROOM_INTERVAL,
        }
    }
}
