use juniper::{FieldError, FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
use std::sync::Arc;

use librad::{git::GitProject, paths::Paths};
use radicle_surf::{
    file_system::{Path, SystemType},
    git::{git2, BranchName, GitBrowser, GitRepository, Sha1, TagName},
};

use crate::schema::error::Error;
use crate::source::{AccountId, Project, ProjectId, Source};

mod error;
mod git;

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
    /// Origin of data needed to server APIs.
    source: Arc<dyn Source + Send + Sync>,
}

impl Context {
    /// Returns a new `Context`.
    pub fn new<S>(dummy_repo_path: String, librad_paths: Paths, source: S) -> Self
    where
        S: Source + Send + Sync + 'static,
    {
        Self {
            dummy_repo_path,
            librad_paths,
            source: Arc::new(source),
        }
    }
}

impl juniper::Context for Context {}

juniper::graphql_scalar!(AccountId where Scalar = <S> {
    description: "AccountId"

    resolve(&self) -> Value {
        Value::scalar(hex::encode(self.0.as_ref() as &[u8]))
    }

    from_input_value(v: &InputValue) -> Option<AccountId> {
        let mut bytes = [0_u8; 32];

        v.as_scalar_value::<String>()
            .map(|s| hex::decode_to_slice(s, &mut bytes as &mut [u8]));

        Some(AccountId(radicle_registry_client::AccountId::from_raw(bytes)))
    }

    // Define how to parse a string value.
    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});

/// Input value used to communciate a `Registry` project id. (domain, name)
#[derive(GraphQLInputObject)]
struct IdInput {
    /// Domain part of the `Registry` namespace (e.g. "rad").
    domain: String,
    /// Name part of the `Registry` namespace (e.g. "upstream").
    name: String,
}

impl Into<ProjectId> for IdInput {
    fn into(self) -> ProjectId {
        ProjectId {
            name: self.name,
            domain: self.domain,
        }
    }
}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_project(ctx: &Context, path: String) -> Result<String, Error> {
        init_repo(path.clone())?;

        // TODO(xla): Have per environment paths configured to seperate the default store paths
        // from testing purposes to avoid polution.
        let paths = librad::paths::Paths::new()?;
        let key = librad::keys::device::Key::new();
        let profile = librad::meta::profile::UserProfile::new("xla");
        let sources = git2::Repository::open(std::path::Path::new(&path))?;

        let project_id = GitProject::init(&paths, &key, &profile, &sources)?;

        Ok(project_id.to_string())
    }

    fn register_project(
        ctx: &Context,
        name: String,
        description: String,
        img_url: String,
    ) -> FieldResult<Project> {
        Ok(ctx.source.register_project(name, description, img_url))
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

    fn projects(ctx: &Context) -> FieldResult<Vec<Project>> {
        Ok(ctx.source.get_all_projects())
    }

    fn project(ctx: &Context, id: IdInput) -> FieldResult<Option<Project>> {
        Ok(ctx.source.get_project(id.into()))
    }
}

fn init_repo(path: String) -> Result<(), Error> {
    let repo = git2::Repository::init(path)?;

    // First use the config to initialize a commit signature for the user.
    let sig = repo.signature()?;
    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // For our purposes, we'll leave the index empty for now.
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use juniper::Variables;
    use librad::paths::Paths;
    use radicle_registry_client::MemoryClient;

    use crate::schema::{Context, Mutation, Query, Schema};
    use crate::source::{setup_fixtures, Ledger};

    const REPO_PATH: &str = "../fixtures/git-platinum";

    fn execute_query(
        query: &str,
        vars: &Variables,
    ) -> (
        juniper::Value,
        Vec<juniper::ExecutionError<juniper::DefaultScalarValue>>,
    ) {
        let registry_client = MemoryClient::new();
        let mut source = Ledger::new(registry_client);

        setup_fixtures(&mut source);

        let librad_paths = {
            let dir = tempfile::tempdir().expect("creating temporary directory for paths failed");

            Paths::from_root(dir.path()).expect("unable to get librad paths")
        };

        let ctx = Context::new(REPO_PATH.into(), librad_paths, source);

        juniper::execute(query, None, &Schema::new(Query, Mutation), vars, &ctx)
            .expect("test execute failed")
    }

    mod mutation {
        use juniper::{InputValue, Variables};
        use pretty_assertions::assert_eq;

        use super::execute_query;

        #[test]
        fn create_project() {
            let dir = tempfile::tempdir().expect("creating temporary directory failed");
            let path = dir.path().to_str().expect("unable to get path");

            let mut vars = Variables::new();
            vars.insert("path".into(), InputValue::scalar(path));

            let (res, errors) = execute_query(
                "mutation($path: String!) { createProject(path: $path) }",
                &vars,
            );
            assert_eq!(errors, []);
            assert_ne!(res, graphql_value!(""));

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
            let (res, errors) = execute_query("query { apiVersion }", &Variables::new());
            assert_eq!(errors, []);
            assert_eq!(res, graphql_value!({ "apiVersion": "1.0" }));
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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $revision: String!, $path: String!) {
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
            }",
                &vars,
            );

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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $revision: String!, $path: String!) {
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
            }",
                &vars,
            );

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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $revision: String!, $path: String!) {
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
            }",
                &vars,
            );

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
        }

        #[test]
        fn branches() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));

            vars.insert("id".into(), InputValue::object(id_map));

            let (res, errors) = execute_query("query($id: IdInput!) { branches(id: $id) }", &vars);

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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $sha1: String!) {
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
            }",
                &vars,
            );

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
        }

        #[test]
        fn tags() {
            let mut vars = Variables::new();
            let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

            id_map.insert("domain".into(), InputValue::scalar("rad"));
            id_map.insert("name".into(), InputValue::scalar("upstream"));
            vars.insert("id".into(), InputValue::object(id_map));

            let (res, errors) = execute_query("query($id: IdInput!) { tags(id: $id) }", &vars);

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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $revision: String!, $prefix: String!) {
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
            }",
                &vars,
            );

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

            let (res, errors) = execute_query(
                "query($id: IdInput!, $revision: String!, $prefix: String!) {
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
            }",
                &vars,
            );

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
        }

        #[test]
        fn projects() {
            let (res, errors) = execute_query("query { projects { name } }", &Variables::new());

            assert_eq!(errors, []);
            assert_eq!(
                res,
                graphql_value!({
                    "projects": [
                        {"name": "monokel"},
                        {"name": "Monadic"},
                        {"name": "open source coin"},
                        {"name": "radicle"},
                    ]
                })
            );
        }
    }
}
