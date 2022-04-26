// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use std::sync::{Arc, Weak};

/// [`SemaphoreMap`] allows for synchronization around named resources.
///
/// Conceptually, [`SemaphoreMap`] is a map where every key corresponds to a
/// [`tokio::sync::Semaphore`] with one permit. This semaphore can be used to synchronize access to
/// a resource identified by the key.
#[derive(Debug, Clone)]
pub struct SemaphoreMap<K> {
    // invariant enforced by the mutex: `Weak` has at least one strong reference.
    inner: Arc<parking_lot::Mutex<std::collections::HashMap<K, Weak<tokio::sync::Semaphore>>>>,
}

impl<K: Eq + std::hash::Hash + Clone + Send> SemaphoreMap<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquire a permit for `key`.
    ///
    /// See also [`tokio::sync::Semaphore::acquire`].
    pub async fn acquire(&self, key: K) -> SemaphoreMapPermit<'_, K> {
        let mutex = {
            let mut inner_map = self.inner.lock();
            let entry = inner_map.entry(key.clone());
            match entry {
                std::collections::hash_map::Entry::Occupied(occ) => Weak::upgrade(occ.get())
                    .expect("invariant violated: occupied entry has no strong reference"),
                std::collections::hash_map::Entry::Vacant(vac) => {
                    let mutex = Arc::new(tokio::sync::Semaphore::new(1));
                    vac.insert(Arc::downgrade(&mutex));
                    mutex
                },
            }
        };

        let permit = mutex
            .acquire_owned()
            .await
            .expect("permits cannot be closed");
        SemaphoreMapPermit {
            owner: self,
            key: Some(key),
            permit: Some(permit),
        }
    }
}

impl<K: Eq + std::hash::Hash + Clone + Send> Default for SemaphoreMap<K> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

pub struct SemaphoreMapPermit<'a, K>
where
    K: Eq + std::hash::Hash + Clone + Send,
{
    owner: &'a SemaphoreMap<K>,
    key: Option<K>,
    permit: Option<tokio::sync::OwnedSemaphorePermit>,
}

impl<'a, K: Eq + std::hash::Hash + Clone + Send> Drop for SemaphoreMapPermit<'a, K> {
    fn drop(&mut self) {
        // Drop the owned permit first so that this permit is not considered when checking the
        // reference count below.
        drop(self.permit.take().expect("permit already taken"));
        let mut inner_map = self.owner.inner.lock();
        let entry = inner_map.entry(self.key.take().expect("key already taken"));
        match entry {
            std::collections::hash_map::Entry::Occupied(occ) => {
                if Weak::strong_count(occ.get()) == 0 {
                    occ.remove_entry();
                }
            },
            std::collections::hash_map::Entry::Vacant(_) => {},
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::prelude::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn serialize_for_same_key() {
        let map = SemaphoreMap::<usize>::new();
        let task_count = 1;
        let barrier = Arc::new(tokio::sync::Barrier::new(task_count));
        let lock_1 = Arc::new(tokio::sync::Mutex::new(()));
        let tasks = (0..task_count)
            .map({
                let map = map.clone();
                move |_| {
                    let map = map.clone();
                    let lock_1 = lock_1.clone();
                    let barrier = barrier.clone();
                    tokio::task::spawn(async move {
                        barrier.wait().await;
                        let map_lock = map.acquire(1).await;
                        let locked_1 = lock_1.try_lock().unwrap();
                        // make the ciritical section longer to make it more likely to run into
                        // potential bugs.
                        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                        drop(locked_1);
                        drop(map_lock);
                    })
                }
            })
            .collect::<Vec<_>>();
        future::try_join_all(tasks).await.unwrap();
        assert!(map.inner.lock().is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_for_different_keys() {
        let map = SemaphoreMap::<usize>::new();
        let task_count = 100;
        let barrier = Arc::new(tokio::sync::Barrier::new(task_count));
        let tasks = (0..task_count)
            .map({
                let map = map.clone();
                move |x| {
                    let map = map.clone();
                    let barrier = barrier.clone();
                    tokio::task::spawn(async move {
                        let _map_lock = map.acquire(x).await;
                        barrier.wait().await;
                    })
                }
            })
            .collect::<Vec<_>>();
        future::try_join_all(tasks).await.unwrap();
        assert!(map.inner.lock().is_empty());
    }
}
