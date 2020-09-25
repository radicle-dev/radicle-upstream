use std::{
    future::Future as _,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use either::Either;
use futures::Stream;
use tokio::{
    sync::RwLock,
    time::{delay_for, Delay},
};

use librad::uri::RadUrn;

use crate::request::{
    waiting_room::WaitingRoom, Created, Found, Request, RequestState, Requested, SomeRequest,
};

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
        let cloon = self.waiting_room.clone();
        let waiting_room = futures::executor::block_on(cloon.read());

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.delay = delay_for(Duration::from_millis(10));

                waiting_room
                    .next_query(Instant::now(), self.delta)
                    .map_or(Poll::Pending, |req| Poll::Ready(Some(req)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Clones {
    waiting_room: Arc<RwLock<WaitingRoom<Instant>>>,
}

impl Stream for Clones {
    type Item = Request<Found, Instant>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use futures::future;
    use futures::StreamExt as _;
    use pretty_assertions::assert_eq;
    use tokio::sync::RwLock;
    use tokio::time::timeout;

    use librad::uri::RadUrn;

    use crate::request::waiting_room::{Config, WaitingRoom};

    use super::Queries;

    #[tokio::test]
    async fn can_stream_queries() -> Result<(), Box<dyn std::error::Error>> {
        let waiting_room = WaitingRoom::new(Config::default());
        let waiting_room = Arc::new(RwLock::new(waiting_room));

        let mut queries = Queries::new(waiting_room.clone());

        let mut urns = vec![
            "rad:git:hwd1yren5bpr71yoy9qzmtk1qzrtren9gynxh49dwubprmqix8dn46x3r8w"
                .parse::<RadUrn>()?,
            "rad:git:hwd1yrerotfs5hskz8cag8at6g16gxa4x6e8snc9pq7ir1s35u95bro9ybr"
                .parse::<RadUrn>()?,
        ];

        let cloon = waiting_room.clone();
        let new_urns = urns.clone();
        tokio::spawn(async move {
            let mut waiting_room = cloon.write().await;

            for urn in new_urns {
                waiting_room.request(urn.clone(), Instant::now());
            }
        });

        let mut have = timeout(Duration::from_millis(50), async move {
            let mut seen = vec![];

            let urn = queries.next().await.unwrap();
            {
                let mut waiting_room = waiting_room.write().await;
                waiting_room.queried(&urn, Instant::now());
            }
            seen.push(urn);

            let urn = queries.next().await.unwrap();
            {
                let mut waiting_room = waiting_room.write().await;
                waiting_room.queried(&urn, Instant::now());
            }
            seen.push(urn);

            seen
        })
        .await?;

        urns.sort();
        have.sort();

        assert_eq!(urns, have);

        Ok(())
    }
}
