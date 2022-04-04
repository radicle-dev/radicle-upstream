# Developer Guide

## Development prerequisites

* [NodeJS](https://nodejs.org/en/)
* [Yarn](https://yarnpkg.com/getting-started/install)
* [Rustup](https://github.com/rust-lang/rustup)
* [`cargo-watch`](https://github.com/watchexec/cargo-watch#install)
* [`cmake`](https://cmake.org/download/)
* Dependencies of [Cypress](https://docs.cypress.io/guides/getting-started/installing-cypress#System-requirements)

## Running Upstream from source

To start Upstream run `yarn start`.

Running Upstream with `yarn start` will use `<repo_root>/sandbox/lnk_home` as
the default `LNK_HOME` value to isolate your development state. To reset the
application state close Upstream, remove `<repo_root>/sandbox/lnk_home` and
start Upstream again.

If you want to run Upstream against your real user data you need to build the
application package with `yarn dist`. You can find the packaged application in
the `dist` folder and run it from there.

### Using the Git remote

Upstream provides the `git-remote-rad` binary to fetch and push Git
repositories. If you want to use the rad remote in development you need to set
the environment in your terminal:

```bash
source ./scripts/env
```

Make sure to source the script from the repository root.

## Run and connect multiple instances

We provide the `scripts/devnet.ts` tool for running, connecting and managing
multiple Upstream instances and a seed peer on a local machine. Run `yarn run
devent --help` for a list of all commands.

The devnet tool uses numbers from 1 to 100 to identify and reference Upstream
instances.

```bash
yarn run devnet upstream 1
```

This command will start an Upstream instance. The instance will be fully
initialized with user name `1`. The `LNK_HOME` for the instance is
`./sandbox/devnet/1`. The Peer ID is derived from the instance ID. The default
passphrase for all devnet Upstream instances is `asdf`.

To connect a second instance to the first one run:

```bash
yarn run devnet upstream 2 --bootstrap 1
```

Make sure you rebuild the project before running instances:

```bash
cargo build
yarn run webpack --config-name ui
```

You can also run a seed peer that tracks a certain project.

```bash
yarn run devnet seed --project <urn>
```

By default, all Upstream instances include the seed address as a bootstrap peer.
You can find the seed peer data in `./sandbox/devnet/seed`.

## Merging changes into the `main` branch

Maintainers are responsible for adding contribution to the `main` branch on
the command line. We avoid the Github UI for merging changes. See the [Licensing
and DCO RFC][dr] for background on this.

We require commits on `main` to be [signed with GPG][cm].

After successful review, Github pull requests can be merged to `main` in two
ways with a fast-forward merge being preferred.

### Fast-forward merge (preferred)

1. Rebase the pull request branch onto `main` and push it. The pull request
   branch must be only one commit ahead of main and the commit must be
   GPG-signed.
2. Wait for CI to pass
3. Check out main and merge the feature branch with

   ```bash
   git checkout main
   git merge --ff origin/feature-branch
   git push
   ```

If `main` has been updated since the rebase you need to repeat the steps.

### Squash merge

You can squash merge a branch with

```bash
git checkout main
git merge --squash --signoff <branch-name>
git push
```

Please edit the commit message so that it adequately summarizes the change and
retains all sign-offs by contributors.

## UI

The UI is written in JavaScript, [Svelte][se] is our [component language][cl]
of choice and [Electron][el] wraps it all together into a native desktop
experience. The UI code is split into `/native` and `/ui`.

For dependency management and script execution we use `yarn`. Code formatting
is dictated by [prettier][pr] and linting is provided by [eslint][es]. Both
linting and formatting are enforced locally on a pre-commit basis with
[husky][hu] and [lint-staged][ls].

Additionally we run the same checks as separate build steps on our CI, just to
make sure only properly formatted and lint-free code lands into main.

### Running tests

Before running UI end-to-end tests locally you'll need to check out the latest
test fixtures which are included in this repository as a git submodule:

```sh
./scripts/test-setup.sh
```

💡 *You'll have to run the previous command every time there are any updates to
the test fixture repository.*

We use [Cypress](https://www.cypress.io/) for integration tests and
[Jest](https://jestjs.io/) for unit tests. You can find integration tests in
the `cypress/` directory and unit tests next to the modules they correspond to.

- To run all UI tests run: `yarn test`.
- To troubleshoot integration tests via the Cypress GUI, run:
  `yarn test:integration:debug`.
- To isolate a single integration test for debugging purposes, use
  the [`.only` method][on].
- To develop unit tests in watch mode, run: `yarn test:unit:watch`


### Running on Windows (experimental)

There might be [issues due to long file paths on windows][lf]. A workaround
for this is to set `LNK_HOME` to a root folder, for example:

`$env:LNK_HOME="C:\lnk_home"`.

To try out Upstream on Windows, you can use a [free VM][fv] provided by
Microsoft.

### Building an Upstream package for your platform

You can build and package Upstream with: `yarn dist`. The generated package
will be in: `dist/` as `radicle-upstream-X.X.X.{dmg|AppImage|exe}`.

#### Apple notarization

We do Apple notarization manually on developer machines.
To set it up for the first time you'll need:

  - a paid Apple developer account
  - an App-specific password generated from your Apple ID, this allows the
    notarization script to run on behalf of our developer account
    - [Account Manage][ma] ->
    - APP-SPECIFIC PASSWORDS ->
    - Generate password…
  - a valid "Developer ID Application" certificate
    - [Certificates Add][ca] ->
    - Developer ID Application

Once you've created the _Developer ID Application_ certificate, download it
locally and add it to your keychain by double clicking on the file.

Before building a notarized DMG, make sure you're connected to the internet,
then run:

```bash
git checkout release-candidate/v0.X.XX
CSC_NAME="<YOUR_FIRST_NAME> <YOUR_LAST_NAME> (XXXXXXXXXX)" \
APPLE_ID="<YOUR_APPLE_ID_EMAIL>" \
APPLE_ID_PASSWORD="<APP_SPECIFIC_PASSWORD>" \
NOTARIZE=true \
yarn dist
```

### Scripts

To get a list of all available script commands, run: `yarn run`.

**Note:** Scripts marked with `_private` are not meant to be executed from the
the CLI, they're only to be used by other scripts.

Here's a list of all scripts that are intended for developer use:

    yarn start                  # Start Upstream in development mode

    yarn test                   # Run all UI tests
    yarn test:integration       # Run only Cypress integration tests
    yarn test:integration:debug # Show the Cypress GUI, useful for
                                # visual debugging
    yarn test:unit              # Run only Jest unit tests
    yarn test:unit:watch        # Run Jest tests in watch mode

    yarn dist                   # Build an installable Upstream package for the
                                # current platform

    yarn typescript:check       # Type-check all UI *.ts and *.svelte files
    yarn prettier:check         # Check UI code formatting
    yarn prettier:write         # Auto-format UI code
    yarn lint                   # Check UI code for linting errors

    yarn reset:state            # Delete all local state:
                                #   - identity keys
                                #   - monorepo
                                #   - saved preferences

## upstream-proxy

All of Upstream's business logic tying together the Radicle code collaboration
is provided to the UI via an HTTP API by a rust binary called `upstream-proxy`.
It uses [warp][wa] to serve a RESTish JSON API.

For dependency management and execution of common tasks we use [Cargo][co]. To
get up to speed with common functionality and manifest file intricacies consult
the exhaustive [Cargo Book][cb].

The proxy binary's lifecycle is managed by the main renderer of the UI in:
`native/main.ts`. When running `yarn dist` it is bundled together into an
application package by [electron-builder][eb].

### Testing

The proxy and UI share the same test fixtures, if you haven't done it already,
set up the test fixtures like so:

```sh
./scripts/test-setup.sh
```

💡 *You'll have to run the command every time there are any updates to the test
fixture repository.*

Then run tests as usual: `cargo test --all-features --all-targets`.

We strive for two kinds of tests: classic unit tests contained in
implementation files and integration tests. The integration tests are meant to
assert correctness of the API provided by the proxy, these can be found under
`proxy/tests`. To find out where to place and how to lay out tests, check the
Rust book [test chapter][rt].

## CI setup

We run CI builds on [Github Actions][ga].

If the UI end-to-end tests fail, screenshots and logs for the failing tests are
uploaded.

On pushes of the master branch we also build and upload distribution artifacts.

## Updating NPM dependencies

1. Run `yarn upgrade-interactive`.

2. Select newer versions if appropriate
   * For major version upgrades review the changelog to assess the impact of
     breaking changes. The assessment should be included in the commit message.
   * The following packages cannot be upgraded to the next major version because
     we don’t support ES modules yet. See [#2227](https://github.com/radicle-dev/radicle-upstream/issues/2227).
     If possible, do a minor version upgrade.
     * `node-fetch` and `@types/node-fetch`
     * `exit-hook`
     * `strip-ansi`
     * `execa`
   * Don’t update `radicle-contracts`.
   * Don’t do a major upgrade of `@types/node`.
   * `electron-builder` shows `^22.14.5` as the latest version. This is
     not the latest version. Choose the version from the “Range” column.
   * Don’t update `graphql` to v16. v15 is required as a peer dependency for
     `@apollo/client`.

3. Update transitive dependencies: Remove `yarn.lock` and run `yarn install`.

4. Run `yarn dedupe`.

## Releases

This section describes how to release a new version of Upstream.

### Prerequisites

* [`gcloud`][gc] to upload artifacts. You need to ask for access to the
  `radicle-upstream-releases` storage bucket.
* [`hub`][hb] version >= 2.14 to interact with GitHub’s API. See [its
  documentation][hc] on how to configure access
* [`brew`][br] to update the Uptream cask.

All Github access tokens _must_ have the `public_repo` scope.

### Process

1. Create release candidate
    1. Create a release candidate branch with  a commit that updates the version
       and changelog and create a pull-request

       ```bash
       ./scripts/release.ts create-rc patch
       ```

    2. Open a draft pull request on [`radicle.xyz`](http://radicle.xyz) to
       update latest version.

       ```bash
       cd radicle.xyz
       git fetch --all
       git checkout -b update-latest-release origin/master
       echo -n X.Y.Z > partials/upstream-version.mustache && make
       git commit --all --message "Point to latest upstream release"
       hub pull-request --push --draft --no-edit
       ```

2. Test the release
    1. Wait for the Linux release candidate build on CI to pass.
    2. Build and notarize the macOS binary on your local machine:

    ```bash
    CSC_NAME=… \
    APPLE_ID=… \
    APPLE_ID_PASSWORD=… \
    NOTARIZE=true \
    yarn dist
    ```

    3. Publish the CI artifacts as release candidate binaries.

       ```bash
       ./scripts/release.ts publish-rc-binaries
       ```

    4. Create QA issues for the release that link to the release candidate
       binaries.

       ```bash
       ./scripts/release.ts create-qa-issues
       ```

    5. Test the release by walking through the QA issues.
    6. (Optional) To fix bugs, create a PR with the fixes based on the release
       candidate branch. Once it has been approved, squash merge it into the
       release candidate branch (see [“Merging Pull Requests"][mp]).
       Then restart the “Test the release” step. (Skip creating a QA
       issue in 2.3.)
    7. Close the QA issues.

3. Publish and announce the release
    1. Publish the release candidate binaries under
       `https://releases.radicle.xyz` and create and publish a release tag.

       ```bash
       ./scripts/release.ts publish
       ```

    2. Merge the pull request on `radicle.xyz`.
    3. Announce the release on discord and `radicle.community`. The community
       post should highlight the important changes in the release.

       ```bash
       ./scripts/release.ts announcements
       ```

    4. Announce the release to the in-app update notification

       ```bash
       ./scripts/release.ts set-latest-release
       ```

    5. Update the [Homebrew
       cask](https://formulae.brew.sh/cask/radicle-upstream)

       ```bash
       brew tap homebrew/cask
       brew bump-cask-pr --version X.Y.Z radicle-upstream
       ```

4. Finish the release by merging the release candidate branch into main.

    ```bash
    git checkout main
    git pull --ff-only
    git merge release-candidate/vX.Y.Z --signoff
    git push
    ```

    Merging may produce a merge commit on `main` instead of fast-forwarding.
    This is ok for release candidate branches.


[br]: https://brew.sh
[ca]: https://developer.apple.com/account/resources/certificates/add
[cb]: https://doc.rust-lang.org/cargo/
[cl]: https://gist.github.com/Rich-Harris/0f910048478c2a6505d1c32185b61934
[cm]: https://docs.github.com/en/github/authenticating-to-github/managing-commit-signature-verification/signing-commits
[co]: https://github.com/rust-lang/cargo
[dr]: https://github.com/radicle-dev/radicle-decisions/blob/master/proposals/0003.md#merging-pull-requests
[eb]: https://github.com/electron-userland/electron-builder
[el]: https://www.electronjs.org
[es]: https://eslint.org
[fv]: https://developer.microsoft.com/en-us/windows/downloads/virtual-machines
[ga]: https://docs.github.com/en/actions
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[hb]: https://github.com/github/hub
[hc]: https://hub.github.com/hub.1.html#configuration
[hu]: https://github.com/typicode/husky
[lf]: https://github.com/libgit2/libgit2/issues/3053
[ls]: https://github.com/okonet/lint-staged
[ma]: https://appleid.apple.com/account/manage
[mp]: https://github.com/radicle-dev/radicle-decisions/blob/master/proposals/0003.md#merging-pull-requests
[on]: https://docs.cypress.io/guides/core-concepts/writing-and-organizing-tests.html#Excluding-and-Including-Tests
[pr]: https://prettier.io
[rt]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[se]: https://svelte.dev
[wa]: https://github.com/seanmonstar/warp
