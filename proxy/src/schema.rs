use juniper::{FieldError, FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
use std::sync::Arc;

use radicle_surf::{
    file_system::{Path, SystemType},
    git::{git2, BranchName, GitBrowser, GitRepository, Sha1, TagName},
};

use crate::source::{AccountId, Project, ProjectId, Source};

/// Glue to bundle our read and write APIs together.
pub type Schema = RootNode<'static, Query, Mutation>;

/// Returns a `Schema` with the default parameterised `Query` and `Mutation`.
pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

/// Container to pass the `Source` around for data access.
#[derive(Clone)]
pub struct Context {
    /// Intermediate repo used to serve dummy data to be presented to the API consumer.
    dummy_repo_path: String,
    /// Origin of data needed to server APIs.
    source: Arc<dyn Source + Send + Sync>,
}

impl Context {
    /// Returns a new `Context`.
    pub fn new<S: Source + Send + Sync + 'static>(dummy_repo_path: String, source: S) -> Self {
        Self {
            dummy_repo_path,
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

#[derive(GraphQLInputObject)]
struct IdInput {
    domain: String,
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

#[derive(GraphQLObject)]
struct Branch {
    name: String,
}

#[derive(GraphQLObject)]
struct Tag {
    name: String,
}

#[derive(GraphQLObject)]
struct Person {
    name: String,
    email: String,
}

#[derive(GraphQLObject)]
struct Commit {
    sha1: String,
    author: Person,
    summary: String,
    message: String,
    time: String,
}

// FIXME(xla): This should be a `std::convert::TryFrom` and needs to be addressed together with
//             consistent error handling.
impl From<&git2::Commit<'_>> for Commit {
    fn from(commit: &git2::Commit) -> Self {
        let signature = commit.author();

        Self {
            sha1: commit.id().to_string(),
            author: Person {
                name: signature.name().unwrap_or("invalid name").into(),
                email: signature.email().unwrap_or("invalid email").into(),
            },
            summary: commit.summary().unwrap_or("invalid subject").into(),
            message: commit.message().unwrap_or("invalid message").into(),
            time: commit.time().seconds().to_string(),
        }
    }
}

#[derive(GraphQLEnum)]
enum ObjectType {
    Blob,
    Tree,
}

#[derive(GraphQLObject)]
struct Info {
    name: String,
    object_type: ObjectType,
    last_commit: Commit,
}

#[derive(GraphQLObject)]
struct Tree {
    path: String,
    entries: Vec<TreeEntry>,
    info: Info,
}

#[derive(GraphQLObject)]
struct TreeEntry {
    info: Info,
    path: String,
}

#[derive(GraphQLObject)]
struct Blob {
    content: String,
    info: Info,
}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
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

    fn blob(ctx: &Context, id: IdInput, revision: String, path: String) -> FieldResult<Blob> {
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
        let file = root.find_file(&p).expect("unable to find file");
        let last_commit = browser.last_commit(&p).expect("unable to get last commit");
        let (_rest, last) = p.split_last();

        Ok(Blob {
            content: std::str::from_utf8(&file.contents)
                .expect("invalid content")
                .to_string(),
            info: Info {
                name: last.label,
                object_type: ObjectType::Blob,
                last_commit: Commit::from(&last_commit),
            },
        })
    }

    fn commit(ctx: &Context, id: IdInput, sha1: String) -> FieldResult<Commit> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        browser
            .commit(radicle_surf::vcs::git::Sha1::new(&sha1))
            .expect("setting commit failed");

        let history = browser.get_history();
        let commit = history.0.first();

        Ok(Commit::from(commit))
    }

    fn branches(ctx: &Context, id: IdInput) -> FieldResult<Vec<Branch>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let branches = browser
            .list_branches(None)
            .expect("Getting branches failed")
            .into_iter()
            .map(|b| Branch {
                name: b.name.name(),
            })
            .collect();

        Ok(branches)
    }

    fn tags(ctx: &Context, id: IdInput) -> FieldResult<Vec<Tag>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let tag_names = browser.list_tags().expect("Getting branches failed");
        let tags = tag_names
            .into_iter()
            .map(|tag_name| Tag {
                name: tag_name.name(),
            })
            .collect();

        Ok(tags)
    }

    fn tree(ctx: &Context, id: IdInput, revision: String, prefix: String) -> FieldResult<Tree> {
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

        let prefix_path = std::path::Path::new(&prefix);
        let path_components: Vec<radicle_surf::file_system::Label> = prefix_path
            .iter()
            .filter_map(|c| c.to_str())
            .map(|c| c.into())
            .collect();
        let root_dir = browser
            .get_directory()
            .expect("getting repo directory failed");
        let prefix_dir = root_dir
            .find_directory(&Path::with_root(&path_components))
            .expect("directory listing failed");
        let mut prefix_contents = prefix_dir.list_directory();
        prefix_contents.sort();

        let entries = prefix_contents
            .iter()
            .map(|(label, system_type)| {
                let path = prefix_path
                    .join(label.to_string())
                    .to_str()
                    .expect("invalid path")
                    .to_string();
                let last_commit = Commit::from(
                    &browser
                        .last_commit(&radicle_surf::file_system::Path::from_string(&path))
                        .expect("unable to get last commit"),
                );
                let info = Info {
                    name: label.to_string(),
                    object_type: match system_type {
                        SystemType::Directory => ObjectType::Tree,
                        SystemType::File => ObjectType::Blob,
                    },
                    last_commit,
                };

                TreeEntry { info, path }
            })
            .collect();

        let last_commit = Commit::from(
            &browser
                .last_commit(&Path::with_root(&[prefix.clone().into()]))
                .expect("unable to get last commit"),
        );
        let info = Info {
            name: prefix_path
                .file_name()
                .expect("invalid filename")
                .to_str()
                .expect("invalid string")
                .to_string(),
            object_type: ObjectType::Tree,
            last_commit,
        };

        Ok(Tree {
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

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use juniper::{InputValue, Variables};
    use pretty_assertions::assert_eq;
    use radicle_registry_client::MemoryClient;

    use super::{Context, Mutation, Query, Schema};
    use crate::source::{setup_fixtures, Ledger};

    const REPO_PATH: &str = "../data/git-golden";

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

        let ctx = Context::new(REPO_PATH.into(), source);

        juniper::execute(query, None, &Schema::new(Query, Mutation), &vars, &ctx)
            .expect("test execute failed")
    }

    #[test]
    fn query_blob() {
        let mut vars = Variables::new();
        let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

        id_map.insert("domain".into(), InputValue::scalar("rad"));
        id_map.insert("name".into(), InputValue::scalar("upstream"));

        vars.insert("id".into(), InputValue::object(id_map));
        vars.insert("revision".into(), InputValue::scalar("master"));
        vars.insert("path".into(), InputValue::scalar("src/main.rs"));

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
                            time,
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
                    "content": "fn main() {\n    println!(\"Hello, world!\");\n}\n",
                    "info": {
                        "name": "main.rs",
                        "objectType": "BLOB",
                        "lastCommit": {
                            "sha1": "74ba370ee5643f310873fb288af1c99d639da8ca",
                            "author": {
                                "name": "Fintan Halpenny",
                                "email": "fintan.halpenny@gmail.com",
                            },
                            "summary": "Add .gitignore",
                            "message": "Add .gitignore\n",
                            "time": "1574786128",
                        }
                    },
                }
            }),
        );
    }

    #[test]
    fn query_branches() {
        let mut vars = Variables::new();
        let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

        id_map.insert("domain".into(), InputValue::scalar("rad"));
        id_map.insert("name".into(), InputValue::scalar("upstream"));

        vars.insert("id".into(), InputValue::object(id_map));

        let (res, errors) =
            execute_query("query($id: IdInput!) { branches(id: $id) { name } }", &vars);

        assert_eq!(errors, []);
        assert_eq!(
            res,
            graphql_value!({
                "branches": [
                    { "name": "master" },
                    { "name": "origin/HEAD" },
                    { "name": "origin/master" },
                    { "name": "origin/add-tests" },
                ]
            }),
        );
    }

    #[test]
    fn query_commit() {
        const SHA1: &str = "b9053ab4916e8ba50a78c243c3440ebdd7500263";

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
                    time,
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
                        "name": "Alexander Simmerl",
                        "email": "a.simmerl@gmail.com",
                    },
                    "summary": "Add a long commit message for integration testing",
                    "message": "Add a long commit message for integration testing\n\nIn order to have a long commit message with distinct summary this commit\nis added to the repo to integrate against.\n",
                    "time": "1575988508",
                },
            }),
        )
    }

    #[test]
    fn query_tags() {
        let mut vars = Variables::new();
        let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

        id_map.insert("domain".into(), InputValue::scalar("rad"));
        id_map.insert("name".into(), InputValue::scalar("upstream"));
        vars.insert("id".into(), InputValue::object(id_map));

        let (res, errors) = execute_query("query($id: IdInput!) { tags(id: $id) { name } }", &vars);

        assert_eq!(errors, []);
        assert_eq!(
            res,
            graphql_value!({
                "tags": [
                    { "name": "v0.0.1" },
                ]
            }),
        )
    }

    #[test]
    fn query_tree() {
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
                            time,
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
                                time,
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
                            "sha1": "74ba370ee5643f310873fb288af1c99d639da8ca",
                            "author": {
                                "name": "Fintan Halpenny",
                                "email": "fintan.halpenny@gmail.com",
                            },
                            "summary": "Add .gitignore",
                            "message": "Add .gitignore\n",
                            "time": "1574786128",
                        },
                    },
                    "entries": [
                        {
                            "path": "src/main.rs",
                            "info": {
                                "name": "main.rs",
                                "objectType": "BLOB",
                                "lastCommit": {
                                    "sha1": "74ba370ee5643f310873fb288af1c99d639da8ca",
                                    "author": {
                                        "name": "Fintan Halpenny",
                                        "email": "fintan.halpenny@gmail.com",
                                    },
                                    "summary": "Add .gitignore",
                                    "message": "Add .gitignore\n",
                                    "time": "1574786128",
                                }
                            },
                        },
                    ],
                }
            }),
        );
    }

    #[test]
    fn query_projects() {
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
