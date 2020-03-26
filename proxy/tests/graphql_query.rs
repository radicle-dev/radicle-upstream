#[macro_use]
extern crate juniper;

use juniper::{InputValue, Variables};
use pretty_assertions::assert_eq;
use std::str::FromStr as _;
use std::time;

use proxy::coco;
use proxy::graphql::schema;
use proxy::registry;

mod common;

use common::{execute_query, with_fixtures};

#[test]
fn api_version() {
    with_fixtures(|librad_paths, _repos_dir, _platinum_id| {
        let query = "query { apiVersion }";

        execute_query(librad_paths, query, &Variables::new(), |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(res, graphql_value!({ "apiVersion": "1.0" }));
        });
    });
}

#[test]
fn avatar() {
    with_fixtures(|librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();

        vars.insert("handle".into(), InputValue::scalar("cloudhead"));

        let query = "query($handle: ID!) {
            avatar(handle: $handle) {
                emoji
                background {
                    r
                    g
                    b
                }
            }
        }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "avatar": {
                        "emoji": "📐",
                        "background": {
                            "r": 24,
                            "g": 105,
                            "b": 216,
                        },
                    }
                })
            );
        });
    })
}

#[test]
fn blob() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("path".into(), InputValue::scalar("text/arrows.txt"));

        let query = "query($id: ID!, $revision: String!, $path: String!) {
                    blob(id: $id, revision: $revision, path: $path) {
                        binary,
                        content,
                        info {
                            name,
                            objectType,
                            lastCommit{
                                sha1,
                                author {
                                    name,
                                    email,
                                },
                                summary,
                                message,
                                committerTime,
                            },
                        },
                    }
                }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "blob": {
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
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "summary": "Add text files",
                                "message": "Add text files\n",
                                "committerTime": "1575283425",
                            },
                        },
                    }
                }),
            );
        });
    });
}

#[test]
fn blob_binary() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("path".into(), InputValue::scalar("bin/ls"));

        let query = "query($id: ID!, $revision: String!, $path: String!) {
                    blob(id: $id, revision: $revision, path: $path) {
                        binary,
                        content,
                        info {
                            name,
                            objectType,
                            lastCommit{
                                sha1,
                                author {
                                    name,
                                    email,
                                },
                                summary,
                                message,
                                committerTime,
                            },
                        },
                    }
                }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "blob": {
                        "binary": true,
                        "content": None,
                        "info": {
                            "name": "ls",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "19bec071db6474af89c866a1bd0e4b1ff76e2b97",
                                "author": {
                                    "name": "Rūdolfs Ošiņš",
                                    "email": "rudolfs@osins.org",
                                },
                                "summary": "Add some binary files",
                                "message": "Add some binary files\n",
                                "committerTime": "1575282964",
                            },
                        },
                    }
                }),
            );
        });
    });
}

#[test]
fn blob_in_root() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("path".into(), InputValue::scalar("README.md"));

        let query = "query($id: ID!, $revision: String!, $path: String!) {
                    blob(id: $id, revision: $revision, path: $path) {
                        content,
                        info {
                            name,
                            objectType,
                            lastCommit{
                                sha1,
                                author {
                                    name,
                                    email,
                                },
                                summary,
                                message,
                                committerTime,
                            },
                        },
                    }
                }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "blob": {
                        "content": "This repository is a data source for the Upstream front-end tests and the\n[`radicle-surf`](https://github.com/radicle-dev/git-platinum) unit tests.\n",
                        "info": {
                            "name": "README.md",
                            "objectType": "BLOB",
                            "lastCommit": {
                                "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                                "author": {
                                    "name": "Alexander Simmerl",
                                    "email": "a.simmerl@gmail.com",
                                },
                                "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                                "message": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig\n\nUpdated README",
                                "committerTime": "1584367899",
                            },
                        },
                    }
                }),
            );
        });
    });
}

#[test]
fn branches() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();
        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));

        let query = "query($id: ID!) { branches(id: $id) }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "branches": [
                        "dev",
                        "master",
                        "rad/contributor",
                        "rad/project",
                    ]
                }),
            );
        });
    });
}

#[test]
fn local_branches() {
    with_fixtures(|librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert(
            "path".into(),
            InputValue::scalar("../fixtures/git-platinum"),
        );

        let query = "query($path: String!) { localBranches(path: $path) }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "localBranches": [
                        "dev",
                        "master",
                        "origin/HEAD",
                        "origin/dev",
                        "origin/master",
                    ]
                }),
            );
        });
    });
}

#[test]
fn commit() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        const SHA1: &str = "3873745c8f6ffb45c990eb23b491d4b4b6182f95";

        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("sha1".into(), InputValue::scalar(SHA1));

        let query = "query($id: ID!, $sha1: String!) {
                    commit(id: $id, sha1: $sha1) {
                        sha1,
                        author {
                            name,
                            email,
                        },
                        summary,
                        message,
                        committerTime,
                    }
                }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "commit": {
                        "sha1": SHA1,
                        "author": {
                            "name": "Fintan Halpenny",
                            "email": "fintan.halpenny@gmail.com",
                        },
                        "summary": "Extend the docs (#2)",
                        "message": "Extend the docs (#2)\n\nI want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                        "committerTime": "1578309972",
                    },
                }),
            )
        });
    });
}

#[test]
fn tags() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();
        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));

        let query = "query($id: ID!) { tags(id: $id) }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "tags": [
                        "v0.1.0",
                        "v0.2.0",
                        "v0.3.0",
                        "v0.4.0",
                        "v0.5.0",
                    ]
                }),
            )
        });
    });
}

#[allow(clippy::too_many_lines)]
#[test]
fn tree() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("prefix".into(), InputValue::scalar("src"));

        let query = "query($id: ID!, $revision: String!, $prefix: String!) {
                    tree(id: $id, revision: $revision, prefix: $prefix) {
                        path,
                        info {
                            name
                            objectType
                            lastCommit {
                                sha1,
                                author {
                                    name,
                                    email,
                                },
                                summary,
                                message,
                                committerTime,
                            }
                        }
                        entries {
                            path,
                            info {
                                name,
                                objectType,
                                lastCommit {
                                    sha1,
                                    author {
                                        name,
                                        email,
                                    },
                                    summary,
                                    message,
                                    committerTime,
                                }
                            },
                        },
                    }
                }";
        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "tree": {
                        "path": "src",
                        "info": {
                            "name": "src",
                            "objectType": "TREE",
                            "lastCommit": {
                                "sha1": "223aaf87d6ea62eef0014857640fd7c8dd0f80b5",
                                "author": {
                                    "name": "Alexander Simmerl",
                                    "email": "a.simmerl@gmail.com",
                                },
                                "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                                "message": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig\n\nUpdated README",
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
                                            "name": "Alexander Simmerl",
                                            "email": "a.simmerl@gmail.com",
                                        },
                                        "summary": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
                                        "message": "Merge pull request #4 from FintanH/fintan/update-readme-no-sig\n\nUpdated README",
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
                                            "name": "Rūdolfs Ošiņš",
                                            "email": "rudolfs@osins.org",
                                        },
                                        "summary": "Move examples to \"src\"",
                                        "message": "Move examples to \"src\"\n",
                                        "committerTime": "1575283266",
                                    },
                                },
                            },
                        ],
                    }
                }),
            );
        });
    });
}

#[test]
fn tree_root() {
    with_fixtures(|librad_paths, _repos_dir, platinum_id| {
        let mut vars = Variables::new();

        vars.insert("id".into(), InputValue::scalar(platinum_id.to_string()));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("prefix".into(), InputValue::scalar(""));

        let query = "query($id: ID!, $revision: String!, $prefix: String!) {
                    tree(id: $id, revision: $revision, prefix: $prefix) {
                        path,
                        info {
                            name
                            objectType
                        }
                        entries {
                            path,
                            info {
                                objectType
                            }
                        },
                    }
                }";

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "tree": {
                        "path": "",
                        "info": {
                            "name": "",
                            "objectType": "TREE",
                        },
                        "entries": [
                            { "path": "bin", "info": { "objectType": "TREE" } },
                            { "path": "src", "info": { "objectType": "TREE" } },
                            { "path": "text", "info": { "objectType": "TREE" } },
                            { "path": "this", "info": { "objectType": "TREE" } },
                            { "path": ".i-am-well-hidden", "info": { "objectType": "BLOB" } },
                            { "path": ".i-too-am-hidden", "info": { "objectType": "BLOB" } },
                            { "path": "README.md", "info": { "objectType": "BLOB" } },
                        ],
                    }
                }),
            );
        });
    });
}

#[tokio::test]
async fn list_transactions() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let librad_paths = librad::paths::Paths::from_root(tmp_dir.path()).unwrap();
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

    let ctx = schema::Context::new(librad_paths, registry);

    let mut vars = Variables::new();
    vars.insert("ids".into(), InputValue::list(vec![]));
    let query = "query($ids: [ID!]!) {
            listTransactions(ids: $ids) {
                transactions {
                    messages {
                        ... on ProjectRegistrationMessage {
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
    with_fixtures(|librad_paths, repos_dir, _platinum_id| {
        let repo_dir = tempfile::tempdir_in(repos_dir.path()).expect("repo dir failed");
        let path = repo_dir.path().to_str().expect("repo path").to_string();
        coco::init_repo(path.clone()).expect("repo init failed");

        let (project_id, _project_meta) =
                    coco::init_project(
                        &librad_paths,
                        &path,
                        "upstream",
                        "Code collaboration without intermediates.",
                        "master",
                        "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
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
                            imgUrl
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

        execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "project": {
                        "id": id,
                        "metadata": {
                            "name": "upstream",
                            "description": "Code collaboration without intermediates.",
                            "defaultBranch": "master",
                            "imgUrl": "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                        },
                        "registered": None,
                    },
                })
            );
        });
    });
}

#[test]
fn identity() {
    common::with_fixtures(|librad_paths, _repo_dir, _platinum_id| {
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

        common::execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
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
                            "emoji": "🚡",
                            "background": {
                                "r": 24,
                                "g": 105,
                                "b": 216,
                            },
                        }
                    },
                })
            );
        });
    });
}

#[test]
fn user() {
    common::with_fixtures(|librad_paths, _repo_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("handle".into(), InputValue::scalar("cloudhead"));

        let query = "query($handle: ID!) {
            user(handle: $handle)
        }";

        common::execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(res, graphql_value!({ "user": None }));
        });
    });
}

// TODO(xla): Ressurect once we have figure out the project listing strategy.
// #[test]
// fn projects() {
//     with_fixtures(|librad_paths, _repos_dir, _platinum_id| {
//         let query = "{
//             projects {
//                 metadata {
//                     name
//                     description
//                     defaultBranch
//                     imgUrl
//                 }
//             }
//         }";

//         execute_query(librad_paths, query, &Variables::new(), |res, errors| {
//             assert_eq!(errors, []);
//             assert_eq!(
//                 res,
//                 graphql_value!({
//                     "projects": [
//                         {
//                             "metadata": {
//                                 "name": "Monadic",
//                                 "description": "Open source organization of amazing
// things.",                                 "defaultBranch": "stable",
//                                 "imgUrl": "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
//                             },
//                         },
//                         {
//                             "metadata": {
//                                 "name": "monokel",
//                                 "description": "A looking glass into the future",
//                                 "defaultBranch": "master",
//                                 "imgUrl": "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
//                             },
//                         },
//                         {
//                             "metadata": {
//                                 "name": "open source coin",
//                                 "description": "Research for the sustainability of the
// open source community.",                                 "defaultBranch":
// "master",                                 "imgUrl": "https://avatars0.githubusercontent.com/u/31632242",
//                             },
//                         },
//                         {
//                             "metadata": {
//                                 "name": "radicle",
//                                 "description": "Decentralized open source collaboration",
//                                 "defaultBranch": "dev",
//                                 "imgUrl": "https://avatars0.githubusercontent.com/u/48290027",
//                             },
//                         },
//                     ],
//                 })
//             );
//         });
//     });
// }
