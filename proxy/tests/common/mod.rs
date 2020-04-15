use juniper::{DefaultScalarValue, FieldError, Value, Variables};
use librad::git::ProjectId;
use librad::paths::Paths;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tempfile::{tempdir_in, TempDir};
use tokio::sync::RwLock;

use proxy::coco;
use proxy::graphql::schema::{Context, Mutation, Query, Schema};
use proxy::registry;

pub fn with_fixtures<F>(f: F)
where
    F: FnOnce(
        &Context,
        TempDir,
        ProjectId,
    ) -> (
        &'static str,
        Variables,
        Option<Vec<FieldError<DefaultScalarValue>>>,
        Value,
    ),
{
    let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
    let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
    let repos_dir = tempdir_in(tmp_dir.path()).expect("unable to create repos directory");
    let (platinum_id, _platinum_project) = coco::replicate_platinum(
        &tmp_dir,
        &librad_paths,
        "git-platinum",
        "fixture data",
        "master",
    )
    .unwrap();

    let ctx = Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))),
        Arc::new(RwLock::new(store)),
    );

    let (query, vars, expect_errors, expect_res) = f(&ctx, repos_dir, platinum_id);

    let (res, errors) = juniper::execute(query, None, &Schema::new(Query, Mutation), &vars, &ctx)
        .expect("test execute failed");

    if let Some(expect) = expect_errors {
        assert_eq!(
            errors
                .iter()
                .map(|e| e.error())
                .collect::<Vec<&FieldError>>(),
            expect.iter().map(|e| e).collect::<Vec<&FieldError>>(),
        );
    } else {
        assert_eq!(errors, []);
    }
    assert_eq!(res, expect_res);
}
