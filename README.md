[![Build status][0]][1]

# radicle-upstream

A monorepo for related Radicle projects:

- `app`: Cross-platform desktop client for radicle.
- `proxy`: Intermediate serving a specialised API to the app frontend
  via GraphQL.

See the README.md of each respective project for further details, including
installation, usage, and testing instructions.


## Development

Notes and guidelines for committers:

- [Use tpope commit norms][2]: Capitalize commit messages, use the imperative
  present tense, wrap commit messages at 50 characters.


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
   `.buildkite/Dockerfile`

4. Get the current image version from `pipeline.yaml` and build a new Docker
   image (remember to bump the version):
```
cd .buildkite
docker build . -t gcr.io/opensourcecoin/radicle-upstream:0.2.1
docker push gcr.io/opensourcecoin/radicle-upstream:0.2.1
```

5. Push the new image version to Google Cloud:
   `docker push gcr.io/opensourcecoin/radicle-upstream:0.2.1`

6. Update the image version in `pipeline.yaml`:
```
DOCKER_IMAGE: 'gcr.io/opensourcecoin/radicle-upstream:0.2.1'

```
7. Commit changes to `Dockerfile` and `pipeline.yaml` and push to origin, this
   should build the new branch with the updated image


[0]: https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master
[1]: https://buildkite.com/monadic/radicle-upstream
[2]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[3]: https://cloud.google.com/sdk/docs/quickstart-macos
