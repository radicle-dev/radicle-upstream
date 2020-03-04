use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
use librad::git::ProjectId;
use librad::paths::Paths;
use tempfile::{tempdir_in, TempDir};

use proxy::coco;
use proxy::graphql::schema::{Context, Mutation, Query, Schema};

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
        "https://avatars0.githubusercontent.com/u/48290027",
    )
    .unwrap();

    f(librad_paths, repos_dir, platinum_id)
}

pub fn execute_query<F>(librad_paths: Paths, query: &str, vars: &Variables, f: F)
where
    F: FnOnce(Value, Vec<ExecutionError<DefaultScalarValue>>) -> (),
{
    let ctx = Context::new(
        librad_paths,
        radicle_registry_client::Client::new_emulator(),
    );
    let (res, errors) = juniper::execute(query, None, &Schema::new(Query, Mutation), vars, &ctx)
        .expect("test execute failed");

    f(res, errors);
}
