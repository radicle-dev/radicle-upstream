//! Endpoints and serialisation for [`session::Session`] related types.

use serde::ser::SerializeStruct as _;
use serde::{Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::identity;
use crate::session;

/// GET /
pub fn get_filter(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("session")
        .and(warp::get())
        .and(super::with_store(store))
        .and(document::document(document::description(
            "Fetch active Session",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(
                200,
                document::body(session::Session::document()).mime("application/json"),
            )
            .description("Currently active Session"),
        ))
        .and_then(handler::get)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::{reply, Rejection, Reply};

    use crate::session;

    /// Fetch the [`session::Session`].
    pub async fn get(store: Arc<RwLock<kv::Store>>) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        let sess = session::get(&store)?;

        Ok(reply::json(&sess))
    }
}

impl Serialize for session::Session {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Session", 1)?;
        state.serialize_field("identity", &self.identity)?;

        state.end()
    }
}

impl ToDocumentedType for session::Session {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "identity".into(),
            identity::Identity::document().nullable(true),
        );

        document::DocumentedType::from(properties).description("Session")
    }
}

#[cfg(test)]
mod test {}
