// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Filter Jest tests. Currently, we skip tests that require a docker container on CI on macOS.
module.exports = function filter(paths) {
  const filtered = paths.filter(isTestEnabled).map(path => ({
    test: path,
  }));
  return { filtered };
};

function isTestEnabled(path) {
  if (
    // Disable git fetch test on macOS because Docker is unavailable
    process.env.CI === "true" &&
    process.env.RUNNER_OS === "macOS" &&
    path.endsWith("/gitFetch.test.ts")
  ) {
    return false;
  }

  return true;
}
