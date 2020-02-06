# App

A frontend written in JavaScript using Svelte and Electron.


## Development

For dependency management and script execution we use `yarn`. Code formatting
is dictated by [prettier][pr] and enforced locally on a pre-commit basis with
[husky][hu] and [lint-staged][ls].


### Running the app

1. Get the code: `git clone git@github.com:radicle-dev/radicle-upstream.git`.
2. Install dependencies: `cd radicle-upstream/app && yarn install`.
3. Start the app in development mode: `yarn start`.


### Running tests

Before running tests locally you'll need to set up a test fixture repository:
`git submodule foreach "git fetch --all"`.

- to run the tests do: `yarn test`
- to troubleshoot tests in the Cypress GUI, run: `yarn test:debug`


### Design System

The overall look of Upstream is governed by a style guide which is continuously
being improved and extanded by our designers. This style guide is translated
into code that forms a design system. The design system contains all design
primitives which can be composed to create rich user experiences.


#### Colors

Colors amongst other design system tokens are stored in `tokens/colors.json`.
If a color gets added, removed or changed in the style guide, we have to make
changes to `colors.json` accordingly.

Entries in `colors.json` have the following shape:
```javascript
{
  "name": "pink",    // color name as defined in style guide
  "hex": "#e074cb",  // hex color code
  "shades": [10],    // array of auto-generated color shades
                     // possible values range from 5 to 95
                     // values should be kept to multiples of 5

  "tints": [10],     // array of auto-generated color tints
                     // possible values range from 5 to 95
                     // values should be kept to multiples of 5
}
```

When the `colors.json` file is changed, we need to re-generate all the
tints and shades via: `yarn generate:colors`. This will update the
`public/colors.css` file. All changes to both files should be committed to the
repository.

The main entry point of the electron renderer is `public/index.html`. Amongst
other things it loads `colors.css` which, in turn, provides colors including
their tints and shades as CSS variables to the rest of the code.

When developing new features we should only use these CSS variables instead of
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
yarn start            # start electron app in development mode with code
                      # hot-reloading

yarn test             # run cypress e2e tests
yarn test:debug       # run tests via the cypress GUI

yarn dist             # packages the app into an installable package

yarn electron:start   # wait for dependency start-up and start electron without
                      # code hot-reloading

yarn proxy:build      # build the backend GraphQL proxy binary
yarn proxy:start      # start the backend proxy and serve mock data

yarn svelte:clean     # remove build artifacts
yarn svelte:build     # compile svelte to JS
yarn svelte:watch     # compile svelte to JS on every change to the code

yarn generate:colors  # update color CSS variables in public/colors.css from
                      # colors.json

yarn release          # start a two-step process to cut a new release, for more
                      # details have a look at ../DEVELOPMENT.md
```


[pr]: https://prettier.io/
[hu]: https://github.com/typicode/husky
[ls]: https://github.com/okonet/lint-staged
