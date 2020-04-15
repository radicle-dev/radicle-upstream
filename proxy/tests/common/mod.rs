use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
use librad::git::ProjectId;
use librad::paths::Paths;
use std::sync::Arc;
use tempfile::{tempdir_in, TempDir};
use tokio::sync::RwLock;

use proxy::coco;
use proxy::graphql::schema::{Context, Mutation, Query, Schema};
use proxy::registry;

pub fn with_fixtures<F>(f: F)
where
    F: FnOnce(Paths, TempDir, ProjectId) -> (),
{
    let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
    let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");
    let repos_dir = tempdir_in(tmp_dir.path()).expect("unable to create repos directory");
    let (platinum_id, _platinum_project) = coco::replicate_platinum(
        &tmp_dir,
        &librad_paths,
        "git-platinum",
        "fixture data",
        "master",
    )
    .unwrap();

    f(librad_paths, repos_dir, platinum_id)
}

pub fn execute_query<F>(librad_paths: Paths, query: &str, vars: &Variables, f: F)
where
    F: FnOnce(Value, Vec<ExecutionError<DefaultScalarValue>>) -> (),
{
    let ctx = Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))),
    );
    let (res, errors) = juniper::execute(query, None, &Schema::new(Query, Mutation), vars, &ctx)
        .expect("test execute failed");

    f(res, errors);
}
