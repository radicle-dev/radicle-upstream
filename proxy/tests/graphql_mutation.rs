#[macro_use]
extern crate juniper;

use indexmap::IndexMap;
use juniper::{InputValue, Variables};
use librad::surf::git::git2;

use proxy::coco;

mod common;
use common::with_fixtures;

#[test]
fn create_identity() {
    with_fixtures(|_librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("handle".into(), InputValue::scalar("cloudhead"));
        vars.insert("displayName".into(), InputValue::scalar("Alexis Sellier"));
        vars.insert(
            "avatarUrl".into(),
            InputValue::scalar("https://avatars1.githubusercontent.com/u/40774"),
        );

        let query = "mutation($handle: String!, $displayName: String, $avatarUrl: String) {
                createIdentity(handle: $handle, displayName: $displayName, avatarUrl: $avatarUrl) {
                    id
                    shareableEntityIdentifier
                    metadata {
                        handle
                        displayName
                        avatarUrl
                    }
                    registered
                }
            }";
        let res = graphql_value!({
            "createIdentity": {
                "id": "123abcd.git",
                "shareableEntityIdentifier": "cloudhead@123abcd.git",
                "metadata": {
                    "handle": "cloudhead",
                    "displayName": "Alexis Sellier",
                    "avatarUrl": "https://avatars1.githubusercontent.com/u/40774",
                },
                "registered": None,
            },
        });

        (query, vars, None, res)
    });
}

#[test]
fn create_identity_existing() {
    with_fixtures(|_librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("handle".into(), InputValue::scalar("cloudhead"));
        vars.insert("displayName".into(), InputValue::scalar("Alexis Sellier"));
        vars.insert(
            "avatarUrl".into(),
            InputValue::scalar("https://avatars1.githubusercontent.com/u/40774"),
        );

        let query = "mutation($handle: String!, $displayName: String, $avatarUrl: String) {
                createIdentity(handle: $handle, displayName: $displayName, avatarUrl: $avatarUrl) {
                    id
                }
            }";
        let res = graphql_value!({
            "error": "foo",
        });

        (query, vars, None, res)
    });
}

#[test]
fn create_project_existing_repo() {
    with_fixtures(|_librad_paths, repos_dir, _platinum_id| {
        let dir =
            tempfile::tempdir_in(repos_dir.path()).expect("creating temporary directory failed");
        let path = dir.path().to_str().expect("unable to get path");

        crate::coco::init_repo(path.to_string()).expect("unable to create repo");
        git2::Repository::init(path).expect("unable to create repo");

        let mut metadata_input: IndexMap<String, InputValue> = IndexMap::new();
        metadata_input.insert("name".into(), InputValue::scalar("upstream"));
        metadata_input.insert(
            "description".into(),
            InputValue::scalar("Code collaboration without intermediates."),
        );
        metadata_input.insert("defaultBranch".into(), InputValue::scalar("master"));

        let mut vars = Variables::new();
        vars.insert("metadata".into(), InputValue::object(metadata_input));
        vars.insert("path".into(), InputValue::scalar(path));
        vars.insert("publish".into(), InputValue::scalar(false));

        let query =
            "mutation($metadata: ProjectMetadataInput!, $path: String!, $publish: Boolean!) {
                createProject(metadata: $metadata, path: $path, publish: $publish) {
                    metadata {
                        name
                        description
                        defaultBranch
                    }
                }
            }";
        let res = graphql_value!({
            "createProject": {
                "metadata": {
                    "name": "upstream",
                    "description": "Code collaboration without intermediates.",
                    "defaultBranch": "master",
                },
            },
        });

        (query, vars, None, res)
    })
}

#[test]
fn create_project() {
    with_fixtures(|_librad_paths, repos_dir, _platinum_id| {
        let dir =
            tempfile::tempdir_in(repos_dir.path()).expect("creating temporary directory failed");
        let path = dir.path().to_str().expect("unable to get path");

        let mut metadata_input: IndexMap<String, InputValue> = IndexMap::new();

        metadata_input.insert("name".into(), InputValue::scalar("upstream"));
        metadata_input.insert(
            "description".into(),
            InputValue::scalar("Code collaboration without intermediates."),
        );
        metadata_input.insert("defaultBranch".into(), InputValue::scalar("master"));

        let mut vars = Variables::new();
        vars.insert("metadata".into(), InputValue::object(metadata_input));
        vars.insert("path".into(), InputValue::scalar(path));
        vars.insert("publish".into(), InputValue::scalar(false));

        let query =
            "mutation($metadata: ProjectMetadataInput!, $path: String!, $publish: Boolean!) {
                createProject(metadata: $metadata, path: $path, publish: $publish) {
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
                    stats {
                        branches
                        commits
                        contributors
                    }
                }
            }";
        let res = graphql_value!({
            "createProject": {
                "metadata": {
                    "name": "upstream",
                    "description": "Code collaboration without intermediates.",
                    "defaultBranch": "master",
                },
                "registered": None,
                "stats": {
                    "branches": 11,
                    "commits": 267,
                    "contributors": 8,
                },
            },
        });

        (query, vars, None, res)
    })
}

#[test]
fn register_project() {
    with_fixtures(|_librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("orgId".into(), InputValue::scalar("monadic"));
        vars.insert("projectName".into(), InputValue::scalar("upstream"));

        let query = "mutation($projectName: String!, $orgId: String!) {
                        registerProject(projectName: $projectName, orgId: $orgId) {
                            messages {
                                ... on ProjectRegistrationMessage {
                                    projectName,
                                    orgId
                                }
                            },
                        }
                    }";
        let res = graphql_value!({
            "registerProject": {
                "messages": [
                    { "projectName": "upstream", "orgId": "monadic" },
                ],
            },
        });
        (query, vars, None, res)
    });
}

#[test]
fn register_user() {
    with_fixtures(|_librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("handle".into(), InputValue::scalar("cloudhead"));
        vars.insert("id".into(), InputValue::scalar("123abcd.git"));

        let query = "mutation($handle: ID!, $id: ID!) {
                        registerUser(handle: $handle, id: $id) {
                            messages {
                                ... on UserRegistrationMessage {
                                    handle,
                                    id,
                                }
                            },
                        }
                    }";
        let res = graphql_value!({
            "registerUser": {
                "messages": [
                    { "handle": "cloudhead", "id": "123abcd.git" },
                ],
            },
        });

        (query, vars, None, res)
    });
}
