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
    blob_filter(Arc::<RwLock<Paths>>::clone(&paths))
        .or(commit_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(tree_filter(paths))
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
            .description("Blob for path found"),
        ))
        .and_then(handler::blob)
}

/// GET /commit/<project_id>/<sha1>
fn commit_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("commit")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::param::<String>("sha1", "Git object id"))
        .and(document::document(document::description("Fetch a Commit")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Commit::document()).mime("application/json"),
            )
            .description("Commit for SHA1 found"),
        ))
        .and_then(handler::commit)
}

/// GET /tree/<project_id>/<revision>/<prefix>
fn tree_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tree")
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
        .and(document::tail("prefix", "Path prefix to query"))
        .and(document::document(document::description("Fetch a Tree")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Blob::document()).mime("application/json"),
            )
            .description("Tree for path found"),
        ))
        .and_then(handler::tree)
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

    /// Fetch a [`coco::Commit`].
    pub async fn commit(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        sha1: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let commit = coco::commit(&paths, &project_id, &sha1)?;

        Ok(reply::json(&commit))
    }

    /// Fetch a [`coco::Tree`].
    pub async fn tree(
        librad_paths: Arc<RwLock<Paths>>,
        project_id: String,
        revision: String,
        prefix: Tail,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let tree = coco::tree(&paths, &project_id, &revision, prefix.as_str())?;

        Ok(reply::json(&tree))
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

impl Serialize for coco::Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tree", 3)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("entries", &self.entries)?;
        state.serialize_field("info", &self.info)?;
        state.end()
    }
}

impl ToDocumentedType for coco::Tree {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "path".into(),
            document::string()
                .description("Absolute path to the tree object from the repo root.")
                .example("ui/src"),
        );
        properties.insert(
            "entries".into(),
            document::array(coco::TreeEntry::document())
                .description("Entries listed in that tree result."),
        );
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("Tree")
    }
}

impl Serialize for coco::TreeEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tree", 2)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("info", &self.info)?;
        state.end()
    }
}

impl ToDocumentedType for coco::TreeEntry {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert(
            "path".into(),
            document::string()
                .description("Absolute path to the object from the root of the repo.")
                .example("ui/src/main.ts"),
        );
        properties.insert("info".into(), coco::Info::document());

        document::DocumentedType::from(properties).description("TreeEntry")
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

        // Get ASCII blob.
        let path = "text/arrows.txt";
        let res = request()
            .method("GET")
            .path(&format!("/blob/{}/{}/{}", platinum_id, revision, path))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::blob(&librad_paths, &platinum_id.to_string(), revision, path).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "binary": false,
                "content": "  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
..;;;;;..    ..;;;;;..    ..;;;;;..
 ':::::'      ':::::'      ':::::'
   ':`          ':`          ':`
",
                "info": {
                    "name": "arrows.txt",
                    "objectType": "BLOB",
                    "lastCommit": {
                        "sha1": "1e0206da8571ca71c51c91154e2fee376e09b4e7",
                        "author": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "summary": "Add text files",
                        "description": "",
                        "committerTime": "1575283425",
                    },
                },
            })
        );

        // Get binary blob.
        let path = "bin/ls";
        let res = request()
            .method("GET")
            .path(&format!("/blob/{}/{}/{}", platinum_id, revision, path))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::blob(&librad_paths, &platinum_id.to_string(), revision, path).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "binary": true,
                "content": Value::Null,
                "info": {
                    "name": "ls",
                    "objectType": "BLOB",
                    "lastCommit": {
                        "sha1": "19bec071db6474af89c866a1bd0e4b1ff76e2b97",
                        "author": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                            "name": "Rūdolfs Ošiņš",
                            "email": "rudolfs@osins.org",
                        },
                        "summary": "Add some binary files",
                        "description": "",
                        "committerTime": "1575282964", },
                },
            })
        );
    }

    #[tokio::test]
    async fn commit() {
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

        let sha1 = "3873745c8f6ffb45c990eb23b491d4b4b6182f95";

        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/commit/{}/{}", platinum_id, sha1))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::commit(&librad_paths, &platinum_id.to_string(), sha1).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "sha1": sha1,
                "author": {
                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6367167426181048581.svg",
                    "name": "Fintan Halpenny",
                    "email": "fintan.halpenny@gmail.com",
                },
                "committer": {
                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                    "email": "noreply@github.com",
                    "name": "GitHub",
                },
                "summary": "Extend the docs (#2)",
                "description": "I want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                "committerTime": "1578309972",
            }),
        );
    }

    #[tokio::test]
    async fn tree() {
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
        let prefix = "src";

        let api = super::filters(Arc::new(RwLock::new(librad_paths.clone())));
        let res = request()
            .method("GET")
            .path(&format!("/tree/{}/{}/{}", platinum_id, revision, prefix))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = coco::tree(&librad_paths, &platinum_id.to_string(), revision, prefix).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(want));
        assert_eq!(
            have,
            json!({
                "path": "src",
                "info": {
                    "name": "src",
                    "objectType": "TREE",
                    "lastCommit": {
                        "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                        "author": {
                            "avatar":  "https://avatars.dicebear.com/v2/jdenticon/4800695552551917589.svg",
                            "name": "Alexander Simmerl",
                            "email": "a.simmerl@gmail.com",
                        },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                            "email": "noreply@github.com",
                            "name": "GitHub",
                        },
                        "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                        "description": "Updated README",
                        "committerTime": "1584367899",
                    },
                },
                "entries": [
                    {
                        "path": "src/Eval.hs",
                        "info": {
                            "name": "Eval.hs",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                                "author": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/4800695552551917589.svg",
                                    "name": "Alexander Simmerl",
                                    "email": "a.simmerl@gmail.com",
                                },
                        "committer": {
                            "avatar": "https://avatars.dicebear.com/v2/jdenticon/16701125315436463681.svg",
                            "email": "noreply@github.com",
                            "name": "GitHub",
                        },
                                "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                                "description": "Updated README",
                                "committerTime": "1584367899",
                            },
                        },
                    },
                    {
                        "path": "src/memory.rs",
                        "info": {
                            "name": "memory.rs",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "e24124b7538658220b5aaf3b6ef53758f0a106dc",
                                "author": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "committer": {
                                    "avatar": "https://avatars.dicebear.com/v2/jdenticon/6579925199124505498.svg",
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "summary": "Move examples to \"src\"",
                                "description": "",
                                "committerTime": "1575283266",
                            },
                        },
                    },
                ],
            }),
        );
    }
}
