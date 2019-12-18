[![Build status](https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master)](https://buildkite.com/monadic/radicle-upstream)

# radicle-upstream

This is a cross-platform desktop app for product explorations.

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


#### CI

CI is configured via:
```
radicle-upstream/.buildkite
.
├── Dockerfile
├── pipeline.yaml
└── run.sh
```

The build process is run for every commit that is pushed to GitHub. When the
tests pass, the build process spits out and uploads an app binary as build
artifact. When the tests fail, screenshots of the failing tests will be
uploaded instead of the binary.


#### Buildkite setup

We use a Docker image that has all of the system dependencies installed to run
tests on Buildkite. Follow these steps if you need to update the dependencies
bundled in the image:

1. Install [Google Cloud SDK][3]
2. Authenticate with Google Cloud: `gcloud auth configure-docker`, pick
   `[1] opensourcecoin` when asked for which project to use
3. Prepare a new docker image with all the necessary dependencies by editing:
   `../.buildkite/Dockerfile`
4. Get the current image version from `pipeline.yaml` and build a new Docker
   image (remember to bump the version):
```
cd .buildkite
docker build . -t gcr.io/opensourcecoin/mvp:0.1.3
docker push gcr.io/opensourcecoin/mvp:0.1.3
```
5. Push the new image version to Google Cloud:
   `docker push gcr.io/opensourcecoin/mvp:0.1.3`

6. Update the image version in `pipeline.yaml`:
```
DOCKER_IMAGE: 'gcr.io/opensourcecoin/mvp:0.1.3'
```
7. Commit changes to `Dockerfile` and `pipeline.yaml` and push to origin, this
   should build the new branch with the updated image


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
[3]: https://cloud.google.com/sdk/docs/quickstart-macos
