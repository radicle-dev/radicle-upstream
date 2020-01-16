# radicle desktop client

This is a cross-platform desktop client for Radicle.
We currently support Linux and macOS.


## Development

The app is written in Svelte + Electron. For our dependency management and
script execution we use `yarn`.

Code formatting is dictated by [prettier][0] and we enforce it locally on
a pre-commit basis with [husky][1] and [lint-staged][2].


### Setup

Run the app locally with hot reloading:

1. Get the code: `git clone git@github.com:radicle-dev/radicle-upstream.git`
2. Install dependencies: `cd radicle-upstream/app && yarn install`
3. Start the app in development mode: `yarn start`


Build and package the app:

1. Build: `yarn dist`
2. Get the generated package from: `dist/radicle-upstream-0.0.1.dmg`


#### Running tests locally

Before running tests locally you'll need to set up a test fixture repository:
`git submodule foreach "git fetch --all"`.

To run the tests do: `yarn test`.
To troubleshoot tests in the Cypress GUI, run: `yarn test:debug`.


### Scripts

```
yarn start           - start electron app in development mode with code
                       hot-reloading
yarn test            - run cypress e2e tests
yarn test:debug      - run tests via the cypress GUI
yarn dist            - packages the app into an installable package

yarn electron:start  - wait for dependency start-up and start electron without
                       code hot-reloading

yarn proxy:build     - build the backend GraphQL proxy binary
yarn proxy:start     - start the backend proxy and serve mock data

yarn svelte:clean    - remove build artefacts
yarn svelte:build    - compile svelte to JS
yarn svelte:watch    - compile svelte to JS on every change to the code
```


[0]: https://prettier.io/
[1]: https://github.com/typicode/husky
[2]: https://github.com/okonet/lint-staged
