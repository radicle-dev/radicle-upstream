# How do we work?

Our current workflow is to put our changes in feature branches which get
submitted for review on GitHub as pull requests. Ideally a pull request is
small and changes only one aspect of the code at a time. After a pull request
is reviewed by at least one peer and passes all tests, it can be squash-merged
into master.

💡 *We require all commits to be signed for a branch to be merged into
master. Learn more on setting up commit signing [here][cs].*

To automate our release process as much as possible we're using
[Standard Version][sv] and commits on master should be formatted according to
the [conventional commits specification][cc].

Here are a couple of examples:
```
  chore: remove leftover error mod reference (#74)
  fix: improve project creation validations (#76)
  feat: project creation (#70)
```

The pull request ref goes in brackets at the end of the subject.
Generally we also follow [good commit message hygene][tp].

Work on the UI and proxy can happen in parallel. However for this to work, the
teams working on both code bases have to agree on a common API. The definitive
source-of-truth for this API is the proxy source code.

To change or extend the API, first open an issue on GitHub, discuss what is
needed and then propose proxy code changes which implement the API.

We don't keep around a copy of an SDL schema file as they tend to get outdated
quickly and it is way easier to explore the API via introspection. You can read
more on that in the [GraphiQL Explorer](#graphiql-explorer) section below.

If you need the schema to guide you writing new queries or mutations, you can
extract the latest schema version from the proxy via `yarn generate:schema`.
It'll be saved into `./schema.gql`.


## UI

The UI is written in JavaScript, [Svelte][se] is our [component language][cl]
of choice and [Electron][el] wraps it all together into a native desktop
experience. The UI code is respectively split up into [`/native`] and [`/ui`].

For dependency management and script execution we use `yarn`. Code formatting
is dictated by [prettier][pr] and linting is provided by [eslint][es]. Both
linting and formatting are enforced locally on a pre-commit basis with
[husky][hu] and [lint-staged][ls].

Additionally we run the same checks as separate build steps on our CI, just to
make sure only properly formatted and linted code lands into master.


### Running Upstream

1. Get the code: `git clone git@github.com:radicle-dev/radicle-upstream.git`.
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

- To run tests call: `yarn test`.
- To troubleshoot tests via the Cypress GUI, run: `yarn test:debug`.
- To isolate a single test for debugging purposes, use the `.only` method like
  so:

```javascript
context("a bazillion tests in this context", () => {
  it("does one thing", () => {
    ...
  });

  it.only("does another thing", () => {
    ...
  });

  ...
});
```

Then, to execute just this single test, fire up the Cypress GUI:
`yarn test:debug` and choose the respective `.spec.js` file from the list.

💡 *Don't forget to remove all `.only` methods from the tests before
committing changes, otherwise these tests will be skipped on CI.*


### GraphiQL explorer

The introspective and self-docummenting nature of GraphQL allows us to easily
explore our API by querying the proxy. For better developer ergonomics we
include a [static version][gs] of [GraphiQL-explorer][ge].

To see this in action:

1. Start Upstream in development mode: `yarn start`.
2. Launch GraphiQL from a separate shell: `yarn graphiql`
   or directly navigate to http://localhost:5000 in your browser.

To see all the possible queries and query parameters, open the GraphiQL
explorer sidebar simply by clicking the `Explorer` button. More extensive API
documentation is provided in the `Docs` sidebar. You can navigate through all
the queries and types by clicking on them.

Annotations for queries, mutations, types and fields propagate from the proxy
source into the `Docs` section automatically.


### Design System

The overall look of Upstream is governed by a style guide which is continuously
being improved and extended. This style guide is translated into code forming
the design system. The design system contains all design primitives which, in
turn, can be composed to create rich user experiences.

Most of the components defined by the design system can be conveniently seen on
one page within Upstream by pressing <kbd>shift</kbd> + <kbd>D</kbd>. This will
bring up the Design System Guide modal. To close the modal, hit <kbd>ESC</kbd>.

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
    <Title variant="huge">Radicle</Title>
    ```

  - `Component`: reusable low-to-high complexity components and layouts.

    Sub-folders in `DesignSystem/Component` should only be created for breaking
    up larger components into smaller fragments. If a component is broken up in
    fragments, make sure to only export the component which is intended for
    public use.

    ```html
    <script>
      import { Avatar, Placeholder, Rad } from "../DesignSystem/Component";
    </script>

    <Avatar size="big" title="My name" />
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

```sh
.
├── App.svelte                     # Root component
├── DesignSystem
│   ├── Component
│   │   ├── Avatar.svelte          # Simple component
│   │   ├── Sidebar                # Folder containing Sidebar fragments
│   │   │   ├── Avatar.svelte      # These are private to Sidebar.svelte
│   │   │   ├── Item.svelte        # and should not be exported via index.js
│   │   │   └── Tooltip.svelte
│   │   ├── Sidebar.svelte
│   │   ├── SidebarLayout.svelte   # Layout containing a Sidebar component
│   │   └── index.js               # Exports components for public use
│   └── Primitive
│       ├── Button.svelte          # Single-file component
│       ├── Icon                   # Name-spaced components
│       │   ├── Branch.svelte
│       │   └── index.js
│       └── index.js               # Exports primitives for public use
├── Screen
│   ├── Profile.svelte             # Simple screen
│   ├── Project                    # Project screen fragments
│   │   ├── Feed.svelte
│   │   └── Source.svelte
│   ├── Project.svelte             # Data fetching and routing for Project fragments
│   └── Wallet.svelte
├── config.js                      # UI Configuration constants
├── index.js                       # UI entry-point, loaded from main renderer
├── lib                            # Reusable utilities and other business
│   ├── hash.js                    # logic which doesn't fit in components
│   ├── path.js
│   └── type.js
└── store                          # Svelte stores grouped by use-case
    ├── notification.js
    ├── project.js
    └── sourceBrowser.js
```

#### State

Shared state used by multiple Screen fragments or components to build a more
complex feature should be extracted into a separate store file. A store file
contains one or more Svelte stores and functions which act on these stores.

```sh
stores
├── notification.js
├── project.js
└── sourceBrowser.js
```


#### CSS

The main entry point of the electron renderer is `public/index.html`. This is
the file where any global styling not managed by Svelte should be imported:

```html
<link rel="stylesheet" href="reset.css" />       <!-- style resets -->
<link rel="stylesheet" href="colors.css" />      <!-- color CSS variables -->
<link rel="stylesheet" href="typography.css" />  <!-- font-face setup and typography CSS variables -->
<link rel="stylesheet" href="global.css" />      <!-- global CSS rules -->
<link rel="stylesheet" href="bundle.css" />      <!-- compiled Svelte component CSS -->
```

To avoid extra wrappers for positioning and spacing, and to allow style
overrides, components expose a `style` prop:

```html
  <Rad amount={200} style="margin-right: 24px" size="big" />

  <Input.Dropdown
    items={localBranches}
    bind:value={defaultBranch}
    style="min-width: 240px; --focus-outline-color: var(--color-pink)" />
```

For very common alignment cases we have a helper primitive called `<Flex>`.
A `<Flex>` primitive offers two ways of aligning its contents:

  - via the `align` prop;
  - or by using slots.

For when there is only one element to align, use the `align` prop, otherwise
use slots:

```html
<Flex align="left">
  <Title variant="big">Issues</Title>
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


#### Colors

Colors amongst other design system tokens are stored in the `tokens` folder.
If a color gets added, removed or changed in the style guide, we have to make
changes to `tokens/colors.json` accordingly.

Entries in `colors.json` have the following shape:
```javascript
{
  "name": "pink",    // Color name as defined in style guide
  "hex": "#e074cb",  // Hex color code

  "shades": [10],    // Array of auto-generated color shades,
                     // possible values range from 5 to 95,
                     // values should be kept to multiples of 5.

  "tints": [10],     // Array of auto-generated color tints,
                     // possible values range from 5 to 95,
                     // values should be kept to multiples of 5.
}
```

When `tokens/colors.json` is changed, we have to re-generate all tints and
shades via: `yarn generate:colors`. This will update the global color CSS
variables in `public/colors.css`. Changes to both files should be committed.

Throughout the codebase we use only CSS variables. Raw color codes should not
be used so changes to global styling can be applied in a central place.

```html
<style>
  button {
    background-color: var(--color-black);
    border-color: var(--color-orange-tint-65);
  }
</style>

<button>
</button>
```


### Building an Upstream package for your platform

You can build and package Upstream with: `yarn dist`. The generated package
will be in: `dist/` as `radicle-upstream-X.X.X.{dmg|AppImage|snap}`.

## Scripts

To get a list of all available script commands, run: `yarn run`.
Here is a list of the most commonly used ones:

```sh
yarn start            # Start Upstream in development mode

yarn test             # Run Cypress end-to-end tests
yarn test:debug       # Show the Cypress GUI, handy for visual debugging

yarn dist             # Bundles Upstream into an installable package

yarn generate:colors  # Update color CSS variables in public/colors.css from
                      # colors.json

yarn generate:schema  # Start proxy and save the latest GraphQL schema
                      # in ./schema.gql

yarn graphiql         # Open GraphiQL with explorer extension in browser

yarn release          # Start a two-step process to cut a new release, for more
                      # details have a look at ../DEVELOPMENT.md

yarn prettier:check   # Check UI code formatting
yarn prettier:write   # Auto-format UI code
yarn lint             # Check UI code for linting errors
```


## Proxy

All of Upstream's business logic tying together the radicle code collaboration and
registry protocols is provided to the UI via [GraphQL][gq] by a rust binary called
the proxy. It uses [warp][wa] to serve a [Juniper][ju] powered API providing all necessary
queries and mutations.

For dependency management and execution of common tasks we use [Cargo][co]. To
get up to speed with common functionality and manifest file intricacies consult
the exhaustive [Cargo Book][cb].

The proxy binary's lifecycle is managed by the main renderer of the UI in:
`native/main.js`. When running `yarn dist` it is bundled together into an
application package by [electron-builder][eb].


### Running the proxy in stand-alone mode

To start the proxy binary, run: `cd proxy && cargo run -- --registry=emulator`.
After that the GraphQL API is served on `http://127.0.0.1:8080/graphql`.


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

We strive for two kinds of tests: classic unit tests contained in implementation
files and integration tests. The integration tests are meant to assert correctness
of the API provided by the proxy, these can be found under `proxy/tests`. To find
out where to place and how to lay out tests, check the Rust book [test chapter][rt].

### File structure

The GraphQL API exposes the application's domain logic. Therefore we try to treat
it as a thin layer exposing well-typed entities. The heavy lifting is done in the
modules named after the protocols we consume - [radicle-link][rl] through it
[radicle-surf][rs], for code collaboration and [radicle-registry][rr] for global
unique entries for users, projects and organisations. By isolating concerns this
way, we hope to enable ease-of-contribution to downstream teams. Empowering them
to reflect changes in their public APIs easily with code contributions to Upstream.

```
proxy/src/
├── coco.rs
├── env.rs
├── error.rs
├── graphql
│   ├── api.rs
│   ├── error.rs
│   ├── project.rs
│   └── schema.rs
├── graphql.rs
├── lib.rs
├── main.rs
└── registry.rs
```

## Scripts

To get a list of all available script commands, run: `yarn run`.
Here is a list of the most commonly used ones:

```sh
yarn start                # Start Upstream in development mode

yarn test                 # Run Cypress end-to-end tests
yarn test:debug           # Show the Cypress GUI, handy for visual debugging

yarn dist                 # Bundles Upstream into an installable package

yarn generate:colors      # Update color CSS variables in public/colors.css from
                          # colors.json

yarn graphiql             # Open GraphiQL with explorer extension in browser

yarn release              # Start a two-step process to cut a new release, for more
                          # details have a look at ../DEVELOPMENT.md

yarn prettier:check       # Check UI code formatting
yarn prettier:write       # Auto-format UI code
yarn lint                 # Check UI code for linting errors

yarn proxy:build          # Build the proxy binary
yarn proxy:build:release  # Build the release version of the proxy, stripped of debug symbols
yarn proxy:start          # Start only the proxy with its default configuration
yarn proxy:start:test     # Start the proxy in test mode, where state is isolated and lives in memory or temporary directories
```

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
    docker push gcr.io/opensourcecoin/radicle-upstream:0.2.1
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

  👉 https://github.com/rudolfs/test/pull/17

To merge the pull request and finalize this release run:

  👉 yarn release:finalize v0.0.11 17


$ yarn release:finalize v0.0.11 17

Finalizing release v0.0.11:

  ✔ hub api -XPUT "repos/radicle-dev/radicle-upstream/pulls/17/merge"
  ✔ git checkout master && git pull
  ✔ git tag v0.0.11 74369ee3c078bc3688f0b668cc94a36491271d52
  ✔ git push --tags

Release v0.0.11 successfully completed! 👏 🎉 🚀
```



[ar]: https://buildkite.com/monadic/radicle-upstream/builds?branch=master
[bk]: https://buildkite.com/monadic/radicle-upstream
[cb]: https://doc.rust-lang.org/cargo/
[cc]: https://www.conventionalcommits.org/en/v1.0.0
[cl]: https://gist.github.com/Rich-Harris/0f910048478c2a6505d1c32185b61934
[co]: https://github.com/rust-lang/cargo
[cs]: https://help.github.com/en/github/authenticating-to-github/signing-commits
[eb]: https://github.com/electron-userland/electron-builder
[el]: https://www.electronjs.org
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[ge]: https://github.com/OneGraph/graphiql-explorer
[gs]: https://github.com/OneGraph/graphiql-with-extensions
[gq]: https://graphql.org/
[hb]: https://github.com/github/hub
[hu]: https://github.com/typicode/husky
[ju]: https://github.com/graphql-rust/juniper
[ls]: https://github.com/okonet/lint-staged
[pr]: https://prettier.io
[rl]: https://github.com/radicle-dev/radicle-link
[rr]: https://github.com/radicle-dev/radicle-registry
[rs]: https://github.com/radicle-dev/radicle-surf/
[rt]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[se]: https://svelte.dev
[sv]: https://github.com/conventional-changelog/standard-version
[tp]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[wa]: https://github.com/seanmonstar/warp
