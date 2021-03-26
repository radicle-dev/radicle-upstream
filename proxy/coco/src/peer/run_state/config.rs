//! Configuration types for configuring the `RunState`.

use std::time::Duration;

/// Default time to wait between announcement subroutine runs.
pub(super) const DEFAULT_ANNOUNCE_INTERVAL: Duration = std::time::Duration::from_secs(1);

pub(super) const DEFAULT_STATS_INTERVAL: Duration = Duration::from_millis(1000);

pub(super) const DEFAULT_SYNC_INTERVAL: Duration = std::time::Duration::from_secs(30);

/// Default period at which we query the waiting room.
pub(super) const DEFAULT_WAITING_ROOM_INTERVAL: Duration = Duration::from_millis(500);

/// Default period to consider until a query has timed out.
pub(crate) const DEFAULT_WAITING_ROOM_TIMEOUT: Duration = Duration::from_secs(10);

/// Set of knobs to change the behaviour of the `RunState`.
#[derive(Clone, Default)]
pub struct Config {
    /// Set of knobs to alter announce behaviour.
    pub announce: Announce,
    /// Set of knobs to alter stats polling.
    pub stats: Stats,
    /// Set of knobs to alter sync behaviour.
    pub sync: Sync,
    /// Set of knobs to alter [`WaitingRoom`] behaviour.
    pub waiting_room: WaitingRoom,
}

/// Set of knobs to alter announce behaviour.
#[derive(Clone, Debug)]
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

/// Set of knobs to alter stats polling.
#[derive(Clone, Debug)]
pub struct Stats {
    /// Determines how often the stats subroutine should be run.
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
#[derive(Clone, Debug)]
pub struct Sync {
    /// Duration to issue periodic syncs.
    pub interval: Duration,
}

impl Default for Sync {
    fn default() -> Self {
        Self {
            interval: DEFAULT_SYNC_INTERVAL,
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
