use juniper::{FieldError, FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
use std::sync::Arc;

use radicle_surf::{
    file_system::Path,
    git::{BranchName, GitBrowser, GitRepository, Sha1, TagName},
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
impl From<&radicle_surf::vcs::git::git2::Commit<'_>> for Commit {
    fn from(commit: &radicle_surf::vcs::git::git2::Commit) -> Self {
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

#[derive(GraphQLObject)]
struct Tree {
    path: String,
    entries: Vec<TreeEntry>,
}

#[derive(GraphQLObject)]
struct TreeEntry {
    path: String,
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn commit(ctx: &Context, id: IdInput, sha1: String) -> FieldResult<Option<Commit>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let mut browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        browser
            .commit(radicle_surf::vcs::git::Sha1::new(&sha1))
            .expect("setting commit failed");

        let history = browser.get_history();
        let commit = history.0.first();

        Ok(Some(Commit::from(commit)))
    }

    fn branches(ctx: &Context, id: IdInput) -> FieldResult<Vec<Branch>> {
        let repo = GitRepository::new(&ctx.dummy_repo_path).expect("setting up repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let branche_names = browser.list_branches().expect("Getting branches failed");
        let branches = branche_names
            .into_iter()
            .map(|branch_name| Branch {
                name: branch_name.name(),
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

        let path = std::path::Path::new(&prefix);
        let path_components: Vec<radicle_surf::file_system::Label> = path
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
            .map(|(label, _type)| TreeEntry {
                path: label.to_string(),
            })
            .collect();

        Ok(Tree {
            path: prefix,
            entries,
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
                    entries {
                        path,
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
                    "entries": [
                        { "path": "main.rs" },
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
