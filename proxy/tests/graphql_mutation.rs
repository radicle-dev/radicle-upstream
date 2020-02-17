#[macro_use]
extern crate juniper;

use indexmap::IndexMap;
use juniper::{InputValue, Variables};
use librad::surf::git::git2;
use pretty_assertions::assert_eq;

mod common;
use proxy::coco;

#[test]
fn create_project_existing_repo() {
    common::with_fixtures(|librad_paths, repos_dir, _platinum_id| {
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
        metadata_input.insert("imgUrl".into(), InputValue::scalar("https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png"));

        let mut vars = Variables::new();
        vars.insert("metadata".into(), InputValue::object(metadata_input));
        vars.insert("path".into(), InputValue::scalar(path));
        vars.insert("publish".into(), InputValue::scalar(false));

        let query = "mutation($metadata: MetadataInput!, $path: String!, $publish: Boolean!) {
                        createProject(metadata: $metadata, path: $path, publish: $publish) {
                            metadata {
                                name
                                description
                                defaultBranch
                                imgUrl
                            }
                        }
                    }";

        common::execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "createProject": {
                        "metadata": {
                            "name": "upstream",
                            "description": "Code collaboration without intermediates.",
                            "defaultBranch": "master",
                            "imgUrl": "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                        },
                    },
                })
            );
        });

        dir.close().expect("directory teardown failed");
    })
}

#[test]
fn create_project() {
    common::with_fixtures(|librad_paths, repos_dir, _platinum_id| {
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
        metadata_input.insert("imgUrl".into(), InputValue::scalar("https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png"));

        let mut vars = Variables::new();
        vars.insert("metadata".into(), InputValue::object(metadata_input));
        vars.insert("path".into(), InputValue::scalar(path));
        vars.insert("publish".into(), InputValue::scalar(false));

        let query = "mutation($metadata: MetadataInput!, $path: String!, $publish: Boolean!) {
                        createProject(metadata: $metadata, path: $path, publish: $publish) {
                            metadata {
                                name
                                description
                                defaultBranch
                                imgUrl
                            }
                        }
                    }";

        common::execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "createProject": {
                        "metadata": {
                            "name": "upstream",
                            "description": "Code collaboration without intermediates.",
                            "defaultBranch": "master",
                            "imgUrl": "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                        },
                    },
                })
            );
        });

        dir.close().expect("directory teardown failed");
    })
}

//TODO(nuno): test register_org

#[test]
fn register_project() {
    common::with_fixtures(|librad_paths, _repos_dir, _platinum_id| {
        let mut vars = Variables::new();
        vars.insert("org_id".into(), InputValue::scalar("monadic"));
        vars.insert("project_name".into(), InputValue::scalar("upstream"));

        let query = "mutation($project_name: String!, $org_id: String!) {
                        registerProject(project_name: $project_name, org_id: $org_id) {
                            messages {
                                ... on ProjectRegistration {
                                    project_name,
                                    org_id
                                }
                            },
                        }
                    }";
        common::execute_query(librad_paths, query, &vars, |res, errors| {
            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "registerProject": {
                        "messages": [
                            { "project_name": "upstream", "org_id": "monadic" },
                        ],
                    },
                })
            );
        });
    });
}
