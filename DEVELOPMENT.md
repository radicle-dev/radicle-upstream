# Development

Our current workflow is to put our changes in feature branches which get
submitted for review on GitHub as pull requests. Ideally a pull request is
small and changes only one aspect of the code at a time. After a pull request
is reviewed by at least one peer and passes all tests, it can be squash-merged
into master.

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


## Work coordination

Work on the UI and proxy can happen in parallel. However for this to work the
teams working on both code bases have to agree on a common API. The definitive
source-of-truth for this API is the proxy source code.

To change or extend the API, first open an issue on GitHub, discuss what is
needed and then propose proxy code changes that implement the API.

We don't keep around a copy of an SDL schema file as they tend to get outdated
quickly and it is way easier to explore the API via introspection. See the next
section for more information on that.


### GraphiQL explorer

The introspective and self-docummenting nature of GraphQL allows us to easily
explore our API by querying the backend. For better developer ergonomics we
include a [static version][gs] of [GraphiQL-explorer][ge]. To see it in action:

1. Start the app in development mode: `yarn start`
2. Launch GraphiQL from a separate shell: `yarn graphiql`
   or directly navigate to http://localhost:5000 in your browser.

You can see all the possible queries and their parameters by opening the
GraphiQL explorer sidebar, for that simply click the `Explorer` button. More
extensive API documentation is provided in the `Docs` sidebar. You can navigate
through all the queries and types by clicking on them.

Annotations for queries, mutations, types and fields also get propagated from
the proxy source into the `Docs` section.


## UI

For dependency management and script execution we use `yarn`. Code formatting
is dictated by [prettier][pr] and enforced locally on a pre-commit basis with
[husky][hu] and [lint-staged][ls].


### Running the app

1. Get the code: `git clone git@github.com:radicle-dev/radicle-upstream.git`.
2. Install dependencies: `cd radicle-upstream && yarn install`.
3. Start the app in development mode: `yarn start`.


### Running tests

Before running tests locally you'll need to set up a test fixture repository:
`git submodule foreach "git fetch --all"`.

- to run tests do: `yarn test`
- to troubleshoot tests in the Cypress GUI, run: `yarn test:debug`
- to isolate a single test for debugging purposes, use the `.only` method:
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

Then to execute just that one test, fire up the Cypress GUI: `yarn test:debug`
and choose the respective `.spec.js` file from the list.

**Note**: don't forget to remove all `.only` methods from tests before
committing changes, otherwise the tests will be skipped on CI.


### Design System

The overall look of Upstream is governed by a style guide which is continuously
being improved and extended. This style guide is translated into code that
forms a design system. The design system contains all design primitives which
can be composed to create rich user experiences.

Most of the components defined by the design system can be conveniently seen on
one page within the app UI by pressing <kbd>shift</kbd> + <kbd>D</kbd>. This
will bring up the Design System Guide modal. To close the modal, hit
<kbd>ESC</kbd>.

The purpose of the Design System Guide is to showcase all available primitives
and components. Having them all on a single screen allows us to see how changes
to components affect all variations at a glance. Therefore newly created
components should be added to the Guide, explaining all the different
variations and use cases.


#### File structure

In Svelte everything is a component, so to be able to build a complex
application and still be able to navigate the code and make changes quickly, we
organize our components in groups defined by use-case, re-usability and
complexity. Currently you'll find the following groups in the `DesignSystem`
directory:

  - `Primitives`: simple, yet highly reusable components like
    typography, buttons, form elements, spacing, positioning and other
    utilities.

    Components in this group are usually just wrappers around standard HTML
    elements with custom styling.

    There are currently two ways of organizing primitives:

      - all-in-one components where the type of the component is passed down
        via a `variant` prop. This is for components which have a very
        similar markup, but whose styling differs across variants.
        Examples in this category are: buttons, typography and positioning
        helpers.

      - namespaced components, where the markup is very different across
        variants, like: form elements and icons.

    To figure which way to write a new primitive component, start by looking at
    how it's going to be used in the code and then go for the better
    ergonomics.

    All public primitives are exported via a central `index.js` file, which
    makes consumption straightforward:

    ```html
    <script>
      import { Button, Title, Icon, Input } from "../DesignSystem/Primitives";
    </script>

    <Icon.Home />
    <Button variant="secondary">OK</Button>
    <Title variant="huge">Radicle</Title>
    ```

  - `Components`: reusable low-to-high complexity components and layouts.

    Sub-folders in this category should only be created for breaking up larger
    components into smaller fragments. If a component is broken up in
    fragments, make sure to only export the component which is intended for
    public use.

    As in `Primitives`, all publicly usable components are exported via
    `index.js`:

    ```html
    <script>
      import { Avatar, Placeholder, Rad } from "../DesignSystem/Components";
    </script>

    <Avatar size="big" title="My name" />
    <Placeholder style="width: 300px; height: 100px" />
    <Rad amount="200" />
    ```

Next to `DesignSystem`, you'll find a directory called `Screens`. Screens bring
together components from the Design System forming what a user in the UI sees
as a whole screen. More complex screens can be broken down into multiple
fragments, in this case the screen will contain data fetching and routing logic
for the fragments. Fragments should be placed in a directory named after the
screen, like so:

```sh
.
├── RegisterProject                    # fragments
│   ├── ConfirmTransactionStep.svelte
│   ├── PickNameStep.svelte
│   ├── PickWalletStep.svelte
│   └── TransactionSummaryStep.svelte
└── RegisterProject.svelte             # screen
```

When multiple screens share the same layout, it should be extracted into a
separate component. Layout components are suffixed with "Layout":
`DesignSystem/Components/ModalLayout.svelt`.

File and directory name casing is as follows:
  - Svelte components and directories containing components - PascalCase
  - everything else: `*.js` files and folders - camelCase

```sh
.
├── App.svelte                     # Root component
├── DesignSystem
│   ├── Components
│   │   ├── Avatar.svelte          # Simple component
│   │   ├── Sidebar                # Folder containing fragments of Sidebar
│   │   │   ├── Avatar.svelte      # These are private to Sidebar.svelte
│   │   │   ├── Item.svelte        # and should not be exported via index.js
│   │   │   └── Tooltip.svelte
│   │   ├── Sidebar.svelte
│   │   ├── SidebarLayout.svelte   # Layout containing a sidebar
│   │   └── index.js               # Defines which components for public use
│   └── Primitives
│       ├── Button.svelte          # Single-file component
│       ├── Icon                   # Name-spaced components
│       │   ├── Branch.svelte
│       │   └── index.js
│       └── index.js
├── Screens
│   ├── Profile.svelte             # Simple screen
│   ├── Project                    # Project screen fragments
│   │   ├── Feed.svelte
│   │   └── Source.svelte
│   ├── Project.svelte             # Data fetching and routing for project fragments
│   └── Wallet.svelte
├── config.js                      # Configuration constants
├── index.js                       # UI entry-point, loaded by the main renderer
├── lib                            # Reusable logic that doesn't fit components
│   ├── hash.js
│   ├── path.js
│   └── types.js
└── stores                         # Svelte stores grouped by use-case
    ├── notification.js
    ├── project.js
    └── sourceBrowser.js
```

#### State

State shared across multiple component fragments which together build a more
complex feature should be extracted into a separate store file. A store file
can contain one or more Svelte stores, as well as other methods that act on
the stores.

```sh
stores
├── notification.js
├── project.js
└── sourceBrowser.js
```


#### CSS

The main entry point of the electron renderer is `public/index.html`. Any
global styling which is not managed by Svelte should be imported there:

```html
<link rel="stylesheet" href="reset.css" />       <!-- style resets -->
<link rel="stylesheet" href="colors.css" />      <!-- color CSS variables -->
<link rel="stylesheet" href="typography.css" />  <!-- font-face setup and typography CSS variables -->
<link rel="stylesheet" href="global.css" />      <!-- global CSS rules -->
<link rel="stylesheet" href="bundle.css" />      <!-- Svelte component CSS -->
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

For very common alignment cases we have a helper component called `Flex`, which
offers two ways of aligning things:

  - via the `align` prop;
  - or by using slots.

If there is only one element to align, use the `align` prop, otherwise use
slots.

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

  "shades": [10],    // Array of auto-generated color shades
                     // possible values range from 5 to 95
                     // values should be kept to multiples of 5

  "tints": [10],     // Array of auto-generated color tints
                     // possible values range from 5 to 95
                     // values should be kept to multiples of 5
}
```

When the `tokens/colors.json` file is changed, we have to re-generate all tints
and shades via: `yarn generate:colors`. This will update the global color CSS
variables in `public/colors.css`. All changes to both files should be
committed.

When developing new features we should only use the CSS variables instead of
raw color codes, like so:

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


### Building a package

Build and package the app with: `yarn dist`. The generated package can be found
in: `dist/radicle-upstream-X.X.X.dmg`.


## Scripts

Most commonly used yarn commands:

```sh
yarn start            # Start electron app in development mode with code
                      # hot-reloading

yarn test             # Run cypress e2e tests
yarn test:debug       # Run tests via the cypress GUI

yarn dist             # Packages the app into an installable package

yarn electron:start   # Wait for dependency start-up and start electron without
                      # code hot-reloading

yarn proxy:build      # Build the backend GraphQL proxy binary
yarn proxy:start      # Start the backend proxy and serve mock data

yarn svelte:clean     # Remove build artifacts
yarn svelte:build     # Compile svelte to JS
yarn svelte:watch     # Compile svelte to JS on every change to the code

yarn generate:colors  # Update color CSS variables in public/colors.css from
                      # colors.json

yarn graphiql         # Open GraphiQL with explorer extension in browser

yarn release          # Start a two-step process to cut a new release, for more
                      # details have a look at ../DEVELOPMENT.md
```

## Proxy

A background service that implements all business logic tying together the
radicle code collaboration and registry protocols. It provides a GraphQL API to
the UI.


### Run

To start up the binary you can run: `cargo run -- --source=memory`.
After that the API is served on `http://127.0.0.1:8080/graphql`.


### Testing

Before running the test suite, download the test fixtures:

```sh
git submodule update --recursive
```

Then run tests as usual:

```sh
cargo test
```


## CI setup

CI is configured via:

```sh
radicle-upstream/.buildkite
.
├── Dockerfile
├── pipeline.yaml
└── run.sh
```

The build process is run for every commit that is pushed to GitHub. When the
tests pass, the build process spits out and uploads an app binary as a build
artifact. When the tests fail, screenshots of the failing tests will be
uploaded instead of the binary.


### Docker image updates

We use a Docker image that has all of the system dependencies installed to run
tests on Buildkite. Follow these steps if you need to update the dependencies
bundled in the image:

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

7. Commit changes to `Dockerfile` and `pipeline.yaml` and push to origin, this
   should build the new branch with the updated image.


## Releases

To perform a release:

1. If you haven't already, install the [`hub`][hb] cli tool.

2. Run: `yarn release` and follow the instructions.
   Here's what a typical release looks like:

```sh
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

3. Once the release PR branch is merged into master a build will be triggered
   on Buildkite, this will build the app for both Linux and macOS. When the
   build has completed you can download the binaries [here][ar].



[pr]: https://prettier.io/
[hu]: https://github.com/typicode/husky
[ls]: https://github.com/okonet/lint-staged
[tp]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[sv]: https://github.com/conventional-changelog/standard-version
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[cc]: https://www.conventionalcommits.org/en/v1.0.0/
[ar]: https://buildkite.com/monadic/radicle-upstream/builds?branch=master
[hb]: https://github.com/github/hub
[ge]: https://github.com/OneGraph/graphiql-explorer
[gs]: https://github.com/OneGraph/graphiql-with-extensions/
