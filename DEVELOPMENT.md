# How do we work?

Our workflow is to put changes in feature branches which we submit for review
on GitHub as pull requests. Ideally a pull request is small and changes only
one aspect of the code at a time. After a pull request is reviewed by at least
one peer and passes all tests, it can be squash-merged into master.

💡 *We require all commits to be signed for a branch to be merged into
master. Learn more on setting up [commit signing][cs].*

To automate our release process as much as possible we're using
[Standard Version][sv]. Commits on master should be formatted according to
the [conventional commits specification][cc].

Here are a couple of examples:
```
  fix: fix clippy on CI (#430)
  refactor(ui): improve cypress spec reliability (#429)
  style(ui): icon refresh (#411)
  chore(release): 0.0.11 (#417)
  test(ui): add missing project creation specs (#404)
  feat(proxy): improve session (#380)
```

When a release is performed, a section in [CHANGELOG.md][ch] is automatically
generated with all the changes from these commit messages.


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


### Running Upstream

You'll have to install some external dependencies to be able to compile the
proxy as well as the UI.

On macOS:
```
xcode-select --install
sudo xcodebuild -license
brew install yarn pkgconfig nettle
```

On Linux:
  - [Autoconf](https://www.gnu.org/software/autoconf)
  - [Clang](https://clang.llvm.org)
  - [Git](https://git-scm.com)
  - [GMP](https://gmplib.org)
  - [GNU M4](https://www.gnu.org/software/m4)
  - [Nettle](http://www.lysator.liu.se/~nisse/nettle)
  - [OpenSSL](https://www.openssl.org)
  - [Yarn](https://yarnpkg.com)

1. Get Upstream: `git clone git@github.com:radicle-dev/radicle-upstream.git`.
2. Install dependencies: `cd radicle-upstream && yarn install`.
3. Start Upstream in development mode: `yarn start`.


### Running tests

Before running UI end-to-end tests locally you'll need to check out the latest
test fixtures which are included in this repository as a git submodule:

```sh
git submodule update --init --remote
git submodule foreach "git fetch --all"
```

💡 *You'll have to run the previous commands every time there are any updates
to the test fixture repository.*

We use [Cypress](https://www.cypress.io/) for integration tests and [Jest](jestjs.io) for unit tests. You can find integration tests in the `cypress/` directory and unit tests next to the modules they correspond to.

- To run all ui tests call: `yarn test`.
- To troubleshoot integration tests via the Cypress GUI, run: `yarn test:integration:debug`.
- To isolate a single integration test for debugging purposes, use the [`.only` method][on].
- To develop unit tests in watch mode, run: `yarn test:unit:watch`


### Building an Upstream package for your platform

You can build and package Upstream with: `yarn dist`. The generated package
will be in: `dist/` as `radicle-upstream-X.X.X.{dmg|AppImage|snap}`.


### Scripts

To get a list of all available script commands, run: `yarn run`.
Here is a list of the most commonly used ones:

```sh
yarn start                  # Start Upstream in development mode

yarn test                   # Run all ui tests
yarn test:integration       # Run only integration tests
yarn test:unit              # Run only unit tests
yarn test:integration:debug # Show the Cypress GUI, handy for visual debugging
yarn test:unit:watch        # Run Jest tests in watch mode

yarn dist                   # Bundles Upstream into an installable package

yarn release                # Start a two-step process to cut a new release, for more
                            # details have a look at ../DEVELOPMENT.md

yarn prettier:check         # Check UI code formatting
yarn prettier:write         # Auto-format UI code
yarn lint                   # Check UI code for linting errors
```


### Design System

The overall look of Upstream is governed by a style guide which is continuously
being improved and extended. This style guide is translated into code forming
the design system. The design system contains all design primitives which, in
turn, can be composed to create rich user experiences.

Most of the components defined by the design system can be conveniently seen on
one page within Upstream by pressing <kbd>shift</kbd> + <kbd>D</kbd>. This will
bring up the Design System Guide modal.

The purpose of the Design System Guide is to showcase all available primitives
and components. Having them all on a single screen allows us to see how changes
to components affect all variations at a glance. Therefore newly created
components should always be added to the Guide, explaining all the different
variations and use cases.


#### File structure

In Svelte everything is a component, so to be able to build a complex
application and still be able to navigate the code and make changes quickly, we
organize our components in groups defined by use-case, re-usability and
complexity. Currently you'll find the following types of components in the
`DesignSystem` directory:

  - `Primitive`: simple, yet highly reusable components like typography,
    buttons, form elements, spacing, positioning and other utilities.

    Components of this type are usually just wrappers around standard HTML
    elements with custom styling.

    There are currently two ways of organizing primitives:

      - as all-in-one components where the type of the component is passed down
        via a `variant` prop. This is for components which have a very similar
        markup, but whose styling differs across variants.  Examples in this
        category are: buttons, typography and positioning helpers.

      - as namespaced components, where the component markup is very different
        across variants, for example: form elements and icons.

    To decide which way to write a new primitive component, start by looking at
    how it's going to be used in code and then optimise for ergonomics.

    All public primitives are exported via a central `index.js` file, which
    makes consumption straightforward:

    ```html
    <script>
      import { Button, Title, Icon, Input } from "../DesignSystem/Primitive";
    </script>

    <Icon.Home />
    <Button variant="secondary">OK</Button>
    ```

  - `Component`: reusable low-to-high complexity components and layouts.

    Sub-folders in `DesignSystem/Component` should only be created for breaking
    up larger components into smaller fragments. If a component is broken up in
    fragments, make sure to only export the component which is intended for
    public use.

    ```html
    <script>
      import { Placeholder, Rad } from "../DesignSystem/Component";
    </script>

    <Placeholder style="width: 300px; height: 100px" />
    <Rad amount="200" />
    ```

Next to `DesignSystem`, you'll find a directory called `Screens`. Screens bring
together components from the Design System forming what a user in the UI sees
as a whole screen. More complex screens, similar to components, can be broken
down into multiple fragments. In this case the screen will contain data
fetching and routing logic for the fragments. Fragments should be placed in a
directory named after the screen, like so:

```sh
.
├── RegisterProject                    # fragment directory
│   ├── ConfirmTransactionStep.svelte
│   ├── PickNameStep.svelte
│   ├── PickWalletStep.svelte
│   └── TransactionSummaryStep.svelte
└── RegisterProject.svelte             # screen
```

When multiple screens share the same layout it should be extracted into a
separate component. Layout components are suffixed with "Layout":
`DesignSystem/Components/ModalLayout.svelt`.

Finally, our file and directory naming rules are as follows:

  - Svelte components and directories containing components - PascalCase;
  - everything else, including `*.js` files and folders - camelCase;
  - all folders in `/ui` should be named in singular form as they represent a
    type, not content.


#### Styling

The main entry point of the electron renderer is `public/index.html`. This is
the file where any global styling which is not managed by Svelte should be
imported.

To avoid extra wrappers for positioning and spacing, and to allow style
overrides, components expose a `style` prop:

```html
  <Rad amount={200} style="margin-right: 24px" size="big" />
```

For very common alignment cases we have a helper primitive called `<Flex>`.
A `<Flex>` primitive offers two ways of aligning its contents:

  - via the `align` prop;
  - or by using slots.

For when there is only one element to align, use the `align` prop, otherwise
use slots:

```html
<Flex align="left">
  <h1>Issues</h1>
</Flex>

<Flex style="margin-top: 48px;">
  <div slot="left">
    <Button style="margin-right: 24px">
      Back
    </Button>
  </div>

  <div slot="right">
    <Button variant="primary">
      Pay
    </Button>
  </div>
</Flex>
```


#### Typography

The design system provides a constrained set of typographic styles. This consists of a set of styled headers, a set of styled paragraphs and a set of modifiers. These also overlap with the components we have in our design system in Figma, where the design of the app exists. All classes are prefixed with `typo-` so this might be helpful if you have any autocomplete in your editor.

For the headers you can just use `<h1>` up to `<h5>`, if you want to apply the same styles to other html elements you can use the matching classes `typo-header-1` to `typo-header-5` (use `<h1>` to `<h5>` where you can).

For text we recommend using a `<p>` wherever they are used in the Figma files. There are matching modifiers if you're looking for a bold paragraph, `<p class="typo-bold">`, or any of the other ones we have. Check out [typography.css](./public/typography.css) to get an idea of the possible combinations. All the ones we're using in figma are represented here. If you need to use another html element than `<p>`, there are classes that match the paragraph styling, e.g.: `<p class="typo-bold">` -> `<span class="typo-text-bold">`

The modifiers are to give us some flexibility and allow us to create classes for certain css functionality we use over and over. Such as, `typo-overflow-ellipses` and `typo-all-caps`. These should be self-explanatory.

We also added a set of modifiers that allow you to add the font-family as a class where you need it, here again we would recommend not doing that as most styles should fit into one of the two categories above.

The only place in the app we're not using this is in `<Markdown />`, since the library we use doesn't allow us to overwrite the styles without using global declarations. If you have any questions or improvements, open an issue and we're happy to help you along.

#### Colors

The design system supports multiple color palettes via themes which can be
changed in the Settings screen.

Throughout the codebase we use only CSS variables. Raw color codes should not
be used so changes to global styling can be applied in one central place:
`public/colors.css`.

Read more about the colors used in Upstream in the [Color System post][cg].


## Proxy

All of Upstream's business logic tying together the radicle code collaboration
and registry protocols is provided to the UI via an HTTP API by a rust binary
called the proxy. It uses [warp][wa] to serve a RESTish JSON API.

For dependency management and execution of common tasks we use [Cargo][co]. To
get up to speed with common functionality and manifest file intricacies consult
the exhaustive [Cargo Book][cb].

The proxy binary's lifecycle is managed by the main renderer of the UI in:
`native/main.js`. When running `yarn dist` it is bundled together into an
application package by [electron-builder][eb].


### Running the proxy in stand-alone mode

To be able to build the proxy first install all required dependencies from the
[Running Upstream](#running-upstream) section.

To start the proxy binary, run: `cd proxy && cargo run -- --registry=emulator`.
After that the API docs are served under `http://127.0.0.1:8080/docs`.

### Registry chains

The proxy connects to a registry node to get data from a registry chain. The
node host is configured via the `--registry` option.

The `--registry` option accepts a special value `emulator`. If set the proxy
simulates the registry chain in memory on the node.

We provide the following shortcuts.

* `yarn run proxy:start`. Runs the proxy with the emulator.
* `yarn run proxy:start:ffnet` Connects to a node in the cloud that runs the
  ffnet chain.
* `yarn run proxy:start:devnet` Connects to a node in the cloud that runs the
  devnet chain. This chain is frequently reset and contains the latest master
  version.


### Testing

The proxy and UI share the same test fixtures, if you haven't done it already,
set up the test fixtures like so:

```sh
git submodule update --init --remote
git submodule foreach "git fetch --all"
```

💡 *You'll have to run the submodule commands every time there are any updates
to the test fixture repository.*

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
[radicle-surf][rs], for code collaboration and [radicle-registry][rr] for
global unique entries for users, projects and organisations. By isolating
concerns this way, we hope to enable ease-of-contribution to downstream teams.
Empowering them to reflect changes in their public APIs easily with code
contributions to Upstream.


## CI setup

Our CI infrastructure runs on [Buildkite][bk]. The build process is run for
every commit which is pushed to GitHub. When tests pass, the build process
uploads the Upstream binary as a build artifact. If the UI end-to-end tests
fail, screenshots of the failing tests are uploaded instead of the binary.

All relevant configuration can be found here:

```sh
radicle-upstream/.buildkite
.
├── Dockerfile
├── pipeline.yaml
└── run.sh
```


### Docker image updates

We use a Docker image with all system dependencies pre-installed to reduce
build times. If you need to update this image, proceed as follows:

1. Install [Google Cloud SDK][gc].

2. Authenticate with Google Cloud: `gcloud auth configure-docker`, pick
   `[1] opensourcecoin` when asked for which project to use.

3. Prepare a new docker image with all the necessary dependencies by editing:
   `.buildkite/Dockerfile`.

4. Get the current image version from `pipeline.yaml` and build a new Docker
   image (remember to bump the version):

    ```sh
    cd .buildkite
    docker build . -t gcr.io/opensourcecoin/radicle-upstream:0.2.1
    ```

5. Push the new image version to Google Cloud:

   `docker push gcr.io/opensourcecoin/radicle-upstream:0.2.1`

6. Update the image version in `pipeline.yaml`:

   ```yaml
   DOCKER_IMAGE: 'gcr.io/opensourcecoin/radicle-upstream:0.2.1'
   ```

7. Commit changes to `Dockerfile` and `pipeline.yaml`. Pushing the changes will
   create a new branch and build the updated image.


## Releases

Before you begin: install the [`hub`][hb] cli tool. We use `hub` in our release
automation script to create a pull-request off of a release branch and later
for merging this branch into master and closing the pull-request.

You'll also have to authenticate the hub cli once by running any command that
does a request to GitHub, like so: `hub api`.

To perform a release run: `git checkout master && yarn release` and follow the
instructions.

Once the release PR branch is merged into master a build will be triggered on
Buildkite, this will build Upstream for both Linux and macOS. When the build
has completed you can download binaries for your platform [here][ar].

This is what a typical release looks like:

```sh
$ git checkout master
$ yarn release

Cutting release v0.0.11:

  ✔ git checkout master
  ✔ git branch release-v0.0.11 && git checkout release-v0.0.11
  ✔ yarn run standard-version
  ✔ git push origin release-v0.0.11
  ✔ hub pull-request -p --no-edit

Now ask a peer to review the following pull request,
but don't merge it just yet:

  👉 https://github.com/radicle-upstream/pull/417

To merge the pull request and finalize this release run:

  👉 yarn release:finalize v0.0.11 417


$ yarn release:finalize v0.0.11 417

Finalizing release v0.0.11:

  ✔ hub api -XPUT "repos/radicle-dev/radicle-upstream/pulls/417/merge"
  ✔ git checkout master && git pull
  ✔ git tag v0.0.11 ed968ee61ec30a18653b621f645a6abe354d2d16
  ✔ git push --tags

Release v0.0.11 successfully completed! 👏 🎉 🚀
```



[ar]: https://buildkite.com/monadic/radicle-upstream/builds?branch=master
[bk]: https://buildkite.com/monadic/radicle-upstream
[cb]: https://doc.rust-lang.org/cargo/
[cc]: https://www.conventionalcommits.org/en/v1.0.0
[cg]: https://radicle.community/t/color-system/166
[ch]: CHANGELOG.md
[cl]: https://gist.github.com/Rich-Harris/0f910048478c2a6505d1c32185b61934
[co]: https://github.com/rust-lang/cargo
[cs]: https://help.github.com/en/github/authenticating-to-github/signing-commits
[eb]: https://github.com/electron-userland/electron-builder
[el]: https://www.electronjs.org
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[hb]: https://github.com/github/hub
[hu]: https://github.com/typicode/husky
[ls]: https://github.com/okonet/lint-staged
[on]: https://docs.cypress.io/guides/core-concepts/writing-and-organizing-tests.html#Excluding-and-Including-Tests
[pr]: https://prettier.io
[rl]: https://github.com/radicle-dev/radicle-link
[rr]: https://github.com/radicle-dev/radicle-registry
[rs]: https://github.com/radicle-dev/radicle-surf/
[rt]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[se]: https://svelte.dev
[sv]: https://github.com/conventional-changelog/standard-version
[tp]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[wa]: https://github.com/seanmonstar/warp
