//! A queue that manages & caches asynchronous tasks

use std::time::{self, SystemTime};

use kv::Codec as _;
use serde::{Deserialize, Serialize};

/// Possible messages a [`Task`] can carry.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Message {
    /// Find a [`Project`] of a given `urn`
    FindProject {
        /// The unique identifier of the desired [`Project`]
        urn: String,
    },
}

/// Possible states a [`Task`] can have. Useful to reason about the lifecycle and
/// whereabouts of a given [`Task`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum State {
    /// [`Task`] has completed successfully
    Confirmed {
        /// Time when [`Task`] was completed
        timestamp: Timestamp,
    },
    /// [`Task`] has failed
    Failed {
        /// Description of the error that occurred
        error: String,
        /// Time when the [`Task`] failed
        timestamp: Timestamp,
    },
    /// [`Task`] has been executed but its ultimate status has not been determined
    Pending {
        /// Time when [`Task`] was executed
        timestamp: Timestamp,
    },
}

/// Wrapper for [`SystemTime`] carrying the time since epoch.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// Seconds since unix epoch.
    secs: u64,
    /// Sub-second nanos part.
    nanos: u32,
}

impl Timestamp {
    /// Creates a new [`Timestamp`] at the current time.
    #[must_use]
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now
            .duration_since(time::UNIX_EPOCH)
            .expect("time should be after unix epoch");

        Self {
            nanos: duration.subsec_nanos(),
            secs: duration.as_secs(),
        }
    }
}

/// An asynchronous, unique task to be enqueued and observed for status updates
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Unique identifier of the task. This handle should be used by
    /// the API consumer to query state changes of a task.
    pub id: String,
    /// List of operations to be tracked & executed, currently limited to exactly one. We use
    /// a Vec here to accommodate future extensibility.
    pub messages: Vec<Message>,
    /// Current state of the [`Task`]
    pub state: State,
}

/// Functionality for interacting with the underlying [`kv::Store`]
pub trait Cache {
    /// Clears all cached tasks.
    ///
    /// # Errors
    ///
    /// Returns `Err` if access to the [`kv::Store`] fails
    fn clear(&self) -> Result<(), kv::Error>;

    /// Caches a [`Task`] locally.
    ///
    /// # Errors
    ///
    /// Returns `Err` if access to the [`kv::Store`] fails
    fn add(&self, task: Task) -> Result<(), kv::Error>;

    /// Deletes a locally-cached [`Task`] of the given `id`.
    fn delete(&self, id: String) -> Result<(), kv::Error>;

    /// Returns a cached task of the given `id`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if access to the [`kv::Store`] fails
    fn get(&self, id: String) -> Result<Option<Task>, kv::Error>;
}

/// [`kv::Bucket`] to persist tasks
type Tasks = kv::Bucket<'static, &'static str, kv::Json<Task>>;

#[derive(Clone)]
/// Queue to persist & manage [`Task`]s
pub struct Queue {
    // TODO(sos): add api
    // api: coco::Api,
    tasks: Tasks,
}

impl Queue {
    /// Returns a new `Queue` with corresponding [`kv::Bucket`] to persist and manage observed
    /// tasks.
    pub fn new(store: &kv::Store) -> Self {
        Self {
            tasks: store
                .bucket::<&'static str, kv::Json<Task>>(Some("queue"))
                .expect("Unable to retrieve 'queue' bucket"),
        }
    }
}

impl Cache for Queue {
    fn clear(&self) -> Result<(), kv::Error> {
        Ok(self.tasks.clear()?)
    }

    fn add(&self, task: Task) -> Result<(), kv::Error> {
        let key = task.id.as_str();
        self.tasks.set(key, kv::Json(task.clone()))?;

        Ok(())
    }

    fn delete(&self, id: String) -> Result<(), kv::Error> {
        // TODO(sos): Clean up with `find` or other better syntax
        for item in self.tasks.iter() {
            let item = item?;
            let t = item.value::<kv::Json<Task>>()?.to_inner();

            if t.id == id {
                let key: String = item.key()?;
                self.tasks.remove(key.as_str())?;
            }
        }

        Ok(())
    }

    fn get(&self, id: String) -> Result<Option<Task>, kv::Error> {
        let mut task: Option<Task> = None;

        // TODO(sos): Clean up with `find` or other better syntax
        for item in self.tasks.iter() {
            let t = item?.value::<kv::Json<Task>>()?.to_inner();
            if t.id == id {
                task = Some(t)
            };
        }

        Ok(task)
    }
}

#[cfg(test)]
mod test {
    use super::{Cache, Queue, State, Task, Timestamp};

    #[test]
    fn can_add_a_task() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

        let queue = Queue::new(&store);

        let task = Task {
            id: "123".to_string(),
            messages: vec![],
            state: State::Pending {
                timestamp: Timestamp::now(),
            },
        };
        queue.add(task).expect("Failed to cache task");

        let size = queue.tasks.iter().size_hint();
        assert_eq!((1, Some(1)), size);
    }

    #[test]
    fn can_remove_a_task() {
        let mut queue = Queue::new();

        let task = Task {
            id: "123".to_string(),
            messages: vec![],
            state: State::Pending {
                timestamp: Timestamp::now(),
            },
        };

        let other_task = Task {
            id: "567".to_string(),
            messages: vec![],
            state: State::Pending {
                timestamp: Timestamp::now(),
            },
        };

        queue.add(task.clone()).expect("Failed to cache task");
        queue.add(other_task.clone()).expect("Failed to cache task");

        queue.delete(task.id).expect("Failed to delete task");

        assert_eq!(queue.tasks, vec![other_task])
    }

    #[test]
    fn can_retrieve_a_task() {
        let mut queue = Queue::new();

        let task = Task {
            id: "789".to_string(),
            messages: vec![],
            state: State::Pending {
                timestamp: Timestamp::now(),
            },
        };

        queue.add(task.clone()).expect("Failed to cache task");

        assert_eq!(queue.get(task.clone().id), task.clone())
    }

    #[test]
    fn can_store_in_kv() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

        let test = store.bucket::<kv::Raw, kv::Raw>(Some("test")).unwrap();

        test.set(b"test", b"123").expect("could not set store");

        let result = test.get(b"test").unwrap().unwrap();

        assert_eq!(result, "123");
        assert!(test.get(b"something else").unwrap() == None)
    }
}
