use std::collections::HashMap;

use librad::uri::RadUrn;

use crate::request::{
    self, Attempt, Canceled, Created, Fulfilled, PeerId, Repo, Request, Requested, SomeRequest,
    TimedOut,
};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] request::Error),
    #[error("the URN '{0}' was not found in the waiting room")]
    MissingUrn(RadUrn),
    #[error("the state fetched from the waiting room was not the expected state")]
    StateMismatch,
}

pub struct WaitingRoom<T> {
    requests: HashMap<RadUrn, SomeRequest<T>>,
}

// TODO(finto): Consider fairness of selection, e.g. rando sample, fifo, lifo, instant
// comparison, etc.
// TODO(finto): Test scenario of "running" a request and updating the waiting room. Testing state
// transitions.
// TODO(finto): De/Serialize for waiting room.
impl<T> WaitingRoom<T> {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }

    pub fn create(&mut self, urn: RadUrn, timestamp: T) -> Option<SomeRequest<T>>
    where
        T: Clone,
    {
        match self.requests.get(&urn) {
            None => {
                let request = SomeRequest::Created(Request::new(urn.clone(), timestamp));
                self.requests.insert(urn, request);
                None
            }
            Some(request) => Some(request.clone()),
        }
    }

    fn transition<Prev, Next>(
        &mut self,
        matcher: impl FnOnce(&SomeRequest<T>) -> Option<&Prev>,
        transition: impl FnOnce(Prev) -> Result<Next, request::Error>,
        urn: &RadUrn,
    ) -> Result<Next, Error>
    where
        Prev: Clone,
        Next: Into<SomeRequest<T>> + Clone,
    {
        match self.requests.get(urn) {
            None => Err(Error::MissingUrn(urn.clone())),
            Some(request) => match matcher(request) {
                Some(previous) => {
                    let next = transition(previous.clone())?;
                    self.requests.insert(urn.clone(), next.clone().into());
                    Ok(next)
                }
                None => Err(Error::StateMismatch),
            },
        }
    }

    pub fn requested(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<Requested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.request(timestamp)),
            urn,
        )
    }

    pub fn attempted(
        &mut self,
        urn: &RadUrn,
        attempt: Attempt,
    ) -> Result<Request<Requested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.attempt(attempt)),
            urn,
        )
    }

    pub fn found_peer(
        &mut self,
        urn: &RadUrn,
        peer: PeerId,
        timestamp: T,
    ) -> Result<Request<Requested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.found_peer(peer, timestamp)),
            urn,
        )
    }

    pub fn cloning(
        &mut self,
        urn: &RadUrn,
        peer: PeerId,
        timestamp: T,
    ) -> Result<Request<Requested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.cloning(peer, timestamp)),
            urn,
        )
    }

    pub fn fulfilled(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
        repo: Repo,
    ) -> Result<Request<Fulfilled, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| previous.fulfilled(repo, timestamp),
            urn,
        )
    }

    pub fn canceled(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<Canceled, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |prev| Ok(prev.cancel(timestamp)),
            urn,
        )
    }

    pub fn timed_out(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<TimedOut, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |prev| Ok(prev.timed_out(timestamp)),
            urn,
        )
    }

    pub fn list(&self) -> impl Iterator<Item = &RadUrn> {
        self.requests.keys()
    }

    pub fn next(&self) -> Option<&Request<Created, T>> {
        self.requests
            .iter()
            .filter_map(|(_, request)| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            })
            .next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use librad::uri::RadUrn;

    #[test]
    fn happy_path_of_full_request() {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new();
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let request = waiting_room.create(urn.clone(), ());

        assert_eq!(request, None);

        let created = waiting_room.next();
        assert_eq!(created, Some(&Request::new(urn.clone(), ())),);

        let requested = waiting_room.requested(&urn, ());
        assert_eq!(requested, Ok(Request::new(urn.clone(), ()).request(())));

        let requested = waiting_room.attempted(&urn, Attempt::Query);
        assert_eq!(
            requested,
            Ok(Request::new(urn.clone(), ())
                .request(())
                .attempt(Attempt::Query))
        );

        let found = waiting_room.found_peer(&urn, "peer1".to_string(), ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ());
        assert_eq!(found, Ok(expected),);

        let requested = waiting_room.attempted(&urn, Attempt::CloneRepo);
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo);
        assert_eq!(requested, Ok(expected));

        let requested = waiting_room.cloning(&urn, "peer1".to_string(), ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo)
            .cloning("peer1".to_string(), ());
        assert_eq!(requested, Ok(expected));

        let repo = Repo {
            urn: urn.clone(),
            name: "next-please!".to_string(),
        };

        let fulfilled = waiting_room.fulfilled(&urn, (), repo.clone());
        let expected = Request::new(urn, ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo)
            .cloning("peer1".to_string(), ())
            .fulfilled(repo, ());
        assert_eq!(fulfilled, expected);
    }
}
