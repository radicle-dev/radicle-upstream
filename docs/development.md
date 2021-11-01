# Developer Guide

## Development prerequisites

* [NodeJS](https://nodejs.org/en/)
* [Yarn](https://yarnpkg.com/getting-started/install)
* [Rustup](https://github.com/rust-lang/rustup)
* [`cargo-watch`](https://github.com/watchexec/cargo-watch#install)
* Dependencies of [Cypress](https://docs.cypress.io/guides/getting-started/installing-cypress#System-requirements)

## Running Upstream from source

To start Upstream run `yarn start`.

Running Upstream with `yarn start` will use `<repo_root>/sandbox/rad_home` as
the default `RAD_HOME` value to isolate your development state. To reset the
application state close Upstream, remove `<repo_root>/sandbox/rad_home` and
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

## Merging changes into master

Maintainers are responsible for adding contribution to the `master` branch on
the command line. We avoid the Github UI for merging changes. See the [Licensing
and DCO RFC][dco-rfc] for background on this.

We require commits on master to be [signed with GPG][commit-sign-gpg].

After successful review, Github pull requests can be merged to `master` in two
ways with a fast-forward merge being preferred.

### Fast-forward merge (preferred)

1. Rebase the pull request branch onto `master` and push it. The pull request
   branch must be only one commit ahead of master and the commit must be
   GPG-signed.
2. Wait for CI to pass
3. Check out master and merge the feature branch with

   ```bash
   git checkout master
   git merge --ff origin/feature-branch
   git push
   ```

If `master` has been updated since the rebase you need to repeat the steps.

### Squash merge

You can squash merge a branch with

```bash
git checkout master
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
make sure only properly formatted and lint-free code lands into master.

### Running tests

Before running UI end-to-end tests locally you'll need to check out the latest
test fixtures which are included in this repository as a git submodule:

```sh
./scripts/test-setup.sh
```

💡 *You'll have to run the previous commands every time there are any updates
to the test fixture repository.*

We use [Cypress](https://www.cypress.io/) for integration tests and
[Jest](jestjs.io) for unit tests. You can find integration tests in the
`cypress/` directory and unit tests next to the modules they correspond to.

- To run all ui tests call: `yarn test`.
- To troubleshoot integration tests via the Cypress GUI, run:
  `yarn test:integration:debug`.
- To isolate a single integration test for debugging purposes, use
  the [`.only` method][on].
- To develop unit tests in watch mode, run: `yarn test:unit:watch`


### Running multiple Upstream instances on the same machine

For testing purposes it is possible to launch multiple Upstream instances at
the same time. At the moment this is only possible in development mode.

```
mkdir /Users/rudolfs/work/19000
mkdir /Users/rudolfs/work/20000

# Launch the first instance.

RAD_HOME="/Users/rudolfs/work/19000" \
RADICLE_UPSTREAM_PROXY_ARGS="--http-listen 127.0.0.1:19000 --peer-listen 0.0.0.0:19000" \
RADICLE_UPSTREAM_UI_PROXY_ADDRESS="localhost:19000" \
yarn start

# And then in a separate shell, launch the second instance.

RAD_HOME="/Users/rudolfs/work/20000" \
RADICLE_UPSTREAM_PROXY_ARGS="--http-listen 127.0.0.1:20000 --peer-listen 0.0.0.0:20000" \
RADICLE_UPSTREAM_UI_PROXY_ADDRESS="localhost:20000" \
yarn start
```

You can also let the the OS choose a free peer port by setting it to:
`--peer-listen 0.0.0.0:0`. And if you don't need completely isolated state,
then you can use `RAD_PROFILE` instead of `RAD_HOME`, but be aware that
Electron state will be shared by all instances.


### Running on Windows (experimental)

There might be [issues due to long file paths on windows][lf]. A workaround
for this is to set `RAD_HOME` to a root folder, for example:

`$env:RAD_HOME="C:\rad"`.

To try out Upstream on Windows, you can use a [free VM][fv] provided by
Microsoft.


### Building an Upstream package for your platform

You can build and package Upstream with: `yarn dist`. The generated package
will be in: `dist/` as `radicle-upstream-X.X.X.{dmg|AppImage|exe}`.

On Windows you can do `yarn run dist:win:static` to [pre-compile][pc]
`vcruntime140.dll` to avoid the need to install Visual C++ Redistributable on
the target computer.


#### Apple notarization

To allow macOS Gatekeeper [to recognise][so] our Upstream packages as genuine,
which allows users to install and open Upstream without unnecessary
[security warnings][sw], we have to [sign and notarize][sn] our macOS packages.

This notarization step is automated using our custom macOS build host for
releases.

However, if the build host is not available, it is possible to set up and
perform notarization locally on Apple hardware.

For this we need:
  - a paid Apple developer account registered to Monadic
  - an Apple ID token for allowing the notarization script to run on behalf of
    our developer account
    - [Account Manage][ma] -> APP-SPECIFIC PASSWORDS -> Generate password…
  - a valid "Developer ID Application" certificate
    - [Certificates Add][ca] -> Developer ID Application
      **Note:** this can only be created via the company account holder

Once you've created the _Developer ID Application_ certificate, download it
locally and add it to your keychain by double clicking on the file.

Before building a notarized DMG, make sure you're connected to the internet and
then run:

```sh
git checkout vX.X.X
CSC_NAME="Monadic GmbH (XXXXXXXXXX)" \
APPLE_ID="XXXXXXX@monadic.xyz" \
APPLE_ID_PASSWORD="XXXX-XXXX-XXXX-XXXX" \
NOTARIZE=true \
yarn dist
```

Don't forget to replace the `X` in the template with the real values:
  - `vX.X.X` is the version you'd like to build and notarize
  - `CSC_NAME` is the "Developer ID Application" certificate ID
  - `APPLE_ID` your Apple account ID
  - `APPLE_ID_PASSWORD` your Apple account token that you generated in the
    steps above


### Scripts

To get a list of all available script commands, run: `yarn run`.

**Note:** Scripts marked with `_private` are not meant to be executed from the
the CLI, they're only to be used by other scripts.

Here's a list of all scripts that are intended for developer use:

    yarn start                  # Start Upstream with hot-UI-code-reload

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


### Styling

The main entry point of the electron renderer is `public/index.html`. This is
the file where any global styling which is not managed by Svelte should be
imported.

To avoid extra wrappers for positioning and spacing, and to allow style
overrides, components expose a `style` prop:

```html
  <Component style="margin-right: 24px"/>
```


### Typography

The design system provides a constrained set of typographic styles. This
consists of a set of styled headers, a set of styled paragraphs and a set of
modifiers. These also overlap with the components we have in our design system
in Figma, where the design of the app exists. All classes are prefixed with
`typo-` so this might be helpful if you have any autocomplete in your editor.

For the headers you can just use `<h1>` up to `<h5>`, if you want to apply the
same styles to other html elements you can use the matching classes
`typo-header-1` to `typo-header-5` (use `<h1>` to `<h5>` where you can).

For text we you can use the classes that start with `typo-text`. These come
in 2 sizes, the normal one and `typo-text-small`. Check out
[typography.css](./public/typography.css) to get an idea of the possible
combinations. All the ones we're using in Figma are represented here.

The modifiers give us some flexibility and allow us to create classes for
certain css functionality we use over and over. Such as,
`typo-overflow-ellipsis` and `typo-all-caps`. These should be self-explanatory.

We also added a set of modifiers that allow you to add the font-family as a
class where you need it, here again we would recommend not doing that as most
styles should fit into one of the two categories above.

The only place in the app where we're not using this is in `<Markdown />`,
since the library we use doesn't allow us to overwrite the styles without using
global declarations. If you have any questions or improvements, open an issue
and we're happy to help you along.

### Colors

The design system supports multiple color palettes via themes which can be
changed in the Settings screen.

Throughout the codebase we use only CSS variables. Raw color codes should not
be used so changes to global styling can be applied in one central place:
`public/colors.css`.

Read more about the colors used in Upstream in the [Color System post][cg].


## Proxy

All of Upstream's business logic tying together the radicle code collaboration
is provided to the UI via an HTTP API by a rust binary called `radicle-proxy`.
It uses [warp][wa] to serve a RESTish JSON API.

For dependency management and execution of common tasks we use [Cargo][co]. To
get up to speed with common functionality and manifest file intricacies consult
the exhaustive [Cargo Book][cb].

The proxy binary's lifecycle is managed by the main renderer of the UI in:
`native/main.ts`. When running `yarn dist` it is bundled together into an
application package by [electron-builder][eb].


### Running the proxy in stand-alone mode

To be able to build the proxy first install all required dependencies from the
[Running Upstream](#running-upstream) section.

To start the proxy binary, run `cargo run`.


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


### File structure

The API exposes the application's domain logic. Therefore we try to treat it as
a thin layer exposing well-typed entities. The heavy lifting is done in the
modules named after the protocols we consume - [radicle-link][rl] through it
[radicle-surf][rs], for code collaboration. By isolating concerns this way, we
hope to enable ease-of-contribution to downstream teams. Empowering them to
reflect changes in their public APIs easily with code contributions to
Upstream.


## CI setup

We run CI builds with [Buildkite][bk] and [Github Actions][ga]. Buildkite builds
all branch pushes to the Github repository. Github actions builds pull requests
on Github.

When tests pass, the build process uploads the Upstream binary as a build
artifact. If the UI end-to-end tests fail, screenshots of the failing tests are
uploaded instead of the binary.

### Docker image updates

We use a Docker image with all system dependencies pre-installed to reduce
build times. If you need to update this image, proceed as follows:

1. Install [Google Cloud SDK][gc].

2. Authenticate with Google Cloud: `gcloud auth configure-docker`, pick
   `radicle-upstream` when asked for which project to use.

3. Prepare a new docker image with all the necessary dependencies by editing:
   `ci/Dockerfile`.

4. Get the current image version from `pipeline.yaml` and build a new Docker
   image (remember to bump the version):

    ```sh
    cd ci
    docker build . -t gcr.io/radicle-upstream/radicle-upstream-ci:15
    ```

5. Push the new image version to Google Cloud:

   `docker push gcr.io/radicle-upstream/radicle-upstream-ci:15`

6. Update the image version in `.buildkite/pipeline.yaml`:

   ```yaml
   DOCKER_IMAGE: 'gcr.io/radicle-upstream/radicle-upstream-ci:15'
   ```

7. Update the image version in `.github/workflows/build.yaml`

8. Commit changes to `Dockerfile` and `pipeline.yaml`. Pushing the changes will
   create a new branch and build the updated image.

## Updating NPM dependencies

1. Run `yarn upgrade-interactive`.

2. Select newer versions if appropriate
   * For major version upgrades review the changelog to assess the impact of
     breaking changes. The assessment should be included in the commit message.
   * The following packages cannot be upgraded to the next major version because
     we don’t support ES modules yet. See [#2227](https://github.com/radicle-dev/radicle-upstream/issues/2227).
     If possible, do a minor version upgrade.
     * `node-fetch` and `@types/node`
     * `exit-hook`
     * `strip-ansi`
   * Don’t update `radicle-contracts`.
   * Don’t do a major upgrade of `@types/node`.
   * `electron-builder` shows `^22.13.1` as the latest version. This is
     not the latest version. Choose the version from the “Range” column.

3. Update transitive dependencies: Remove `yarn.lock` and run `yarn install`.

4. Run `yarn dedupe`.

## Releases

This section describes how to release a new version of Upstream.

### Prerequisites

* [`gcloud`][gc] to upload artifacts. You need to ask for access to the
  `radicle-upstream-releases` storage bucket.
* [`hub`][hb] version >= 2.14 to interact with GitHub’s API. See [its
  documentation][hub-config] on how to configure access
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
    1. Wait for CI of the release candidate PR to pass.
    2. Publish the CI artifacts as release candidate binaries.

       ```bash
       ./scripts/release.ts publish-rc-binaries
       ```

    3. Create QA issues for the release that link to the release candidate
       binaries.

       ```bash
       ./scripts/release.ts create-qa-issues
       ```

    4. Test the release by walking through the QA issues.
    5. (Optional) To fix bugs, create a PR with the fixes based on the release
       candidate branch. Once it has been approved, squash merge it into the
       release candidate branch (see [“Merging Pull Requests"][merging-prs]).
       Then restart the “Test the release” step. (Skip creating a QA
       issue in 2.3.)
    6. Close the QA issues.

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

4. Finish the release by merging the release candidate branch into master.

    ```bash
    git checkout master
    git pull --ff-only
    git merge release-candidate/vX.Y.Z --signoff
    git push
    ```

    Merging may produce a merge commit on `master` instead of fast-forwarding.
    This is ok for release candidate branches.

[an]: #apple-notarization
[bk]: https://buildkite.com/monadic/radicle-upstream
[br]: https://brew.sh
[bs]: https://docs.brew.sh/How-To-Open-a-Homebrew-Pull-Request#submit-a-new-version-of-an-existing-formula
[ca]: https://developer.apple.com/account/resources/certificates/add
[cb]: https://doc.rust-lang.org/cargo/
[cc]: https://www.conventionalcommits.org/en/v1.0.0
[cg]: https://radicle.community/t/color-system/166
[ch]: CHANGELOG.md
[cl]: https://gist.github.com/Rich-Harris/0f910048478c2a6505d1c32185b61934
[co]: https://github.com/rust-lang/cargo
[cs]: https://help.github.com/en/github/authenticating-to-github/signing-commits
[do]: #docker-image-updates
[eb]: https://github.com/electron-userland/electron-builder
[el]: https://www.electronjs.org
[es]: https://eslint.org
[fv]: https://developer.microsoft.com/en-us/windows/downloads/virtual-machines
[ga]: https://docs.github.com/en/actions
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[gg]: https://cloud.google.com/storage/docs/gsutil_install
[gp]: https://console.cloud.google.com/storage/browser/builds.radicle.xyz/releases/radicle-upstream
[gt]: https://github.com/settings/tokens
[hb]: https://github.com/github/hub
[hub-config]: https://hub.github.com/hub.1.html#configuration
[hu]: https://github.com/typicode/husky
[lf]: https://github.com/libgit2/libgit2/issues/3053
[ls]: https://github.com/okonet/lint-staged
[ma]: https://appleid.apple.com/account/manage
[merging-prs]: https://github.com/radicle-dev/radicle-decisions/blob/master/proposals/0003.md#merging-pull-requests
[on]: https://docs.cypress.io/guides/core-concepts/writing-and-organizing-tests.html#Excluding-and-Including-Tests
[pc]: https://github.com/libgit2/libgit2/issues/3053
[pr]: https://prettier.io
[qa]: qa.md
[rd]: https://github.com/radicle-dev/radicle.xyz/blob/master/pages/downloads.html.mustache
[rl]: https://github.com/radicle-dev/radicle-link
[rs]: https://github.com/radicle-dev/radicle-surf/
[rt]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[se]: https://svelte.dev
[sn]: https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution
[so]: https://support.apple.com/en-us/HT202491
[sv]: https://github.com/conventional-changelog/standard-version
[sw]: https://support.apple.com/en-gb/guide/mac-help/mh40616/mac
[tp]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[wa]: https://github.com/seanmonstar/warp
[commit-sign-gpg]: https://docs.github.com/en/github/authenticating-to-github/managing-commit-signature-verification/signing-commits
[doc-rfc]: https://github.com/radicle-dev/radicle-decisions/blob/master/proposals/0003.md#merging-pull-requests
