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

pub struct Queries {
    delay: Delay,
    delta: Duration,
    waiting_room: Arc<RwLock<WaitingRoom<Instant>>>,
}

impl Queries {
    fn new(waiting_room: Arc<RwLock<WaitingRoom<Instant>>>) -> Self {
        Self {
            delay: delay_for(Duration::from_millis(0)),
            delta: Duration::from_secs(1),
            waiting_room,
        }
    }
}

impl Stream for Queries {
    type Item = RadUrn;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let waiting_room = self.waiting_room.clone();
        let waiting_room = futures::executor::block_on(waiting_room.read());

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.delay = delay_for(Duration::from_millis(10));

                waiting_room
                    .next_query(Instant::now(), self.delta)
                    .map_or(Poll::Pending, |urn| Poll::Ready(Some(urn)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Clones {
    delay: Delay,
    delta: Duration,
    waiting_room: Arc<RwLock<WaitingRoom<Instant>>>,
}

impl Clones {
    fn new(waiting_room: Arc<RwLock<WaitingRoom<Instant>>>) -> Self {
        Self {
            delay: delay_for(Duration::from_millis(0)),
            delta: Duration::from_secs(1),
            waiting_room,
        }
    }
}

impl Stream for Clones {
    type Item = RadUrl;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let waiting_room = self.waiting_room.clone();
        let waiting_room = futures::executor::block_on(waiting_room.read());

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.delay = delay_for(Duration::from_millis(10));

                waiting_room
                    .next_clone(Instant::now(), self.delta)
                    .map_or(Poll::Pending, |url| Poll::Ready(Some(url)))
            }
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

    use librad::keys::SecretKey;
    use librad::peer::PeerId;
    use librad::uri::{RadUrl, RadUrn};

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
            Duration::from_millis(50),
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

                for RadUrl { urn, .. } in urls {
                    waiting_room.request(urn.clone(), Instant::now());
                    waiting_room.queried(&urn.clone(), Instant::now()).unwrap();
                }
            });
        }

        let mut have = timeout(
            Duration::from_millis(50),
            clones
                .take(2)
                .zip(stream::repeat(waiting_room.clone()))
                .then(|(url, waiting_room)| async move {
                    let mut waiting_room = waiting_room.write().await;
                    waiting_room
                        .cloning(&url.urn, url.authority.clone(), Instant::now())
                        .expect("signal cloning for request failed");
                    url
                })
                .collect::<Vec<RadUrl>>(),
        )
        .await?;

        // urls.sort();
        // have.sort();

        assert_eq!(urls, have);

        Ok(())
    }
}
