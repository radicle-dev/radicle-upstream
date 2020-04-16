//! Endpoints and serialisation for source code browsing.

use librad::paths::Paths;
use serde::ser::SerializeStruct as _;
use serde::{Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::coco;

/// Combination of all source routes.
pub fn filters(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    blob_filter(paths)
}

/// GET /blob/<project_id>/<revision>/<path...>
fn blob_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("blob")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::param::<String>(
            "revision",
            "Git revision of the blobs content",
        ))
        .and(document::tail(
            "path",
            "Location of the Blob in the repository tree",
        ))
        .and(document::document(document::description("Fetch a Blob")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Blob::document()).mime("application/json"),
            )
            .description("Returns Blob"),
        ))
        .and_then(handler::blob)
}

/// Source handlers for conversion beetween core domain and http request fullfilment.
mod handler {
    use librad::paths::Paths;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::path::Tail;
    use warp::{reply, Rejection, Reply};

    use crate::coco;

    /// Fetch a [`coco::Blob`].
    pub async fn blob(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        revision: String,
        path: Tail,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let blob = coco::blob(&paths, &project_id, &revision, path.as_str())?;

        Ok(reply::json(&blob))
    }
}

impl Serialize for coco::Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Blob", 3)?;
        state.serialize_field("binary", &self.is_binary())?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("info", &self.info)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Blob {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "binary".into(),
            document::boolean()
                .description("Flag to indicate if the content of the Blob is binary")
                .example(true),
        );
        properties.insert("content".into(), coco::BlobContent::document());
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("Blob")
    }
}

impl Serialize for coco::BlobContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ascii(content) => serializer.serialize_str(content),
            Self::Binary => serializer.serialize_none(),
        }
    }
}

impl ToDocumentedType for coco::BlobContent {
    fn document() -> document::DocumentedType {
        document::string()
            .description("BlobContent")
            .example("print 'hello world'")
            .nullable(true)
    }
}

impl Serialize for coco::Commit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Commit", 6)?;
        state.serialize_field("sha1", &self.sha1.to_string())?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("summary", &self.summary)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("committer", &self.committer)?;
        state.serialize_field("committerTime", &self.committer_time.seconds().to_string())?;
        state.end()
    }
}

impl ToDocumentedType for coco::Commit {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(6);
        properties.insert(
            "sha1".into(),
            document::string()
                .description("SHA1 of the Commit")
                .example("1e0206da8571ca71c51c91154e2fee376e09b4e7"),
        );
        properties.insert("author".into(), coco::Person::document());
        properties.insert(
            "summary".into(),
            document::string()
                .description("Commit message summary")
                .example("Add text files"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("Commit description text")
                .example("Longer desription of the Commit changes."),
        );
        properties.insert("committer".into(), coco::Person::document());
        properties.insert(
            "committerTime".into(),
            document::string()
                .description("Time of the commit")
                .example("1575283425"),
        );
        document::DocumentedType::from(properties).description("Commit")
    }
}

impl Serialize for coco::Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("objectType", &self.object_type)?;
        state.serialize_field("lastCommit", &self.last_commit)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Info {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the file")
                .example("arrows.txt"),
        );
        properties.insert("objectType".into(), coco::ObjectType::document());
        properties.insert("lastCommit".into(), coco::Commit::document());

        document::DocumentedType::from(properties).description("Info")
    }
}

impl Serialize for coco::ObjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Blob => serializer.serialize_unit_variant("ObjectType", 0, "BLOB"),
            Self::Tree => serializer.serialize_unit_variant("ObjectType", 1, "TREE"),
        }
    }
}

impl ToDocumentedType for coco::ObjectType {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["BLOB".to_string(), "TREE".to_string()])
            .description("Object type variants")
            .example(Self::Blob)
    }
}

impl Serialize for coco::Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("avatar", &self.avatar)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Person {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name part of the commit signature.")
                .example("Alexis Sellier"),
        );
        properties.insert(
            "email".into(),
            document::string()
                .description("Email part of the commit signature.")
                .example("self@cloudhead.io"),
        );
        properties.insert(
            "avatar".into(),
            document::string()
                .description("Reference (url/uri) to a persons avatar image.")
                .example("https://avatars1.githubusercontent.com/u/40774"),
        );

        document::DocumentedType::from(properties).description("Person")
    }
}

#[allow(clippy::non_ascii_literal, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use librad::paths::Paths;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;

    #[tokio::test]
    async fn blob() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let (platinum_id, _platinum_project) = coco::replicate_platinum(
            &tmp_dir,
            &librad_paths,
            "git-platinum",
            "fixture data",
            "master",
        )
        .unwrap();
        let revision = "master";
        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));

        for path in &["text/arrows.txt", "bin/ls"] {
            let want = coco::blob(&librad_paths, &platinum_id.to_string(), revision, path).unwrap();

            let res = request()
                .method("GET")
                .path(&format!("/blob/{}/{}/{}", platinum_id, revision, path,))
                .reply(&api)
                .await;

            let have: Value = serde_json::from_slice(res.body()).unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert_eq!(have, json!(want));
        }
    }
}
