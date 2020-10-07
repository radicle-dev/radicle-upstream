//! Streaming interfaces for requests in the [`WaitingRoom`].

use std::{
    future::Future as _,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use futures::Stream;
use tokio::{
    sync::RwLock,
    time::{delay_for, Delay},
};

use librad::uri::{RadUrl, RadUrn};

use crate::request::waiting_room::WaitingRoom;

/// How much a stream should delay before polling the waiting room again.
const REQUEST_DELAY: Duration = Duration::from_millis(20);

/// A stream of queryable requests. See [`Queries::new`] for more information.
pub struct Queries {
    /// How long the stream should delay before each poll of the waiting room.
    delay: Delay,
    /// The waiting room that will be polled for retrieving the requests.
    waiting_room: Arc<RwLock<WaitingRoom<Instant, Duration>>>,
}

impl Queries {
    /// Create a new `Queries` stream.
    ///
    /// This type implements [`Stream`] and is expected to be consumed through that interface.
    ///
    /// The stream will return the next request that is available for querying. This is driven by
    /// the supplied `waiting_room` -- as more requests transtion through it. If no such request
    /// exists then the stream does nothing and waits until the next poll. We note that the stream
    /// is infinite with this respect and will never return `None`.
    #[must_use]
    pub fn new(waiting_room: Arc<RwLock<WaitingRoom<Instant, Duration>>>) -> Self {
        Self {
            delay: delay_for(REQUEST_DELAY),
            waiting_room,
        }
    }

    /// Reset the delay for the stream.
    fn reset_delay(&mut self) {
        self.delay = delay_for(REQUEST_DELAY)
    }
}

impl Stream for Queries {
    type Item = RadUrn;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let waiting_room = self.waiting_room.clone();
        let waiting_room = futures::executor::block_on(waiting_room.read());

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.reset_delay();

                waiting_room
                    .next_query(Instant::now())
                    .map_or(Poll::Pending, |urn| Poll::Ready(Some(urn)))
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A stream of clonable requests. See [`Clones::new`] for more information.
pub struct Clones {
    /// How long the stream should delay before each poll of the waiting room.
    delay: Delay,
    /// The waiting room that will be polled for retrieving the requests.
    waiting_room: Arc<RwLock<WaitingRoom<Instant, Duration>>>,
}

impl Clones {
    /// Create a new `Clones` stream.
    ///
    /// This type implements [`Stream`] and is expected to be consumed through that interface.
    ///
    /// The stream will return the next request that is available for cloning. This is driven by
    /// the supplied `waiting_room` -- as more requests transtion through it. If no such request
    /// exists then the stream does nothing and waits until the next poll. We note that the stream
    /// is infinite with this respect and will never return `None`.
    #[must_use]
    pub fn new(waiting_room: Arc<RwLock<WaitingRoom<Instant, Duration>>>) -> Self {
        Self {
            delay: delay_for(REQUEST_DELAY),
            waiting_room,
        }
    }

    /// Reset the delay for the stream.
    fn reset_delay(&mut self) {
        self.delay = delay_for(REQUEST_DELAY)
    }
}

impl Stream for Clones {
    type Item = RadUrl;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let waiting_room = self.waiting_room.clone();
        let waiting_room = futures::executor::block_on(waiting_room.read());

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.reset_delay();
                waiting_room
                    .next_clone()
                    .map_or(Poll::Pending, |url| Poll::Ready(Some(url)))
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::Arc,
        time::{Duration, Instant},
    };

    use futures::{stream, StreamExt as _};
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use tokio::{sync::RwLock, time::timeout};

    use librad::{
        keys::SecretKey,
        peer::PeerId,
        uri::{RadUrl, RadUrn},
    };

    use crate::request::waiting_room::{Config, WaitingRoom};

    use super::{Clones, Queries};

    // TODO(xla): Find a way to create PeerId from static seed.
    lazy_static! {
        static ref LYLA_PEER_ID: PeerId = PeerId::from(SecretKey::new());
        static ref ROVER_PEER_ID: PeerId = PeerId::from(SecretKey::new());
    }

    #[tokio::test]
    async fn can_stream_queries() -> Result<(), Box<dyn std::error::Error>> {
        let waiting_room = WaitingRoom::new(Config::default());
        let waiting_room = Arc::new(RwLock::new(waiting_room));

        let queries = Queries::new(waiting_room.clone());

        let mut urns = vec![
            "rad:git:hwd1yren5bpr71yoy9qzmtk1qzrtren9gynxh49dwubprmqix8dn46x3r8w"
                .parse::<RadUrn>()?,
            "rad:git:hwd1yrerotfs5hskz8cag8at6g16gxa4x6e8snc9pq7ir1s35u95bro9ybr"
                .parse::<RadUrn>()?,
        ];

        {
            let waiting_room = waiting_room.clone();
            let urns = urns.clone();
            tokio::spawn(async move {
                let mut waiting_room = waiting_room.write().await;

                for urn in urns {
                    waiting_room.request(urn.clone(), Instant::now());
                }
            });
        }

        let mut have = timeout(
            Duration::from_millis(150),
            queries
                .take(2)
                .zip(stream::repeat(waiting_room.clone()))
                .then(|(urn, waiting_room)| async move {
                    let mut waiting_room = waiting_room.write().await;
                    waiting_room
                        .queried(&urn, Instant::now())
                        .expect("signal queried for request failed");
                    urn
                })
                .collect::<Vec<RadUrn>>(),
        )
        .await?;

        urns.sort();
        have.sort();

        assert_eq!(urns, have);

        Ok(())
    }

    #[tokio::test]
    async fn can_stream_clones() -> Result<(), Box<dyn std::error::Error>> {
        let waiting_room = WaitingRoom::new(Config::default());
        let waiting_room = Arc::new(RwLock::new(waiting_room));

        let clones = Clones::new(waiting_room.clone());

        let mut urls = vec![
            RadUrl {
                authority: LYLA_PEER_ID.clone(),
                urn: "rad:git:hwd1yren5bpr71yoy9qzmtk1qzrtren9gynxh49dwubprmqix8dn46x3r8w"
                    .parse::<RadUrn>()?,
            },
            RadUrl {
                authority: ROVER_PEER_ID.clone(),
                urn: "rad:git:hwd1yrerotfs5hskz8cag8at6g16gxa4x6e8snc9pq7ir1s35u95bro9ybr"
                    .parse::<RadUrn>()?,
            },
        ];

        {
            let waiting_room = waiting_room.clone();
            let urls = urls.clone();
            tokio::spawn(async move {
                let mut waiting_room = waiting_room.write().await;

                for url in urls {
                    waiting_room.request(url.urn.clone(), Instant::now());
                    waiting_room
                        .queried(&url.urn, Instant::now())
                        .expect("failed to query waiting room");
                    waiting_room
                        .found(url, Instant::now())
                        .expect("failed to mark peer as found");
                }
            });
        }

        let mut have = timeout(
            Duration::from_millis(150),
            clones
                .take(2)
                .zip(stream::repeat(waiting_room.clone()))
                .then(|(url, waiting_room)| async move {
                    let mut waiting_room = waiting_room.write().await;
                    waiting_room
                        .cloning(url.clone(), Instant::now())
                        .expect("signal cloning for request failed");
                    url
                })
                .collect::<Vec<RadUrl>>(),
        )
        .await?;

        urls.sort_by_key(|url| url.urn.clone());
        have.sort_by_key(|url| url.urn.clone());

        assert_eq!(urls, have);

        Ok(())
    }
}
