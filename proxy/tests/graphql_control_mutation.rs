#[macro_use]
extern crate juniper;

use librad::paths;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tokio::sync::RwLock;

use proxy::graphql::schema;
use proxy::registry;

#[test]
fn nuke_coco_state() {
    let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
    let librad_paths = paths::Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");

    let ctx = schema::Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))),
    );

    let query = "mutation {nukeCocoState}";

    {
        let (res, errors) = juniper::execute(
            query,
            None,
            &schema::create_control(),
            &juniper::Variables::new(),
            &ctx,
        )
        .expect("query execution failed");

        assert_eq!(errors, []);
        assert_eq!(res, graphql_value!({"nukeCocoState": true }));
    }

    // Call twice to check for idempotence.
    {
        let (res, errors) = juniper::execute(
            query,
            None,
            &schema::create_control(),
            &juniper::Variables::new(),
            &ctx,
        )
        .expect("query execution failed");

        assert_eq!(errors, []);
        assert_eq!(res, graphql_value!({"nukeCocoState": true }));
    }
}
