use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use radicle_surf::{
    diff, file_system,
    vcs::git::{self, git2, BranchType, Browser, Rev},
};

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::error;
use crate::session::settings::Theme;

lazy_static::lazy_static! {
    // The syntax set is slow to load (~30ms), so we make sure to only load it once.
    // It _will_ affect the latency of the first request that uses syntax highlighting,
    // but this is acceptable for now.
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

/// Branch name representation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Branch(pub(crate) String);

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tag name representation.
///
/// We still need full tag support.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag(pub(crate) String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of a person (e.g. committer, author, signer) from a repository. Usually
/// extracted from a signature.
pub struct Person {
    /// Name part of the commit signature.
    pub name: String,
    /// Email part of the commit signature.
    pub email: String,
}

/// Commit statistics.
#[derive(Serialize)]
pub struct CommitStats {
    /// Additions.
    pub additions: u64,
    /// Deletions.
    pub deletions: u64,
}

/// Representation of a changeset between two revs.
pub struct Commit {
    /// The commit header.
    pub header: CommitHeader,
    /// The change statistics for this commit.
    pub stats: CommitStats,
    /// The changeset introduced by this commit.
    pub diff: diff::Diff,
    /// The branch this commit belongs to.
    pub branch: Branch,
}

/// Representation of a code commit.
pub struct CommitHeader {
    /// Identifier of the commit in the form of a sha1 hash. Often referred to as oid or object
    /// id.
    pub sha1: git2::Oid,
    /// The author of the commit.
    pub author: Person,
    /// The summary of the commit message body.
    pub summary: String,
    /// The entire commit message body.
    pub message: String,
    /// The committer of the commit.
    pub committer: Person,
    /// The recorded time of the committer signature. This is a convenience alias until we
    /// expose the actual author and commiter signatures.
    pub committer_time: git2::Time,
}

impl CommitHeader {
    /// Returns the commit description text. This is the text after the one-line summary.
    #[must_use]
    pub fn description(&self) -> &str {
        self.message
            .strip_prefix(&self.summary)
            .unwrap_or(&self.message)
            .trim()
    }
}

impl From<&git::Commit> for CommitHeader {
    fn from(commit: &git::Commit) -> Self {
        Self {
            sha1: commit.id,
            author: Person {
                name: commit.author.name.clone(),
                email: commit.author.email.clone(),
            },
            summary: commit.summary.clone(),
            message: commit.message.clone(),
            committer: Person {
                name: commit.committer.name.clone(),
                email: commit.committer.email.clone(),
            },
            committer_time: commit.author.time,
        }
    }
}

/// Git object types.
///
/// `shafiul.github.io/gitbook/1_the_git_object_model.html`
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum ObjectType {
    /// References a list of other trees and blobs.
    Tree,
    /// Used to store file data.
    Blob,
}

/// Set of extra information we carry for blob and tree objects returned from the API.
pub struct Info {
    /// Name part of an object.
    pub name: String,
    /// The type of the object.
    pub object_type: ObjectType,
    /// The last commmit that touched this object.
    pub last_commit: Option<CommitHeader>,
}

/// File data abstraction.
pub struct Blob {
    /// Actual content of the file, if the content is ASCII.
    pub content: BlobContent,
    /// Extra info for the file.
    pub info: Info,
    /// Absolute path to the object from the root of the repo.
    pub path: String,
}

impl Blob {
    /// Indicates if the content of the [`Blob`] is binary.
    #[must_use]
    pub fn is_binary(&self) -> bool {
        self.content == BlobContent::Binary
    }

    /// Indicates if the content of the [`Blob`] is HTML.
    #[must_use]
    pub fn is_html(&self) -> bool {
        matches!(self.content, BlobContent::Html(_))
    }
}

/// Variants of blob content.
#[derive(PartialEq)]
pub enum BlobContent {
    /// Content is ASCII and can be passed as a string.
    Ascii(String),
    /// Content is syntax-highlighted HTML.
    Html(String),
    /// Content is binary and needs special treatment.
    Binary,
}

/// Result of a directory listing, carries other trees and blobs.
pub struct Tree {
    /// Absolute path to the tree object from the repo root.
    pub path: String,
    /// Entries listed in that tree result.
    pub entries: Vec<TreeEntry>,
    /// Extra info for the tree object.
    pub info: Info,
}

// TODO(xla): Ensure correct by construction.
/// Entry in a Tree result.
pub struct TreeEntry {
    /// Extra info for the entry.
    pub info: Info,
    /// Absolute path to the object from the root of the repo.
    pub path: String,
}

/// A revision selector for a `Browser`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Revision<P> {
    /// Select a tag under the name provided.
    Tag {
        /// Name of the tag.
        name: String,
    },
    /// Select a branch under the name provided.
    Branch {
        /// Name of the branch.
        name: String,
        /// The remote peer, if specified.
        peer_id: Option<P>,
    },
    /// Select a SHA1 under the name provided.
    Sha {
        /// The SHA1 value.
        sha: String,
    },
}

impl<P> TryFrom<Revision<P>> for Rev
where
    P: ToString,
{
    type Error = error::Error;

    fn try_from(other: Revision<P>) -> Result<Self, Self::Error> {
        match other {
            Revision::Tag { name } => Ok(git::TagName::new(&name).into()),
            Revision::Branch { name, peer_id } => Ok(match peer_id {
                Some(peer) => git::Branch::remote(&name, &peer.to_string()).into(),
                None => git::Branch::local(&name).into(),
            }),
            Revision::Sha { sha } => Ok(git::Oid::from_str(&sha)?.into()),
        }
    }
}

/// Bundled response to retrieve both [`Branch`]es and [`Tag`]s for a [`user::User`]'s repo.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Revisions<P, U> {
    /// The peer identifier for the provided user.
    pub peer_id: P,
    /// Owner of the revision set.
    pub user: U,
    /// List of [`git::Branch`].
    pub branches: Vec<Branch>,
    /// List of [`git::Tag`].
    pub tags: Vec<Tag>,
}

/// Returns the [`Blob`] for a file at `revision` under `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or a surf interaction fails.
pub fn blob<P>(
    browser: &mut Browser,
    default_branch: git::Branch,
    maybe_revision: Option<Revision<P>>,
    path: &str,
    theme: Option<&Theme>,
) -> Result<Blob, error::Error>
where
    P: ToString,
{
    let maybe_revision = maybe_revision.map(Rev::try_from).transpose()?;
    browser.rev(maybe_revision.unwrap_or_else(|| default_branch.into()))?;

    let root = browser.get_directory()?;
    let p = file_system::Path::from_str(path)?;

    let file = root
        .find_file(p.clone())
        .ok_or_else(|| error::Error::PathNotFound(p.clone()))?;

    let mut commit_path = file_system::Path::root();
    commit_path.append(p.clone());

    let last_commit = browser
        .last_commit(commit_path)?
        .map(|c| CommitHeader::from(&c));
    let (_rest, last) = p.split_last();

    let content = blob_content(path, &file.contents, theme);

    Ok(Blob {
        content,
        info: Info {
            name: last.to_string(),
            object_type: ObjectType::Blob,
            last_commit,
        },
        path: path.to_string(),
    })
}

/// Return a [`BlobContent`] given a file path, content and theme. Attempts to perform syntax
/// highlighting when the theme is `Some`.
fn blob_content(path: &str, content: &[u8], theme: Option<&Theme>) -> BlobContent {
    match (std::str::from_utf8(content), theme) {
        (Ok(content), None) => BlobContent::Ascii(content.to_owned()),
        (Ok(content), Some(theme)) => {
            let syntax = std::path::Path::new(path)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .and_then(|ext| SYNTAX_SET.find_syntax_by_extension(ext));

            let ts = ThemeSet::load_defaults();
            let theme = match theme {
                Theme::Light => ts.themes.get("base16-ocean.light"),
                Theme::Dark => ts.themes.get("base16-ocean.dark"),
            };

            match (syntax, theme) {
                (Some(syntax), Some(theme)) => {
                    let mut highlighter = HighlightLines::new(syntax, theme);
                    let mut html = String::with_capacity(content.len());

                    for line in LinesWithEndings::from(content) {
                        let regions = highlighter.highlight(line, &SYNTAX_SET);
                        syntect::html::append_highlighted_html_for_styled_line(
                            &regions[..],
                            syntect::html::IncludeBackground::No,
                            &mut html,
                        );
                    }
                    BlobContent::Html(html)
                },
                _ => BlobContent::Ascii(content.to_owned()),
            }
        },
        (Err(_), _) => BlobContent::Binary,
    }
}

/// Given a project id to a repo returns the list of branches.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn branches<'repo>(
    browser: &Browser<'repo>,
    branch_type: Option<BranchType>,
) -> Result<Vec<Branch>, error::Error> {
    let mut branches = browser
        .list_branches(branch_type)?
        .into_iter()
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Information about a locally checked out repository.
#[derive(Deserialize, Serialize)]
pub struct LocalState {
    /// List of branches.
    branches: Vec<Branch>,
    /// Indicator if the repository is associated to coco project.
    managed: bool,
}

/// Given a path to a repo returns the list of branches and if it is managed by coco.
///
/// # Errors
///
/// Will return [`error::Error`] if the repository doesn't exist.
pub fn local_state(repo_path: &str) -> Result<LocalState, error::Error> {
    let repo = git::Repository::new(repo_path)?;
    // TODO(finto): This should be the default branch of the project, possibly.
    let browser = Browser::new(&repo, git::Branch::local("master"))?;
    let mut branches = browser
        .list_branches(Some(BranchType::Local))?
        .into_iter()
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    let managed = {
        let repo = git2::Repository::open(repo_path)?;
        repo.remotes()?.into_iter().flatten().any(|r| r == "rad")
    };

    Ok(LocalState { branches, managed })
}

/// Retrieves the [`CommitHeader`] for the given `sha1`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commit_header<'repo>(
    browser: &mut Browser<'repo>,
    sha1: &str,
) -> Result<CommitHeader, error::Error> {
    browser.commit(git::Oid::from_str(sha1)?)?;

    let history = browser.get();
    let commit = history.first();

    Ok(CommitHeader::from(commit))
}

/// Retrieves a [`Commit`].
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commit<'repo>(browser: &mut Browser<'repo>, sha1: &str) -> Result<Commit, error::Error> {
    let oid = git::Oid::from_str(sha1)?;
    browser.commit(oid)?;

    let history = browser.get();
    let commit = history.first();

    let diff = if let Some(parent) = commit.parents.first() {
        browser.diff(*parent, oid)?
    } else {
        browser.initial_diff(oid)?
    };

    let mut deletions = 0;
    let mut additions = 0;

    for file in &diff.modified {
        if let diff::FileDiff::Plain { ref hunks } = file.diff {
            for hunk in hunks.iter() {
                for line in &hunk.lines {
                    match line {
                        diff::LineDiff::Addition { .. } => additions += 1,
                        diff::LineDiff::Deletion { .. } => deletions += 1,
                        _ => {},
                    }
                }
            }
        }
    }

    let branches = browser.revision_branches(oid)?;

    // If a commit figures in more than one branch, there's no real way to know
    // which branch to show without additional context. So, we choose the first
    // branch.
    let branch = branches.first();

    // Known commits always have at least one branch. If this isn't the case, it's a bug.
    let branch = Branch(
        branch
            .expect("known commits must be on a branch")
            .name
            .to_string(),
    );

    Ok(Commit {
        header: commit.into(),
        stats: CommitStats {
            additions,
            deletions,
        },
        branch,
        diff,
    })
}

/// Retrieves the [`Commit`] history for the given `branch`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commits<'repo>(
    browser: &mut Browser<'repo>,
    branch: git::Branch,
) -> Result<Vec<CommitHeader>, error::Error> {
    browser.branch(branch)?;

    let commits = browser.get().iter().map(CommitHeader::from).collect();

    Ok(commits)
}

/// Retrieves the list of [`Tag`] for the given project `id`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn tags<'repo>(browser: &Browser<'repo>) -> Result<Vec<Tag>, error::Error> {
    let tag_names = browser.list_tags()?;
    let mut tags: Vec<Tag> = tag_names
        .into_iter()
        .map(|tag_name| Tag(tag_name.name().to_string()))
        .collect();

    tags.sort();

    Ok(tags)
}

/// Retrieve the [`Tree`] for the given `revision` and directory `prefix`.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the surf interactions fail.
pub fn tree<'repo, P>(
    browser: &mut Browser<'repo>,
    default_branch: git::Branch,
    maybe_revision: Option<Revision<P>>,
    maybe_prefix: Option<String>,
) -> Result<Tree, error::Error>
where
    P: ToString,
{
    let maybe_revision = maybe_revision.map(Rev::try_from).transpose()?;
    let revision = maybe_revision.unwrap_or_else(|| default_branch.into());
    let prefix = maybe_prefix.unwrap_or_default();

    browser.rev(revision)?;

    let path = if prefix == "/" || prefix == "" {
        file_system::Path::root()
    } else {
        file_system::Path::from_str(&prefix)?
    };

    let root_dir = browser.get_directory()?;
    let prefix_dir = if path.is_root() {
        root_dir
    } else {
        root_dir
            .find_directory(path.clone())
            .ok_or_else(|| error::Error::PathNotFound(path.clone()))?
    };
    let mut prefix_contents = prefix_dir.list_directory();
    prefix_contents.sort();

    let entries_results: Result<Vec<TreeEntry>, error::Error> = prefix_contents
        .iter()
        .map(|(label, system_type)| {
            let entry_path = if path.is_root() {
                file_system::Path::new(label.clone())
            } else {
                let mut p = path.clone();
                p.push(label.clone());
                p
            };
            let mut commit_path = file_system::Path::root();
            commit_path.append(entry_path.clone());

            let info = Info {
                name: label.to_string(),
                object_type: match system_type {
                    file_system::SystemType::Directory => ObjectType::Tree,
                    file_system::SystemType::File => ObjectType::Blob,
                },
                last_commit: None,
            };

            Ok(TreeEntry {
                info,
                path: entry_path.to_string(),
            })
        })
        .collect();

    let mut entries = entries_results?;

    // We want to ensure that in the response Tree entries come first. `Ord` being derived on
    // the enum ensures Variant declaration order.
    //
    // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
    entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

    let last_commit = if path.is_root() {
        Some(CommitHeader::from(browser.get().first()))
    } else {
        None
    };
    let name = if path.is_root() {
        "".into()
    } else {
        let (_first, last) = path.split_last();
        last.to_string()
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

/// Get all [`Revisions`] for a given project.
///
/// # Parameters
///
/// * `peer_id` - the identifier of this peer
/// * `owner` - the owner of this peer, i.e. the current user
/// * `peers` - an iterator of a peer and the default self it used for this project
///
/// # Errors
///
///   * [`error::Error::LibradLock`]
///   * [`error::Error::Git`]
pub fn revisions<P, U>(
    browser: &Browser,
    peer_id: P,
    owner: U,
    peers: Vec<(P, U)>,
) -> Result<NonEmpty<Revisions<P, U>>, error::Error>
where
    P: Clone + ToString,
{
    let mut user_revisions = vec![];

    let local_branches = branches(browser, Some(BranchType::Local))?;
    if !local_branches.is_empty() {
        user_revisions.push(Revisions {
            peer_id,
            user: owner,
            branches: local_branches,
            tags: tags(browser)?,
        })
    }

    for (peer_id, user) in peers {
        let remote_branches = branches(browser, Some(into_branch_type(Some(peer_id.clone()))))?;

        user_revisions.push(Revisions {
            peer_id,
            user,
            branches: remote_branches,
            // TODO(rudolfs): implement remote peer tags once we decide how
            // https://radicle.community/t/git-tags/214
            tags: vec![],
        });
    }

    NonEmpty::from_vec(user_revisions).ok_or(error::Error::EmptyRevisions)
}

/// Turn an `Option<P>` into a [`BranchType`]. If the `P` is present then this is
/// set as the remote of the `BranchType`. Otherwise, it's local branch.
#[must_use]
pub fn into_branch_type<P>(peer_id: Option<P>) -> BranchType
where
    P: ToString,
{
    peer_id.map_or(BranchType::Local, |peer_id| BranchType::Remote {
        // We qualify the remotes as the PeerId + heads, otherwise we would grab the tags too.
        name: Some(format!("{}/heads", peer_id.to_string())),
    })
}

#[cfg(test)]
mod tests {
    use librad::keys::SecretKey;
    use radicle_surf::vcs::git;

    use crate::coco;
    use crate::error;

    #[tokio::test]
    async fn browse_commit() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key.clone(), tmp_dir)?;
        let peer = coco::create_peer_api(config).await?;
        let owner = coco::init_user(&peer, key.clone(), "cloudhead")?;
        let owner = coco::verify_user(owner)?;
        let platinum_project = coco::control::replicate_platinum(
            &peer,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();
        let sha = "91b69e00cd8e5a07e20942e9e4457d83ce7a3ff1";

        let commit = coco::with_browser(&peer, &urn, |browser| super::commit_header(browser, sha))?;

        assert_eq!(commit.sha1, git::Oid::from_str(sha)?);

        Ok(())
    }
}
