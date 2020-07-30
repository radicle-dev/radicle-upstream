//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use std::ffi;
use std::path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use librad::git::local::url::LocalUrl;
use librad::meta::project;

use crate::coco;
use crate::error;
use crate::registry;

/// Object the API returns for project metadata.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
}

impl<ST> From<project::Project<ST>> for Metadata
where
    ST: Clone,
{
    fn from(project_meta: project::Project<ST>) -> Self {
        Self {
            name: project_meta.name().to_string(),
            description: project_meta
                .description()
                .clone()
                .unwrap_or_else(|| "".into()),
            default_branch: project_meta.default_branch().to_string(),
        }
    }
}

/// Radicle project for sharing and collaborating.
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Informs if the project is present in the Registry and under what top-level entity it can be
    /// found.
    pub registration: Option<Registration>,
    /// High-level statistics about the project
    pub stats: coco::Stats,
}

/// Construct a Project from its metadata and stats
impl<ST> From<(project::Project<ST>, coco::Stats)> for Project
where
    ST: Clone,
{
    /// Create a `Project` given a `librad` defined [`project::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn from((project, stats): (project::Project<ST>, coco::Stats)) -> Self {
        let id = project.urn();

        Self {
            id: id.clone(),
            shareable_entity_identifier: format!("%{}", id),
            metadata: project.into(),
            registration: None,
            stats,
        }
    }
}

/// Variants for possible registration states of a project.
#[allow(dead_code)]
pub enum Registration {
    /// Project is registered under an Org.
    Org(registry::Id),
    /// Project is registered under a User.
    User(registry::Id),
}

/// Fetch the project with a given urn from a peer
pub fn get(api: &coco::Api, project_urn: &coco::Urn) -> Result<Project, error::Error> {
    let project = api.get_project(project_urn, None)?;
    let stats = api.with_browser(project_urn, |browser| Ok(browser.get_stats()?))?;

    Ok((project, stats).into())
}

/// Specify how to create the git credential helper argument for a [`Checkout`]
enum Credential {
    Password(String),
}

impl Credential {
    fn to_helper(&self) -> String {
        match self {
            Credential::Password(pass) => format!(
                "credential.helper=!f() {{ test \"$1\" = get && echo \"password={}\"; }}; f",
                pass
            ),
        }
    }
}

/// The data necessary for checking out a project.
pub struct Checkout<P>
where
    P: AsRef<path::Path>,
{
    /// The credential helper.
    credential: Credential,
    /// The project URN.
    urn: coco::Urn,
    /// The branch to use for the checkout.
    branch: String,
    /// The path on the filesystem where we're going to checkout to.
    path: P,
    bin_path: Option<ffi::OsString>,
}

impl<P> Checkout<P>
where
    P: AsRef<path::Path>,
{
    /// Create a new `Checkout` with the mock `Credential::Password` helper.
    pub fn new<Bin>(urn: coco::Urn, branch: String, path: P, bin_path: Bin) -> Self
    where
        Bin: Into<Option<ffi::OsString>>,
    {
        Checkout {
            // TODO(rudolfs): we'll have to figure out how to pass the secret
            // key to git in a safe manner. As it is now it could be sniffed
            // out from the process list while the user is doing a clone.
            //
            // How will we get ahold on the secret key here?
            credential: Credential::Password("radicle-upstream".to_owned()),
            urn,
            branch,
            path,
            bin_path: bin_path.into(),
        }
    }

    /// Checkout a working copy of a [`Project`].
    ///
    /// NOTE: 'RAD_HOME' should be expected to be set if using a custom root for
    /// [`librad::paths::Paths`]. If it is not set the underlying binary will delegate to the
    /// `ProjectDirs` setup of the `Paths`.
    pub fn run(self) -> Result<(), error::Error> {
        let bin_path = match self.bin_path {
            Some(path) => Ok(path),
            None => Self::default_bin_path(),
        }?;

        let mut child_process = Command::new("git")
            .arg("-c")
            .arg(self.credential.to_helper())
            .arg("clone")
            .arg("-b")
            .arg(self.branch)
            .arg(LocalUrl::from(self.urn).to_string())
            .arg(&self.path.as_ref().as_os_str())
            .env("PATH", &bin_path)
            .envs(std::env::vars().filter(|(key, _)| key.starts_with("GIT_TRACE")))
            .spawn()?;

        // TODO: Capture the error if any and respond
        let result = child_process.wait()?;

        if result.success() {
            Ok(())
        } else {
            Err(error::Error::Checkout)
        }
    }

    /// Set up the PATH env variable used for running the checkout.
    fn default_bin_path() -> Result<ffi::OsString, error::Error> {
        let exe_path = std::env::current_exe()?;
        let exe_path = exe_path.parent().expect("failed to find executable path");

        let paths = std::env::var_os("PATH").map_or(vec![exe_path.to_path_buf()], |path| {
            let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
            paths.push(exe_path.to_path_buf());
            paths.reverse();
            paths
        });

        Ok(std::env::join_paths(paths)?)
    }
}
