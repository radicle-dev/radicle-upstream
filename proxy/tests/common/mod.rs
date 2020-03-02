use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
use librad::git::ProjectId;
use librad::paths::Paths;
use librad::surf;
use std::env;
use surf::git::git2;
use tempfile::{tempdir_in, TempDir};

use proxy::coco;
use proxy::graphql::schema::{Context, Mutation, Query, Schema};

const REPO_PATH: &str = "../fixtures/git-platinum";

pub fn with_fixtures<F>(f: F)
where
    F: FnOnce(Paths, TempDir, ProjectId) -> (),
{
    let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
    let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");
    let repos_dir = tempdir_in(tmp_dir.path()).expect("unable to create repos directory");

    // Craft the absolute path to git-platinum fixtures.
    let mut platinum_path = env::current_dir().expect("unable to get working directory");
    platinum_path.push(REPO_PATH);
    let mut platinum_from = String::from("file://");
    platinum_from.push_str(
        platinum_path
            .to_str()
            .expect("unable to get fixtures path string"),
    );
    // Construct path for fixtures to clone into.
    let platinum_into = tmp_dir.path().join("git-platinum");

    // Clone a copy into temp directory.
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(&platinum_from, platinum_into.as_path())
        .expect("unable to clone fixtures repo");

    coco::setup_fixtures(
        &librad_paths,
        tmp_dir.path().to_str().expect("path extraction failed"),
    )
    .expect("fixture setup failed");

    // Init as rad project.
    let (platinum_id, _platinum_project) = crate::coco::init_project(
        &librad_paths,
        platinum_into.to_str().unwrap(),
        "git-platinum",
        "fixture data",
        "master",
        "https://avatars0.githubusercontent.com/u/48290027",
    )
    .unwrap();

    let platinum_surf_repo = surf::git::Repository::new(platinum_into.to_str().unwrap()).unwrap();
    let platinum_browser = surf::git::Browser::new(platinum_surf_repo).unwrap();
    let mut rad_remote = platinum_repo.find_remote("rad").unwrap();

    // Push all tags to rad remote.
    let tags = platinum_browser
        .list_tags()
        .unwrap()
        .iter()
        .map(|t| format!("+refs/tags/{}", t.name()))
        .collect::<Vec<String>>();
    rad_remote
        .push(&tags.iter().map(String::as_str).collect::<Vec<_>>(), None)
        .unwrap();

    f(librad_paths, repos_dir, platinum_id)
}

pub fn execute_query<F>(librad_paths: Paths, query: &str, vars: &Variables, f: F)
where
    F: FnOnce(Value, Vec<ExecutionError<DefaultScalarValue>>) -> (),
{
    let ctx = Context::new(
        librad_paths,
        radicle_registry_client::Client::new_emulator(),
    );
    let (res, errors) = juniper::execute(query, None, &Schema::new(Query, Mutation), vars, &ctx)
        .expect("test execute failed");

    f(res, errors);
}
