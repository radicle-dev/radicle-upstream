use juniper::{FieldResult, RootNode, ID};
use std::str::FromStr;

use librad::paths::Paths;
use radicle_surf as surf;

/// Error definitions and type casting logic.
mod error;
pub mod git;
mod project;

use crate::schema::error::Error;

/// Glue to bundle our read and write APIs together.
pub type Schema = RootNode<'static, Query, Mutation>;

/// Returns a `Schema` with the default parameterised `Query` and `Mutation`.
pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

/// Container for data access from handlers.
#[derive(Clone)]
pub struct Context {
    /// Intermediate repo used to serve dummy data to be presented to the API consumer.
    dummy_repo_path: String,
    /// Root on the filesystem for the librad config and storage paths.
    librad_paths: Paths,
}

impl Context {
    /// Returns a new `Context`.
    pub const fn new(dummy_repo_path: String, librad_paths: Paths) -> Self {
        Self {
            dummy_repo_path,
            librad_paths,
        }
    }
}

impl juniper::Context for Context {}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_project(
        ctx: &Context,
        metadata: project::MetadataInput,
        path: String,
        publish: bool,
    ) -> Result<project::Project, Error> {
        if surf::git::git2::Repository::open(path.clone()).is_err() {
            git::init_repo(path.clone())?;
        };

        let (id, meta) = git::init_project(
            &ctx.librad_paths,
            &path,
            &metadata.name,
            &metadata.description,
            &metadata.default_branch,
            &metadata.img_url,
        )?;

        Ok(project::Project {
            id: id.to_string().into(),
            metadata: meta.into(),
        })
    }
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn blob(ctx: &Context, id: ID, revision: String, path: String) -> Result<git::Blob, Error> {
        let repo =
            surf::git::Repository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser =
            surf::git::Browser::new(repo).expect("setting up browser for repo failed");

        // Best effort to guess the revision.
        if let Err(err) = browser
            .branch(surf::git::BranchName::new(&revision))
            .or(browser.commit(surf::git::Sha1::new(&revision)))
            .or(browser.tag(surf::git::TagName::new(&revision)))
        {
            let err_fmt = format!("{:?}", err);

            return Err(Error::Git(surf::git::error::Error::NotBranch));
        };

        let root = browser
            .get_directory()
            .expect("unable to get root directory");

        let mut p = surf::file_system::Path::from_str(&path)?;

        let file = root
            .find_file(&p)
            .unwrap_or_else(|| panic!("unable to find file: {} -> {}", path, p));

        let mut commit_path = surf::file_system::Path::root();
        commit_path.append(&mut p);

        let last_commit = browser
            .last_commit(&commit_path)?
            .map(|c| git::Commit::from(&c));
        let (_rest, last) = p.split_last();
        let (binary, content) = {
            let res = std::str::from_utf8(&file.contents);

            match res {
                Ok(content) => (false, Some(content.to_string())),
                Err(_) => (true, None),
            }
        };

        Ok(git::Blob {
            binary,
            content,
            info: git::Info {
                name: last.label,
                object_type: git::ObjectType::Blob,
                last_commit,
            },
        })
    }

    fn commit(ctx: &Context, id: ID, sha1: String) -> FieldResult<git::Commit> {
        let repo =
            surf::git::Repository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser =
            surf::git::Browser::new(repo).expect("setting up browser for repo failed");
        browser
            .commit(radicle_surf::vcs::git::Sha1::new(&sha1))
            .expect("setting commit failed");

        let history = browser.get_history();
        let commit = history.0.first();

        Ok(git::Commit::from(commit))
    }

    fn branches(ctx: &Context, id: ID) -> Result<Vec<git::Branch>, Error> {
        git::branches(&ctx.dummy_repo_path)
    }

    fn local_branches(ctx: &Context, path: String) -> Result<Vec<git::Branch>, Error> {
        git::branches(&path)
    }

    fn tags(ctx: &Context, id: ID) -> FieldResult<Vec<git::Tag>> {
        let repo =
            surf::git::Repository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = surf::git::Browser::new(repo).expect("setting up browser for repo failed");
        let mut tag_names = browser.list_tags().expect("Getting branches failed");
        tag_names.sort();

        let mut tags: Vec<git::Tag> = tag_names
            .into_iter()
            .map(|tag_name| git::Tag(tag_name.name()))
            .collect();

        tags.sort();

        Ok(tags)
    }

    fn tree(ctx: &Context, id: ID, revision: String, prefix: String) -> Result<git::Tree, Error> {
        let repo = surf::git::Repository::new(&ctx.dummy_repo_path)?;
        let mut browser = surf::git::Browser::new(repo)?;

        if let Err(err) = browser
            .branch(surf::git::BranchName::new(&revision))
            .or(browser.commit(surf::git::Sha1::new(&revision)))
            .or(browser.tag(surf::git::TagName::new(&revision)))
        {
            let err_fmt = format!("{:?}", err);

            return Err(Error::Git(surf::git::error::Error::NotBranch));
        };

        let mut path = if prefix == "/" || prefix == "" {
            surf::file_system::Path::root()
        } else {
            surf::file_system::Path::from_str(&prefix)?
        };

        let root_dir = browser
            .get_directory()
            .expect("getting repo directory failed");
        let prefix_dir = if path.is_root() {
            root_dir
        } else {
            root_dir.find_directory(&path).unwrap_or_else(|| {
                panic!(
                    "directory listing failed: {} -> {} | {:?}",
                    path,
                    path.is_root(),
                    prefix,
                )
            })
        };
        let mut prefix_contents = prefix_dir.list_directory();
        prefix_contents.sort();

        let mut entries: Vec<git::TreeEntry> = prefix_contents
            .iter()
            .map(|(label, system_type)| {
                let mut entry_path = if path.is_root() {
                    surf::file_system::Path(
                        nonempty::NonEmpty::from_slice(&[label.clone()])
                            .expect("unable to create label slice"),
                    )
                } else {
                    let mut p = path.clone();
                    p.push(label.clone());
                    p
                };
                let mut commit_path = surf::file_system::Path::root();
                commit_path.append(&mut entry_path);

                let last_commit = browser
                    .last_commit(&commit_path)
                    .expect("last commit for file failed")
                    .map(|c| git::Commit::from(&c));
                let info = git::Info {
                    name: label.to_string(),
                    object_type: match system_type {
                        surf::file_system::SystemType::Directory => git::ObjectType::Tree,
                        surf::file_system::SystemType::File => git::ObjectType::Blob,
                    },
                    last_commit,
                };

                git::TreeEntry {
                    info,
                    path: entry_path.to_string(),
                }
            })
            .collect();

        // We want to ensure that in the response Tree entries come first. `Ord` being derived on
        // the enum ensures Variant declaration order.
        //
        // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
        entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

        let last_commit = if path.is_root() {
            Some(git::Commit::from(browser.get_history().0.first()))
        } else {
            let mut commit_path = surf::file_system::Path::root();
            commit_path.append(&mut path);

            browser
                .last_commit(&commit_path)?
                .map(|c| git::Commit::from(&c))
        };
        let name = if path.is_root() {
            "".into()
        } else {
            let (_first, last) = path.split_last();
            last.label
        };
        let info = git::Info {
            name,
            object_type: git::ObjectType::Tree,
            last_commit,
        };

        Ok(git::Tree {
            path: prefix,
            entries,
            info,
        })
    }

    fn project(ctx: &Context, id: ID) -> Result<project::Project, Error> {
        use std::str::FromStr;
        let project_id = librad::project::ProjectId::from_str(&id.to_string())?;
        let meta = librad::project::Project::show(&ctx.librad_paths, &project_id)?;

        Ok(project::Project {
            id,
            metadata: meta.into(),
        })
    }

    fn projects(ctx: &Context) -> Result<Vec<project::Project>, Error> {
        let mut projects = librad::project::Project::list(&ctx.librad_paths)
            .map(|id| {
                let project_meta = librad::project::Project::show(&ctx.librad_paths, &id)
                    .expect("unable to get project meta");

                project::Project {
                    id: id.to_string().into(),
                    metadata: project_meta.into(),
                }
            })
            .collect::<Vec<project::Project>>();

        projects.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));

        Ok(projects)
    }
}

#[cfg(test)]
mod tests {
    use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
    use librad::paths::Paths;
    use tempfile::{tempdir_in, TempDir};

    use crate::schema::{Context, Mutation, Query, Schema};

    const REPO_PATH: &str = "../fixtures/git-platinum";

    fn with_fixtures<F>(f: F)
    where
        F: FnOnce(Paths, TempDir) -> (),
    {
        let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
        let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");
        let repos_dir = tempdir_in(tmp_dir.path()).expect("unable to create repos directory");

        crate::schema::git::setup_fixtures(
            &librad_paths,
            tmp_dir.path().to_str().expect("path extraction failed"),
        )
        .expect("fixture setup failed");

        f(librad_paths, repos_dir)
    }

    fn execute_query<F>(librad_paths: Paths, query: &str, vars: &Variables, f: F)
    where
        F: FnOnce(Value, Vec<ExecutionError<DefaultScalarValue>>) -> (),
    {
        let ctx = Context::new(REPO_PATH.into(), librad_paths);
        let (res, errors) =
            juniper::execute(query, None, &Schema::new(Query, Mutation), vars, &ctx)
                .expect("test execute failed");

        f(res, errors);
    }

    mod mutation {
        use indexmap::IndexMap;
        use juniper::{InputValue, Variables};
        use pretty_assertions::assert_eq;
        use radicle_surf::git::git2;

        use super::{execute_query, with_fixtures};

        #[test]
        fn create_project_existing_repo() {
            with_fixtures(|librad_paths, repos_dir| {
                let dir = tempfile::tempdir_in(repos_dir.path())
                    .expect("creating temporary directory failed");
                let path = dir.path().to_str().expect("unable to get path");

                crate::schema::git::init_repo(path.to_string()).expect("unable to create repo");
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

                let query =
                    "mutation($metadata: MetadataInput!, $path: String!, $publish: Boolean!) {
                        createProject(metadata: $metadata, path: $path, publish: $publish) {
                            metadata {
                                name
                                description
                                defaultBranch
                                imgUrl
                            }
                        }
                    }";

                execute_query(librad_paths, query, &vars, |res, errors| {
                    assert_eq!(errors, []);
                    assert_ne!(
                        res,
                        graphql_value!({
                            "metadata": {
                                "name": "upstream",
                                "description": "Code collaboration without intermediates.",
                                "default_branch": "master",
                                "img_url": "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                            }
                        })
                    );
                });

                dir.close().expect("directory teardown failed");
            })
        }

        #[test]
        fn create_project() {
            with_fixtures(|librad_paths, repos_dir| {
                let dir = tempfile::tempdir_in(repos_dir.path())
                    .expect("creating temporary directory failed");
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

                let query =
                    "mutation($metadata: MetadataInput!, $path: String!, $publish: Boolean!) {
                        createProject(metadata: $metadata, path: $path, publish: $publish) {
                            metadata {
                                name
                                description
                                defaultBranch
                                imgUrl
                            }
                        }
                    }";

                execute_query(librad_paths, query, &vars, |res, errors| {
                    assert_eq!(errors, []);
                    assert_ne!(
                        res,
                        graphql_value!({
                            "metadata": {
                                "name": "upstream",
                                "description": "Code collaboration without intermediates.",
                                "default_branch": "master",
                                "img_url": "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                            }
                        })
                    );
                });

                dir.close().expect("directory teardown failed");
            })
        }
    }

    mod query {
        use juniper::{InputValue, Variables};
        use pretty_assertions::assert_eq;

        use crate::schema::git;

        use super::{execute_query, with_fixtures};

        #[test]
        fn api_version() {
            with_fixtures(|librad_paths, _repos_dir| {
                let query = "query { apiVersion }";

                execute_query(librad_paths, query, &Variables::new(), |res, errors| {
                    assert_eq!(errors, []);
                    assert_eq!(res, graphql_value!({ "apiVersion": "1.0" }));
                });
            });
        }

        #[test]
        fn blob() {
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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
                                "content": "This repository is a data source for the Upstream front-end tests.\n",
                                "info": {
                                    "name": "README.md",
                                    "objectType": "BLOB",
                                    "lastCommit": {
                                        "sha1": "d3464e33d75c75c99bfb90fa2e9d16efc0b7d0e3",
                                        "author": {
                                            "name": "Rūdolfs Ošiņš",
                                            "email": "rudolfs@osins.org",
                                        },
                                        "summary": "Initial commit FTW!",
                                        "message": "Initial commit FTW!\n",
                                        "committerTime": "1575282266",
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
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();
                vars.insert("id".into(), InputValue::scalar("git-platinum"));

                let query = "query($id: ID!) { branches(id: $id) }";

                execute_query(librad_paths, query, &vars, |res, errors| {
                    assert_eq!(errors, []);
                    assert_eq!(
                        res,
                        graphql_value!({
                            "branches": [
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
        fn local_branches() {
            with_fixtures(|librad_paths, _repos_dir| {
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
            with_fixtures(|librad_paths, _repos_dir| {
                const SHA1: &str = "80ded66281a4de2889cc07293a8f10947c6d57fe";

                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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
                                    "name": "R\u{16b}dolfs O\u{161}i\u{146}\u{161}",
                                    "email": "rudolfs@osins.org",
                                },
                                "summary": "Delete unneeded file",
                                "message": "Delete unneeded file\n",
                                "committerTime": "1575468397",
                            },
                        }),
                    )
                });
            });
        }

        #[test]
        fn tags() {
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();
                vars.insert("id".into(), InputValue::scalar("git-platinum"));

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
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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
                                        "sha1": "3873745c8f6ffb45c990eb23b491d4b4b6182f95",
                                        "author": {
                                            "name": "Fintan Halpenny",
                                            "email": "fintan.halpenny@gmail.com",
                                        },
                                        "summary": "Extend the docs (#2)",
                                        "message": "Extend the docs (#2)\n\nI want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                                        "committerTime": "1578309972",
                                    },
                                },
                                "entries": [
                                    {
                                        "path": "src/Eval.hs",
                                        "info": {
                                            "name": "Eval.hs",
                                            "objectType": "BLOB",
                                            "lastCommit": {
                                                "sha1": "3873745c8f6ffb45c990eb23b491d4b4b6182f95",
                                                "author": {
                                                    "name": "Fintan Halpenny",
                                                    "email": "fintan.halpenny@gmail.com",
                                                },
                                                "summary": "Extend the docs (#2)",
                                                "message": "Extend the docs (#2)\n\nI want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                                                "committerTime": "1578309972",
                                            },
                                        },
                                    },
                                    {
                                        "path": "src/Folder.svelte",
                                        "info": {
                                            "name": "Folder.svelte",
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
            with_fixtures(|librad_paths, _repos_dir| {
                let mut vars = Variables::new();

                vars.insert("id".into(), InputValue::scalar("git-platinum"));
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

        #[test]
        fn project() {
            with_fixtures(|librad_paths, repos_dir| {
                let repo_dir = tempfile::tempdir_in(repos_dir.path()).expect("repo dir failed");
                let path = repo_dir.path().to_str().expect("repo path").to_string();
                git::init_repo(path.clone()).expect("repo init failed");

                let (project_id, _project_meta) =
                    git::init_project(
                        &librad_paths,
                        &path,
                        "upstream",
                        "Code collaboration without intermediates.",
                        "master",
                        "https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/app/public/icon.png",
                    )
                    .expect("project init failed");

                let mut vars = Variables::new();
                vars.insert("id".into(), InputValue::scalar(project_id.to_string()));

                let query = "query($id: ID!) {
                    project(id: $id) {
                        metadata {
                            name
                            description
                            defaultBranch
                            imgUrl
                        }
                    }
                }";

                execute_query(librad_paths, query, &vars, |res, errors| {
                    assert_eq!(errors, []);
                    assert_eq!(
                        res,
                        graphql_value!({
                            "project": {
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
            });
        }

        #[test]
        fn projects() {
            with_fixtures(|librad_paths, _repos_dir| {
                let query = "{
                    projects {
                        metadata {
                            name
                            description
                            defaultBranch
                            imgUrl
                        }
                    }
                }";

                execute_query(librad_paths, query, &Variables::new(), |res, errors| {
                    assert_eq!(errors, []);
                    assert_eq!(
                        res,
                        graphql_value!({
                            "projects": [
                                {
                                    "metadata": {
                                        "name": "Monadic",
                                        "description": "Open source organization of amazing things.",
                                        "defaultBranch": "stable",
                                        "imgUrl": "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
                                    },
                                },
                                {
                                    "metadata": {
                                        "name": "monokel",
                                        "description": "A looking glass into the future",
                                        "defaultBranch": "master",
                                        "imgUrl": "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
                                    },
                                },
                                {
                                    "metadata": {
                                        "name": "open source coin",
                                        "description": "Research for the sustainability of the open source community.",
                                        "defaultBranch": "master",
                                        "imgUrl": "https://avatars0.githubusercontent.com/u/31632242",
                                    },
                                },
                                {
                                    "metadata": {
                                        "name": "radicle",
                                        "description": "Decentralized open source collaboration",
                                        "defaultBranch": "dev",
                                        "imgUrl": "https://avatars0.githubusercontent.com/u/48290027",
                                    },
                                },
                            ],
                        })
                    );
                });
            });
        }
    }
}
