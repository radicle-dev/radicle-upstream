#[macro_use]
extern crate juniper;

use librad::paths;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tokio::sync::RwLock;

use proxy::graphql::schema;
use proxy::identity;
use proxy::registry;
use proxy::session;

#[test]
fn nuke_coco_state() {
    let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
    let librad_paths = paths::Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

    let ctx = schema::Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))),
        Arc::new(RwLock::new(store)),
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

#[test]
fn nuke_session_state() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir().unwrap();
    let librad_paths = paths::Paths::from_root(tmp_dir.path()).unwrap();
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

    let ctx = schema::Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))),
        Arc::new(RwLock::new(store)),
    );

    // Create an identity and store it in the session.
    {
        let id = identity::create("cloudhead".into(), None, None).unwrap();
        session::set(
            &futures::executor::block_on(ctx.store.read()),
            session::Session { identity: Some(id) },
        )
        .unwrap();
    }

    // Query session and check for session presence.
    {
        let query = "query { session { identity { id }}}";
        let (res, errors) = juniper::execute(
            query,
            None,
            &schema::create(),
            &juniper::Variables::new(),
            &ctx,
        )
        .unwrap();

        assert_eq!(errors, []);
        assert_eq!(
            res,
            graphql_value!({"session": { "identity": { "id": "123abcd.git" } } })
        );
    }

    // Nuke session state.
    {
        let query = "mutation { nukeSessionState }";
        let (res, errors) = juniper::execute(
            query,
            None,
            &schema::create_control(),
            &juniper::Variables::new(),
            &ctx,
        )
        .unwrap();

        assert_eq!(errors, []);
        assert_eq!(res, graphql_value!({"nukeSessionState": true }));
    }

    // Check that session state is empty.
    {
        let query = "query { session { identity { id }}}";
        let (res, errors) = juniper::execute(
            query,
            None,
            &schema::create(),
            &juniper::Variables::new(),
            &ctx,
        )
        .unwrap();

        assert_eq!(errors, []);
        assert_eq!(res, graphql_value!({"session": { "identity": None } }));
    }

    Ok(())
}
