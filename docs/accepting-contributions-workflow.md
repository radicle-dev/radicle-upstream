# Accepting Contributions Workflow

_This document outlines the user flow for users who want to contribute code to
Upstream via Upstream for the first time. We intend to use this to create a
user guide that allows users to successfully contribute._

- Contributor decides to work on an [open issue](https://github.com/radicle-dev/radicle-upstream/issues)
  or wants to propose changes to Upstream
    - [Downloads Upstream](https://radicle.xyz/tryit)
    - Installs Upstream on their machine
    - Starts Upstream and goes through the onboarding to create a new Radicle
      identity.
    - Adds the Upstream project seed in the networking screen:
      `hydyq6xmgp3amt44z41n6cbods1osx73j5z6fky5xx4yx33afycyfc@34.88.37.244:8776`
    - Follows the Upstream project
      `rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o`
    - Forks the project by clicking the `Fork` button and choosing a checkout
      directory
    - Sets up the rad remote helper by following the instructions in the fork
      modal:
        - Adds the git remote helper to their shell path:
        `export PATH="$HOME/.radicle/bin:$PATH"`
- Contributor creates patch
    - Creates a branch based on the merge coordinator’s `main` branch and
      commits the changes to that branch. The merge coordinator is announced
      every week on the [#upstream channel on discord](https://discord.gg/ju4Hjt9QnP).
    - Creates and publishes patch according to Instructions that appear when
      “New patch” is clicked in Upstream
        - Make sure you are on the branch
        - `git tag --annotate radicle-path/<name>`
        - First line is title, rest is description.
        - Reference relevant Github issues by including their URL in the
          description
        - `git push --tags rad`
- Patch is replicated by the seed node
- Author informs the merge coordinator, that there is a new patch that they
  want to merge by posting their Device ID and optionally the patch name via
  Discord or the [GitHub issue]
- The merge coordinator adds the contributor’s Device ID to the project’s
  remotes
- The merge coordinator replicates patch from seed and sees it in Upstream
- The merge coordinator pushes the branch to Github to run the build
- The merge coordinator reviews the patch according to the steps are in our
  [internal process](https://www.notion.so/Upstream-Collaboration-Workflow-9f04a70ec5c44356ad1b905b193e5f8e)
- If the merge coordinator requests changes
    - The merge coordinator informs the author of the requested changes through
      Discord or the [GitHub issue]
    - Author updates the patch
        - Add a commit to the patch branch or otherwise change the head of the
          patch branch. (E.g. by amending the commit)
        - Update the patch: `git tag --annotate radicle-patch/<name> --force`.
          This opens the editor and also allows the author to also edit the
          description.
        - Publish the tag again to radicle: `git push --tags rad --force`
    - Author informs merge coordinator that there are updates.
    - Review process starts again
- Merge coordinator merges the patch into their `main` branch. Then they
  publish their updated `main` branch and also push it to Github
- All other maintainers pull updates to the `main` branch from the merge
  coordinator through Radicle and then push their updated `main` branches to
  Radicle


[Github issue]: https://github.com/radicle-dev/radicle-upstream/issues/1958
