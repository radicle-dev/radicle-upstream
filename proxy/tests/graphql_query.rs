#[macro_use]
extern crate juniper;

use hex::ToHex;
use juniper::{InputValue, Variables};
use pretty_assertions::assert_eq;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time;
use tokio::sync::RwLock;

use proxy::coco;
use proxy::graphql::schema;
use proxy::registry;

mod common;
use common::with_fixtures;

#[test]
fn api_version() {
    with_fixtures(|_ctx, _repos_dir, _platinum_id| {
        let query = "query { apiVersion }";
        let res = graphql_value!({ "apiVersion": "1.0" });

        (query, Variables::new(), None, res)
    });
}

#[test]
fn avatar() {
    with_fixtures(|_ctx, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();

        vars.insert("handle".into(), InputValue::scalar("cloudhead"));
        vars.insert("usage".into(), InputValue::Enum("IDENTITY".to_string()));

        let query = "query($handle: ID!, $usage: AvatarUsage!) {
            avatar(handle: $handle, usage: $usage) {
                emoji
                background {
                    r
                    g
                    b
                }
            }
        }";
        let res = graphql_value!({
            "avatar": {
                "emoji": "🚡",
                "background": {
                    "r": 24,
                    "g": 105,
                    "b": 216,
                },
            }
        });

        (query, vars, None, res)
    })
}

#[test]
fn local_branches() {
    with_fixtures(|_ctx, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert(
            "path".into(),
            InputValue::scalar("../fixtures/git-platinum"),
        );

        let query = "query($path: String!) { localBranches(path: $path) }";
        let res = graphql_value!({
            "localBranches": [
                "dev",
                "master",
                "origin/HEAD",
                "origin/dev",
                "origin/master",
            ]
        });

        (query, vars, None, res)
    });
}

#[tokio::test]
async fn list_transactions() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let librad_paths = librad::paths::Paths::from_root(tmp_dir.path()).unwrap();
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
    let mut registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());

    let tx = registry::Transaction {
        id: radicle_registry_client::TxHash::random(),
        messages: vec![registry::Message::ProjectRegistration {
            project_name: radicle_registry_client::ProjectName::from_str("upstream").unwrap(),
            org_id: radicle_registry_client::OrgId::from_str("radicle").unwrap(),
        }],
        state: registry::TransactionState::Applied(radicle_registry_client::Hash::random()),
        timestamp: time::SystemTime::now(),
    };

    registry.cache_transaction(tx.clone()).await;

    let ctx = schema::Context::new(
        Arc::new(RwLock::new(librad_paths)),
        Arc::new(RwLock::new(registry)),
        Arc::new(RwLock::new(store)),
    );

    let mut vars = Variables::new();
    vars.insert(
        "ids".into(),
        InputValue::list(vec![InputValue::scalar(tx.id.encode_hex::<String>())]),
    );
    let query = "query($ids: [ID!]!) {
            listTransactions(ids: $ids) {
                transactions {
                    messages {
                        ... on ProjectRegistrationMessage {
                            kind,
                            projectName,
                            orgId
                        }
                    },
                }
                thresholds {
                    confirmation
                    settlement
                }
            }
        }";

    let (res, errors) = juniper::execute(
        query,
        None,
        &schema::Schema::new(schema::Query, schema::Mutation),
        &vars,
        &ctx,
    )
    .unwrap();

    assert_eq!(errors, []);
    assert_eq!(
        res,
        graphql_value!({
            "listTransactions": {
                "transactions": [
                    {
                        "messages": [
                            {
                                "kind": "PROJECT_REGISTRATION",
                                "projectName": "upstream",
                                "orgId": "radicle",
                            },
                        ],
                    }
                ],
                "thresholds": {
                    "confirmation": 3,
                    "settlement": 9,
                },
            },
        })
    );
}

#[test]
fn project() {
    with_fixtures(|ctx, repos_dir, _platinum_id| {
        let repo_dir = tempfile::tempdir_in(repos_dir.path()).expect("repo dir failed");
        let path = repo_dir.path().to_str().expect("repo path").to_string();
        coco::init_repo(path.clone()).expect("repo init failed");

        let (project_id, _project_meta) = coco::init_project(
            &futures::executor::block_on(ctx.librad_paths.read()),
            &path,
            "upstream",
            "Code collaboration without intermediates.",
            "master",
        )
        .expect("project init failed");

        let id = project_id.to_string();
        let mut vars = Variables::new();
        vars.insert("id".into(), InputValue::scalar(id.clone()));

        let query = "query($id: ID!) {
                    project(id: $id) {
                        id
                        metadata {
                            name
                            description
                            defaultBranch
                        }
                        registered {
                            ... on OrgRegistration {
                                orgId
                            }
                            ... on UserRegistration {
                                userId
                            }
                        }
                    }
                }";
        let res = graphql_value!({
            "project": {
                "id": id,
                "metadata": {
                    "name": "upstream",
                    "description": "Code collaboration without intermediates.",
                    "defaultBranch": "master",
                },
                "registered": None,
            },
        });

        (query, vars, None, res)
    });
}

#[test]
fn identity() {
    with_fixtures(|_ctx, _repo_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("id".into(), InputValue::scalar("123abcd.git"));

        let query = "query($id: ID!) {
                identity(id: $id) {
                    id
                    shareableEntityIdentifier
                    metadata {
                        handle
                        displayName
                        avatarUrl
                    }
                    registered
                    avatarFallback {
                        emoji
                        background {
                            r
                            g
                            b
                        }
                    }
                }
            }";
        let res = graphql_value!({
            "identity": {
                "id": "123abcd.git",
                "shareableEntityIdentifier": "cloudhead@123abcd.git",
                "metadata": {
                    "handle": "cloudhead",
                    "displayName": "Alexis Sellier",
                    "avatarUrl": "https://avatars1.githubusercontent.com/u/40774",
                },
                "registered": None,
                "avatarFallback": {
                    "emoji": "💡",
                    "background": {
                        "r": 122,
                        "g": 112,
                        "b": 90,
                    },
                }
            },
        });

        (query, vars, None, res)
    });
}

#[test]
fn session() {
    with_fixtures(|_ctx, _repo_dir, _platinum_id| {
        let query = "query {
            session {
                identity {
                    id
                    metadata {
                        handle
                        displayName
                        avatarUrl
                    }
                    registered
                }
            }
        }";
        let res = graphql_value!({ "session": { "identity": None } });

        (query, Variables::new(), None, res)
    });
}

#[test]
fn user() {
    with_fixtures(|_ctx, _repo_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("handle".into(), InputValue::scalar("cloudhead"));

        let query = "query($handle: ID!) {
            user(handle: $handle)
        }";
        let res = graphql_value!({ "user": None });

        (query, vars, None, res)
    });
}
