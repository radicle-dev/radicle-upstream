[![Build status][badge]][status]

# radicle-upstream

A monorepo for related Radicle projects:

- `app`: Cross-platform desktop client for radicle.
- `proxy`: Intermediate serving a specialised API to the app frontend
  via GraphQL.

See the README.md of each respective project for further details, including
installation, usage, and testing instructions.


## Development

Our current workflow is to put our changes in feature branches which get
submitted for review on GitHub as pull requests. Ideally a pull request is
small and changes only one aspect of the code at a time. After a pull request
is reviewed by at least one peer and passes all tests it can be squash-merged
into master.

To automate our release process as much as possible we're using
[Standard Version][sv] and commits on master should be formatted according to
the [conventional commits specification][ccs].

Here are a couple of examples:

  chore: remove leftover error mod reference (#74)
  fix: improve project creation validations (#76)
  feat: project creation (#70)

The pull request ref goes in brackets at the end.
Generally we also follow [good commit message hygene][tpope].


### Releases

:warning::warning::warning:

At the moment radicle-upstream is in **prototype stage** and releases should
be considered a preview of what's to come, **use on your own risk**.

Follow [radicle.community][comm] for official updates on public releases.

:warning::warning::warning:


To perform a release:

1. Create a new branch for the release:
```
git branch release-v0.x.x && git checkout release-v0.x.x
```

2. Cut a release: `(cd app && yarn release)`

4. Make a pull request from the `release-v0.x.x` branch and push the changes:
```
git push origin release-v0.x.x
```

5. Get your pull request reviewed and merge it into master, then tag the
   release commit master:
```
git checkout master
git pull

# fill in actual version and commit SHA
git tag v0.x.x XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
git push --tags
```

6. Once the branch is merged into master a build will be triggered on
   Buildkite, this will build the app for both Linux and macOS. When the build
   has completed [download the artifacts][artifacts].

7. Draft a new release on [Github][releases] by pressing `Draft a new release`
   and fill out the form:
     - `Tag version`: pick the tag generated in step 2.
     - `Release title`: same as tag
     - attach binaries from step 5 (.AppImage, .snap and .dmg)
     - check "This is a pre-release"

7. Click `Publish release`


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

1. Install [Google Cloud SDK][gc]

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


[badge]: https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master
[status]: https://buildkite.com/monadic/radicle-upstream
[tpope]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[sv]: https://github.com/conventional-changelog/standard-version
[gc]: https://cloud.google.com/sdk/docs/quickstart-macos
[ccs]: https://www.conventionalcommits.org/en/v1.0.0/
[artifacts]: https://buildkite.com/monadic/radicle-upstream/builds?branch=master
[releases]: https://github.com/radicle-dev/radicle-upstream/releases
[comm]: https://radicle.community/
