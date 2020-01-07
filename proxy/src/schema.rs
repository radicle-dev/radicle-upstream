use juniper::{
    FieldError, IntoFieldError, ParseScalarResult, ParseScalarValue, RootNode, Value,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use radicle_registry_client;
use radicle_registry_client::{Error as RegistryError};
use radicle_surf::{
    file_system::{Path, SystemType},
    git::{git2, BranchName, GitBrowser, GitRepository, Sha1, TagName},
};

use crate::source::{AccountId, Project, ProjectId, Source, Error as SourceError};

#[derive(Debug)]
/// Enumerable of expected error types.
enum Error {
    /// File at a given path was irretrievable.
    FileNotFound(Path),
    /// Directory at a given path was irretrievable.
    DirectoryNotFound(Path),
    /// Project name exceeded 32 characters.
    BadProjectName(String),
    /// Project domain exceeded 32 characters.
    BadProjectDomain(String),
    /// Errors originating in radicle-surf's Git adapter.
    Git(radicle_surf::git::GitError),
    /// Registry client errors.
    Registry(RegistryError),
    /// VCS Browser could not find the last commit of a branch.
    LastCommitNotFound(Path),
}

impl From<radicle_surf::git::GitError> for Error {
    fn from(git_error: radicle_surf::git::GitError) -> Self {
        Self::Git(git_error)
    }
}

impl From<SourceError> for Error {
    fn from(source_error: SourceError) -> Self {
        match source_error {
            SourceError::BadProjectName(name) => Self::BadProjectName(name),
            SourceError::BadProjectDomain(domain) => Self::BadProjectDomain(domain),
            SourceError::Registry(error) => Self::Registry(error),
        }
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Self::Git(git_error) => {
                match &git_error {
                    radicle_surf::git::GitError::EmptyCommitHistory => {
                        FieldError::new(
                            "Repository has an empty commit history.",
                            graphql_value!({
                                "type": "EMPTY_COMMIT_HISTORY"
                            })
                        )
                    },
                    radicle_surf::git::GitError::BranchDecode => {
                        FieldError::new(
                            "Unable to decode the given branch.",
                            graphql_value!({
                                "type": "BRANCH_DECODE"
                            })
                        )
                    },
                    radicle_surf::git::GitError::NotBranch => {
                        FieldError::new(
                            "Not a known branch.",
                            graphql_value!({
                                "type": "NOT_BRANCH"
                            })
                        )
                    },
                    radicle_surf::git::GitError::NotTag => {
                        FieldError::new(
                            "Not a known tag.",
                            graphql_value!({
                                "type": "NOT_TAG"
                            })
                        )
                    },
                    radicle_surf::git::GitError::Internal(error) => {
                        FieldError::new(
                            format!("Internal Git error: {:?}", error),
                            graphql_value!({
                                "type": "INTERNAL"
                            })
                        )
                    },
                }
            },
            Self::Registry(reg_error) => {
                match reg_error {
                    RegistryError::Codec(codec_error) => {
                        FieldError::new(
                            format!("Failed to decode data: {:?}", codec_error),
                            juniper::Value::scalar("CODEC_ERROR"),
                        )
                    },
                    RegistryError::Rpc(rpc_error) => {
                        FieldError::new(
                            format!("RPC error: {:?}", rpc_error),
                            juniper::Value::scalar("RPC_ERROR"),
                        )
                    },
                    RegistryError::InvalidTransaction(error) => {
                        FieldError::new(
                            format!("Invalid transaction: {:?}", error),
                            juniper::Value::scalar("INVALID_TRANSACTION"),
                        )
                    },
                    RegistryError::Other(error) => {
                        FieldError::new(
                            format!("Registry error: {:?}", error),
                            juniper::Value::scalar("REGISTRY_ERROR"),
                        )
                    },
                }
            },
            Self::DirectoryNotFound(path) => {
                FieldError::new(
                    format!("Directory not found: {:?}", path),
                    juniper::Value::scalar("DIR_NOT_FOUND"),
                )
            },
            Self::FileNotFound(error) => {
                FieldError::new(
                    format!("File not found: {:?}", error),
                    juniper::Value::scalar("FILE_NOT_FOUND"),
                )
            },
            Self::LastCommitNotFound(error) => {
                FieldError::new(
                    format!("Last commit not found: {:?}", error),
                    juniper::Value::scalar("LAST_COMMIT_NOT_FOUND"),
                )
            },
            Self::BadProjectName(error) => {
                FieldError::new(
                    error,
                    juniper::Value::scalar("BAD_PROJECT_NAME"),
                )
            },
            Self::BadProjectDomain(error) => {
                FieldError::new(
                    error,
                    juniper::Value::scalar("BAD_PROJECT_DOMAIN"),
                )
            }
        }
    }
}

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

/// Branch name representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, GraphQLScalarValue)]
struct Branch(String);

/// Tag name representation.
///
/// We still need full tag support.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, GraphQLScalarValue)]
struct Tag(String);

/// Representation of a person (e.g. committer, author, signer) from a repository. Usually
/// extracted from a signature.
#[derive(GraphQLObject)]
struct Person {
    /// Name part of the commit signature commit.
    name: String,
    /// Email part of the commit signature commit.
    email: String,
    /// Reference (url/uri) to a persons avatar image.
    avatar: String,
}

/// Representation of a code commit.
#[derive(GraphQLObject)]
struct Commit {
    /// Identifier of the commit in the form of a sha1 hash. Often referred to as oid or object id.
    sha1: String,
    /// The author of the commit.
    author: Person,
    /// The summary of the commit message body.
    summary: String,
    /// The entire commit message body.
    message: String,
    /// The recorded time of the committer signature. This is a convenience alias until we expose
    /// the actual author and commiter signatures.
    committer_time: String,
}

// FIXME(xla): This should be a `std::convert::TryFrom` and needs to be addressed together with
//             consistent error handling.
impl From<&git2::Commit<'_>> for Commit {
    fn from(commit: &git2::Commit) -> Self {
        let signature = commit.author();
        let email = signature.email().unwrap_or("invalid email");

        let mut s = DefaultHasher::new();
        email.hash(&mut s);

        let avatar = format!(
            "https://avatars.dicebear.com/v2/jdenticon/{}.svg",
            s.finish().to_string()
        );

        Self {
            sha1: commit.id().to_string(),
            author: Person {
                name: signature.name().unwrap_or("invalid name").into(),
                email: email.into(),
                avatar,
            },
            summary: commit.summary().unwrap_or("invalid subject").into(),
            message: commit.message().unwrap_or("invalid message").into(),
            committer_time: commit.time().seconds().to_string(),
        }
    }
}

/// Git object types.
///
/// `shafiul.github.io/gitbook/1_the_git_object_model.html`
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, GraphQLEnum)]
enum ObjectType {
    /// References a list of other trees and blobs.
    Tree,
    /// Used to store file data.
    Blob,
}

/// Set of extra information we carry for blob and tree objects returned from the API.
#[derive(GraphQLObject)]
struct Info {
    /// Name part of an object.
    name: String,
    /// The type of the object.
    object_type: ObjectType,
    /// The last commmit that touched this object.
    last_commit: Commit,
}

/// Result of a directory listing, carries other trees and blobs.
#[derive(GraphQLObject)]
struct Tree {
    /// Absolute path to the tree object from the repo root.
    path: String,
    /// Entries listed in that tree result.
    entries: Vec<TreeEntry>,
    /// Extra info for the tree object.
    info: Info,
}

/// Entry in a Tree result.
#[derive(GraphQLObject)]
struct TreeEntry {
    /// Extra info for the entry.
    info: Info,
    /// Absolute path to the object from the root of the repo.
    path: String,
}

/// File data abstraction.
#[derive(GraphQLObject)]
struct Blob {
    /// Best-effort guess if the content is binary.
    binary: bool,
    /// Actual content of the file, if the content is ASCII.
    content: Option<String>,
    /// Extra info for the file.
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
    ) -> Result<Project, Error> {
        ctx.source.register_project(name, description, img_url)
            .map_err(|error| error.into())
    }
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn blob(ctx: &Context, id: IdInput, revision: String, path: String)
            -> Result<Blob, Error> {
        let repo = GitRepository::new(&ctx.dummy_repo_path)?;
        let mut browser = GitBrowser::new(&repo)?;

        // Best effort to guess the revision.
        browser
            .branch(BranchName::new(&revision))
            .or_else(|_| browser.commit(Sha1::new(&revision)))
            .or_else(|_| browser.tag(TagName::new(&revision)))?;

        let root = browser.get_directory()?;

        let mut p = Path::root();
        p.append(&mut Path::from_string(&path));
        let file = root.find_file(&p).ok_or_else(|| { Error::FileNotFound(p.clone()) })?;
        let last_commit = browser.last_commit(&p)
            .ok_or(radicle_surf::git::GitError::EmptyCommitHistory)?;
        let (_rest, last) = p.split_last();
        let (binary, content) = {
            let res = std::str::from_utf8(&file.contents);

            match res {
                Ok(content) => (false, Some(content.to_string())),
                Err(_) => (true, None),
            }
        };

        Ok(Blob {
            binary,
            content,
            info: Info {
                name: last.label,
                object_type: ObjectType::Blob,
                last_commit: Commit::from(&last_commit),
            },
        })
    }

    fn commit(ctx: &Context, id: IdInput, sha1: String) -> Result<Commit, Error> {
        let repo = GitRepository::new(&ctx.dummy_repo_path)?;
        let mut browser = GitBrowser::new(&repo)?;
        browser.commit(radicle_surf::vcs::git::Sha1::new(&sha1))?;

        let history = browser.get_history();
        let commit = history.0.first();

        Ok(Commit::from(commit))
    }

    fn branches(ctx: &Context, id: IdInput) -> Result<Vec<Branch>, Error> {
        let repo = GitRepository::new(&ctx.dummy_repo_path)?;
        let browser = GitBrowser::new(&repo)?;
        let mut branches: Vec<Branch> = browser
            .list_branches(None)?
            .into_iter()
            .map(|b| Branch(b.name.name()))
            .collect();

        branches.sort();

        Ok(branches)
    }

    fn tags(ctx: &Context, id: IdInput) -> Result<Vec<Tag>, Error> {
        let repo = GitRepository::new(&ctx.dummy_repo_path)?;
        let browser = GitBrowser::new(&repo)?;
        let mut tag_names = browser.list_tags()?;

        tag_names.sort();

        let mut tags: Vec<Tag> = tag_names
            .into_iter()
            .map(|tag_name| Tag(tag_name.name()))
            .collect();

        tags.sort();

        Ok(tags)
    }

    fn tree(ctx: &Context, id: IdInput, revision: String, prefix: String) -> Result<Tree, Error> {
        let repo = GitRepository::new(&ctx.dummy_repo_path)?;
        let mut browser = GitBrowser::new(&repo)?;

        browser
            .branch(BranchName::new(&revision))
            .or_else(|_| browser.commit(Sha1::new(&revision)))
            .or_else(|_| browser.tag(TagName::new(&revision)))?;

        let path = if prefix == "/" || prefix == "" {
            Path::root()
        } else {
            let mut root = Path::root();
            root.append(&mut Path::from_string(&prefix));
            root
        };

        let root_dir = browser.get_directory()?;
        let prefix_dir = if path.is_root() {
            Ok(root_dir)
        } else {
            root_dir.find_directory(&path).ok_or({ Error::DirectoryNotFound(path.clone()) })
        }?;
        let mut prefix_contents = prefix_dir.list_directory();
        prefix_contents.sort();

        let maybe_entries: Result<Vec<TreeEntry>, Error> = prefix_contents
            .iter()
            .map(|(label, system_type)| {
                let entry_path = {
                    let mut path = path.clone();
                    path.push(label.clone());
                    path
                };
                let last_commit = match &browser.last_commit(&path) {
                    Some(last_commit) => Ok(Commit::from(last_commit)),
                    None => Err(radicle_surf::git::GitError::EmptyCommitHistory),
                }?;
                let info = Info {
                    name: label.to_string(),
                    object_type: match system_type {
                        SystemType::Directory => ObjectType::Tree,
                        SystemType::File => ObjectType::Blob,
                    },
                    last_commit,
                };

                let (_root, labels) = entry_path.split_first();
                let clean_path = match nonempty::NonEmpty::from_slice(labels) {
                    Some(clean_path) => Ok(Path(clean_path)),
                    None => Err(Error::FileNotFound(entry_path)),
                }?;

                Ok(TreeEntry {
                    info,
                    path: clean_path.to_string(),
                })
            })
            .collect();

        let mut entries = maybe_entries?;

        // We want to ensure that in the response Tree entries come first. `Ord` being derived on
        // the enum ensures Variant declaration order.
        //
        // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
        entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

        let last_commit = if path.is_root() {
            Ok(Commit::from(browser.get_history().0.first()))
        } else {
            match &browser.last_commit(&path) {
                Some(last_commit) => Ok(Commit::from(last_commit)),
                None => Err(Error::LastCommitNotFound(path.clone())),
            }
        }?;
        let name = if path.is_root() {
            "".into()
        } else {
            let (_first, last) = path.split_last();
            last.label
        };
        let info = Info {
            name,
            object_type: ObjectType::Tree,
            last_commit,
        };

        Ok(Tree {
            path: prefix,
            entries,
            info,
        })
    }

    fn projects(ctx: &Context) -> Result<Vec<Project>, Error> {
        Ok(ctx.source.get_all_projects()?)
    }

    fn project(ctx: &Context, id: IdInput) -> Result<Option<Project>, Error> {
        Ok(ctx.source.get_project(id.into())?)
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

    const REPO_PATH: &str = "../fixtures/git-platinum";

    fn execute_query(
        query: &'static str,
        vars: &Variables,
    ) -> (
        juniper::Value,
        Vec<juniper::ExecutionError<juniper::DefaultScalarValue>>,
    ) {
        let registry_client = MemoryClient::new();
        let mut source = Ledger::new(registry_client);

        setup_fixtures(&mut source);

        let ctx = Context::new(REPO_PATH.into(), source);
        let schema = Schema::new(Query, Mutation);

        juniper::execute(query, None, &schema, vars, &ctx)
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
        let (res, errors) = execute_query(query, &vars);

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
    fn query_blob_binary() {
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
    fn query_blob_in_root() {
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
    fn query_branches() {
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
    fn query_commit() {
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
    fn query_tags() {
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
    fn query_tags_error() {
        let mut vars = Variables::new();
        let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

        id_map.insert("domain".into(), InputValue::scalar("rad"));
        id_map.insert("name".into(), InputValue::scalar("upstream"));
        vars.insert("id".into(), InputValue::object(id_map));

        let (_res, errors) = execute_query("query($id: IdInput!) { tags(id: $id) }", &vars);

        assert_eq!(errors, []);
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
    fn query_tree_root() {
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
    fn query_projects() {
        let vars = Variables::new();
        let (res, errors) = execute_query("query { projects { name } }", &vars);

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
