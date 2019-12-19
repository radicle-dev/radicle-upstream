use juniper::{FieldError, FieldResult, RootNode};

use librad::paths::Paths;
use radicle_surf::{
    file_system::{Path, SystemType},
    git::{BranchName, GitBrowser, GitRepository, Sha1, TagName},
};

mod error;
mod git;
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
    pub fn new(dummy_repo_path: String, librad_paths: Paths) -> Self {
        Self {
            dummy_repo_path,
            librad_paths,
        }
    }
}

impl juniper::Context for Context {}

/// Input value used to communciate a `Registry` project id. (domain, name)
#[derive(GraphQLInputObject)]
struct IdInput {
    /// Domain part of the `Registry` namespace (e.g. "rad").
    domain: String,
    /// Name part of the `Registry` namespace (e.g. "upstream").
    name: String,
}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_project(
        ctx: &Context,
        metadata: project::MetadataInput,
        path: String,
    ) -> Result<project::Project, Error> {
        git::init_repo(path.clone())?;
        let (id, meta) = git::init_project(
            &ctx.librad_paths,
            path,
            metadata.name,
            metadata.description,
            metadata.default_branch,
            metadata.img_url,
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

    fn blob(ctx: &Context, id: IdInput, revision: String, path: String) -> FieldResult<git::Blob> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");

        // Best effort to guess the revision.
        if let Err(err) = browser
            .branch(BranchName::new(&revision))
            .or(browser.commit(Sha1::new(&revision)))
            .or(browser.tag(TagName::new(&revision)))
        {
            let err_fmt = format!("{:?}", err);

            return Err(FieldError::new(
                "Git error occurred",
                graphql_value!({ "git": err_fmt }),
            ));
        };

        let root = browser
            .get_directory()
            .expect("unable to get root directory");

        let mut p = Path::root();
        p.append(&mut Path::from_string(&path));
        let file = root
            .find_file(&p)
            .expect(&format!("unable to find file: {} -> {}", path, p));
        let last_commit = browser
            .last_commit(&p)
            .expect(&format!("[blob] unable to get last commit: {}", p));
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
                last_commit: git::Commit::from(&last_commit),
            },
        })
    }

    fn commit(ctx: &Context, id: IdInput, sha1: String) -> FieldResult<git::Commit> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        browser
            .commit(radicle_surf::vcs::git::Sha1::new(&sha1))
            .expect("setting commit failed");

        let history = browser.get_history();
        let commit = history.0.first();

        Ok(git::Commit::from(commit))
    }

    fn branches(ctx: &Context, id: IdInput) -> FieldResult<Vec<git::Branch>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let mut branches: Vec<git::Branch> = browser
            .list_branches(None)
            .expect("Getting branches failed")
            .into_iter()
            .map(|b| git::Branch(b.name.name()))
            .collect();

        branches.sort();

        Ok(branches)
    }

    fn tags(ctx: &Context, id: IdInput) -> FieldResult<Vec<git::Tag>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let mut tag_names = browser.list_tags().expect("Getting branches failed");
        tag_names.sort();

        let mut tags: Vec<git::Tag> = tag_names
            .into_iter()
            .map(|tag_name| git::Tag(tag_name.name()))
            .collect();

        tags.sort();

        Ok(tags)
    }

    fn tree(
        ctx: &Context,
        id: IdInput,
        revision: String,
        prefix: String,
    ) -> FieldResult<git::Tree> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");

        if let Err(err) = browser
            .branch(BranchName::new(&revision))
            .or(browser.commit(Sha1::new(&revision)))
            .or(browser.tag(TagName::new(&revision)))
        {
            let err_fmt = format!("{:?}", err);

            return Err(FieldError::new(
                "Git error occurred",
                graphql_value!({ "git": err_fmt }),
            ));
        };

        let path = if prefix == "/" || prefix == "" {
            Path::root()
        } else {
            let mut root = Path::root();
            root.append(&mut Path::from_string(&prefix));
            root
        };

        let root_dir = browser
            .get_directory()
            .expect("getting repo directory failed");
        let prefix_dir = if path.is_root() {
            root_dir
        } else {
            root_dir.find_directory(&path).expect(&format!(
                "directory listing failed: {} -> {} | {:?}",
                path,
                path.is_root(),
                prefix,
            ))
        };
        let mut prefix_contents = prefix_dir.list_directory();
        prefix_contents.sort();

        let mut entries: Vec<git::TreeEntry> = prefix_contents
            .iter()
            .map(|(label, system_type)| {
                let entry_path = {
                    let mut path = path.clone();
                    path.push(label.clone());
                    path
                };
                let last_commit = git::Commit::from(&browser.last_commit(&entry_path).expect(
                    &format!("[tree] unable to get entry last commit: {}", entry_path),
                ));
                let info = git::Info {
                    name: label.to_string(),
                    object_type: match system_type {
                        SystemType::Directory => git::ObjectType::Tree,
                        SystemType::File => git::ObjectType::Blob,
                    },
                    last_commit,
                };

                let (_root, labels) = entry_path.split_first();
                let clean_path = Path(nonempty::NonEmpty::from_slice(labels).unwrap());

                git::TreeEntry {
                    info,
                    path: clean_path.to_string(),
                }
            })
            .collect();

        // We want to ensure that in the response Tree entries come first. `Ord` being derived on
        // the enum ensures Variant declaration order.
        //
        // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
        entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

        let last_commit = if path.is_root() {
            git::Commit::from(browser.get_history().0.first())
        } else {
            git::Commit::from(
                &browser
                    .last_commit(&path)
                    .unwrap_or_else(|| panic!("[tree] unable to get last commit: {}", path)),
            )
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

    fn projects(ctx: &Context) -> Result<Vec<project::Project>, Error> {
        let mut projects = librad::project::list_projects(&ctx.librad_paths)
            .map(|id| {
                let project_meta = librad::project::show_project(&ctx.librad_paths, &id).unwrap();

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
    use juniper::Variables;
    use librad::paths::Paths;

    use crate::schema::{Context, Mutation, Query, Schema};

    const REPO_PATH: &str = "../fixtures/git-platinum";

    fn execute_query<F>(query: &str, vars: &Variables, f: F) -> ()
    where
        F: FnOnce(juniper::Value, Vec<juniper::ExecutionError<juniper::DefaultScalarValue>>) -> (),
    {
        let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
        let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");

        let infos = vec![
            (
                "monokel",
                "A looking glass into the future",
                "master",
                "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
            ),
            (
                "Monadic",
                "Open source organization of amazing things.",
                "stable",
                "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
            ),
            (
                "open source coin",
                "Research for the sustainability of the open source community.",
                "master",
                "https://avatars0.githubusercontent.com/u/31632242",
            ),
            (
                "radicle",
                "Decentralized open source collaboration",
                "dev",
                "https://avatars0.githubusercontent.com/u/48290027",
            ),
        ];

        for info in infos {
            let repos_dir =
                tempfile::tempdir_in(tmp_dir.path()).expect("unable to create repos directory");
            let repo_dir =
                tempfile::tempdir_in(repos_dir.path()).expect("unable to create repo directory");
            let path = repo_dir.path().to_str().expect("repo dir path").to_string();

            crate::schema::git::init_repo(path.clone()).expect("repo init failed");
            crate::schema::git::init_project(
                &librad_paths,
                path,
                info.0.to_string(),
                info.1.to_string(),
                info.2.to_string(),
                info.3.to_string(),
            )
            .expect("project init failed");
        }

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

        use super::execute_query;

        #[test]
        fn create_project() {
            let dir = tempfile::tempdir().expect("creating temporary directory failed");
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

            let query = "mutation($metadata: MetadataInput!, $path: String!) {
                    createProject(metadata: $metadata, path: $path) {
                        metadata {
                            name
                            description
                            defaultBranch
                            imgUrl
                        }
                    }
                }";

            execute_query(query, &vars, |res, errors| {
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
        }
    }

    mod query {
        use indexmap::IndexMap;
        use juniper::{InputValue, Variables};
        use pretty_assertions::assert_eq;

        use super::execute_query;

        #[test]
        fn api_version() {
            let query = "query { apiVersion }";

            execute_query(query, &Variables::new(), |res, errors| {
                assert_eq!(errors, []);
                assert_eq!(res, graphql_value!({ "apiVersion": "1.0" }));
            });
        }

        #[test]
        fn blob() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));

            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("revision".into(), InputValue::scalar("master"));
            vars.insert("path".into(), InputValue::scalar("text/arrows.txt"));

            let query = "query($id: IdInput!, $revision: String!, $path: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
                                    "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                    "author": {
                                        "name": "Alexander Simmerl",
                                        "email": "a.simmerl@gmail.com",
                                    },
                                    "summary": "Add a long commit message to commit message body (#1)",
                                    "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                    "committerTime": "1576170713",
                                },
                            },
                        }
                    }),
                );
            });
        }

        #[test]
        fn blob_binary() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));

            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("revision".into(), InputValue::scalar("master"));
            vars.insert("path".into(), InputValue::scalar("bin/ls"));

            let query = "query($id: IdInput!, $revision: String!, $path: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
                                    "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                    "author": {
                                        "name": "Alexander Simmerl",
                                        "email": "a.simmerl@gmail.com",
                                    },
                                    "summary": "Add a long commit message to commit message body (#1)",
                                    "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                    "committerTime": "1576170713",
                                },
                            },
                        }
                    }),
                );
            });
        }

        #[test]
        fn blob_in_root() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));

            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("revision".into(), InputValue::scalar("master"));
            vars.insert("path".into(), InputValue::scalar("README.md"));

            let query = "query($id: IdInput!, $revision: String!, $path: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
                                    "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                    "author": {
                                        "name": "Alexander Simmerl",
                                        "email": "a.simmerl@gmail.com",
                                    },
                                    "summary": "Add a long commit message to commit message body (#1)",
                                    "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                    "committerTime": "1576170713",
                                },
                            },
                        }
                    }),
                );
            });
        }

        #[test]
        fn branches() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));

            vars.insert("id".into(), InputValue::object(id_map));

            let query = "query($id: IdInput!) { branches(id: $id) }";

            execute_query(query, &vars, |res, errors| {
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
        }

        #[test]
        fn commit() {
            const SHA1: &str = "80ded66281a4de2889cc07293a8f10947c6d57fe";

            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));
            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("sha1".into(), InputValue::scalar(SHA1));

            let query = "query($id: IdInput!, $sha1: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
        }

        #[test]
        fn tags() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));
            vars.insert("id".into(), InputValue::object(id_map));

            let query = "query($id: IdInput!) { tags(id: $id) }";

            execute_query(query, &vars, |res, errors| {
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
        }

        #[test]
        fn tree() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));
            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("revision".into(), InputValue::scalar("master"));
            vars.insert("prefix".into(), InputValue::scalar("src"));

            let query = "query($id: IdInput!, $revision: String!, $prefix: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
                                    "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                    "author": {
                                        "name": "Alexander Simmerl",
                                        "email": "a.simmerl@gmail.com",
                                    },
                                    "summary": "Add a long commit message to commit message body (#1)",
                                    "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                    "committerTime": "1576170713",
                                },
                            },
                            "entries": [
                                {
                                    "path": "src/Eval.hs",
                                    "info": {
                                        "name": "Eval.hs",
                                        "objectType": "BLOB",
                                        "lastCommit": {
                                            "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                            "author": {
                                                "name": "Alexander Simmerl",
                                                "email": "a.simmerl@gmail.com",
                                            },
                                            "summary": "Add a long commit message to commit message body (#1)",
                                            "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                            "committerTime": "1576170713",
                                        },
                                    },
                                },
                                {
                                    "path": "src/Folder.svelte",
                                    "info": {
                                        "name": "Folder.svelte",
                                        "objectType": "BLOB",
                                        "lastCommit": {
                                            "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                            "author": {
                                                "name": "Alexander Simmerl",
                                                "email": "a.simmerl@gmail.com",
                                            },
                                            "summary": "Add a long commit message to commit message body (#1)",
                                            "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                            "committerTime": "1576170713",
                                        },
                                    },
                                },
                                {
                                    "path": "src/memory.rs",
                                    "info": {
                                        "name": "memory.rs",
                                        "objectType": "BLOB",
                                        "lastCommit": {
                                            "sha1": "d6880352fc7fda8f521ae9b7357668b17bb5bad5",
                                            "author": {
                                                "name": "Alexander Simmerl",
                                                "email": "a.simmerl@gmail.com",
                                            },
                                            "summary": "Add a long commit message to commit message body (#1)",
                                            "message": "Add a long commit message to commit message body (#1)\n\nIn order to test the correct delivery of the message part of the commit\r\nwe add this commit which has both by expanding beyond the summary.",
                                            "committerTime": "1576170713",
                                        },
                                    },
                                },
                            ],
                        }
                    }),
                );
            });
        }

        #[test]
        fn tree_root() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));
            vars.insert("id".into(), InputValue::object(id_map));
            vars.insert("revision".into(), InputValue::scalar("master"));
            vars.insert("prefix".into(), InputValue::scalar(""));

            let query = "query($id: IdInput!, $revision: String!, $prefix: String!) {
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

            execute_query(query, &vars, |res, errors| {
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
        }

        #[test]
        fn projects() {
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

            execute_query(query, &Variables::new(), |res, errors| {
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
        }
    }
}
