# radicle-upstream

This is a cross-platform desktop app for product explorations.

## Development

The app is written in Svelte + Electron. For our dependency management and
script execution we use `yarn`.

Code formatting is dictated by `prettify` and we enforce it locally on
a pre-commit basis with [husky][1] and [lint-staged][2].

### Setup

Run the app locally with hot reloading:

1. Get the code: `git clone git@github.com:oscoin/mvp.git`
2. Install dependencies: `cd mvp/app && yarn install`
3. Start the app in development mode: `yarn start`

### Scripts

```
yarn run             - show a list of all available tasks
yarn start           - start electron app in development mode with code
                       hot-reloading
yarn electron:start  - wait for dependency start-up and start electron without
                       code hot-reloading
yarn proxy:build     - build the backend GraphQL proxy binary
yarn proxy:start     - start the backend proxy and serve mock data
yarn svelte:clean    - remove build artefacts
yarn svelte:build    - compile svelte to JS
yarn svelte:watch    - compile svelte to JS on every change to the code
```
