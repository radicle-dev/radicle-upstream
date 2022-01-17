# Code contribution using Upstream

_This document outlines the process the Upstream team follows to submit, review,
and merge code contributions using Upstream._

Currently, we only describe the process when the author of the process is a
maintainer of Upstream.

To facilitate the contribution process we designate a merge coordinator who is responsible
for merging contributions (called patches) that have been accepted into their
main branch. This role alternates weekly between the maintainers. The change is
announced on Monday morning on the #upstream channel on Discord.

The contribution process goes as follows:

- Author creates patch:
  - Author creates a branch in their working copy of Upstream based on the
    merge coordinator’s `main` branch and commits the changes to that branch.
  - Author pushes the branch to Github to trigger a build
  - Author creates and publishes an Upstream patch according to the
    instructions that appear when “New patch” is clicked in Upstream
    - Switch to the branch that contains the changes
    - Run `git tag --force --annotate radicle-patch/<name>` where `<name>` is
      the name of the patch.
    - Add a title for the patch in the editor that opens.
    - Add a description in the editor. The description should include a link to
      the CI build. The link has the following form:
      `https://github.com/radicle-dev/radicle-upstream/actions?query=branch:<branch-name>`.
      Also reference relevant Github issues by including their URL in the
      description.
    - Close the editor
    - Run `git push --force rad tag radicle-patch/<name>` to publish the patch
      to the Radicle network.
- Author selects a reviewer from the team and informs the reviewer via Discord
  that there is a new patch that they want to merge.
- Reviewer confirms that they can see the patch in Upstream.
- Reviewer reviews the diff using the following methods:
  - Reviewer looks at the diff associated to the individual patch commits in
    Upstream.
  - Reviewer checks out the patch in their working copy according to Upstream
    instructions and reviews the diff using Git-based tooling. The checkout
    instructions can be obtained when the “Checkout” button is clicked on patch.
    E.g.

    ```bash
    git fetch --force rad remotes/hyd5xybasxyicg3ap4izs8jyg1u4ardy95xaddd6yu4mzqb1oqtiye/tags/radicle-patch/cmake-dependency:tags/radicle-patch/yorgos/cmake-dependency
    git checkout tags/radicle-patch/yorgos/cmake-dependency
    ```

- If the reviewer requests some changes the following happens:
  - Reviewer informs the author of the requested changes through discord
  - Author updates the patch:
    - Author adds a commit to the patch branch or otherwise change the head of
      the patch branch. (E.g. by amending the commit)
    - Update the patch: `git tag --annotate radicle-patch/<name> --force`. This
      opens the editor and also allows the author to also edit the description
    - Run `git push rad tag radicle-patch/<name> --force` to update the patch in
      the Radicle network.
  - Author informs reviewer that their are updates
  - Review process continues with the reviewer reviewing the diff again.
- Reviewer informs author out of band that they accept the changes
- Author ensures that the patch is based cleanly on the latest main branch
  published by the merge coordinator. This may requiring rebasing the branch and
  updating the patch again.
- Author informs the reviewer and merge coordinator that the patch is ready.
- Merge coordinator merges the patch into their main branch via
  fast-forward by running

  ```bash
  git switch main
  git pull --ff-only rad "remotes/<peer-id>/tags/radicle-patch/<name>"
  ```

  If a fast-forward merge is not possible, the `--ff-only` option can be
  omitted.
- The merge coordinator publishes their updates to the main branch to Radicle
  and Github by running
  `git push rad main` and `git push origin main`.
- All other maintainers pull updates to the main branch from the merge
  coordinator through Radicle and then push their updated main branches to
  Radicle.

  ```bash
  git switch main
  git fetch peername@hrnkfoo
  git merge --ff-only peername@hrnkfoo/heads/main
  git push rad main
  ```
