// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Crypto from "node:crypto";
import * as Fs from "node:fs/promises";
import * as Path from "node:path";
import execa from "execa";
import Semver from "semver";

import * as PeerManager from "./support/peerManager";
import { retryOnError } from "ui/src/retryOnError";

// Assert that the docker container with the test git-server is
// running. If it is not running, throw an error that explains how to
// run it.
export async function assertGitServerRunning(): Promise<void> {
  const containerName = "upstream-git-server-test";
  const notRunningMessage =
    "The git-server test container is required for this test. You can run it with `./scripts/git-server-test.sh`";
  try {
    const result = await execa("docker", [
      "container",
      "inspect",
      containerName,
      "--format",
      "{{.State.Running}}",
    ]);
    if (result.stdout !== "true") {
      throw new Error(notRunningMessage);
    }
  } catch (err: unknown) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    if ((err as any).stderr === `Error: No such container: ${containerName}`) {
      throw new Error(notRunningMessage);
    } else {
      throw err;
    }
  }
}

// Assert that the `rad` CLI is installed and has the correct version.
export async function assertRadInstalled(): Promise<void> {
  const result = await execa("rad", ["--version"]);
  const versionConstraint = ">=0.4.0";
  const version = result.stdout.replace("rad ", "");
  if (!Semver.satisfies(version, versionConstraint)) {
    throw new Error(
      `rad version ${version} does not satisfy ${versionConstraint}`
    );
  }
}

// Returns a path to a directory where the test can store files.
//
// The directory is cleared before it is returned.
export async function prepareStateDir(
  testPath: string,
  testName: string
): Promise<string> {
  const stateDir = Path.resolve(`${testPath}--state`, testName);
  await Fs.rm(stateDir, { recursive: true, force: true });
  await Fs.mkdir(stateDir, { recursive: true });
  return stateDir;
}

// Call `fn` until it does not throw an error and return the result. Re-throws
// the error raised by `fn()` if it still fails after two seconds.
export function retry<T>(fn: () => Promise<T>): Promise<T> {
  return retryOnError(fn, () => true, 100, 30);
}

// Create a project using the rad CLI.
export async function createProject(
  proxy: PeerManager.UpstreamPeer,
  name: string
): Promise<{ urn: string; checkoutPath: string; defaultBranch: string }> {
  const defaultBranch = "main";
  const checkoutPath = Path.join(proxy.checkoutPath, name);

  await proxy.spawn("git", [
    "init",
    checkoutPath,
    "--initial-branch",
    defaultBranch,
  ]);
  await proxy.spawn(
    "git",
    ["commit", "--allow-empty", "--message", "initial commit"],
    {
      cwd: checkoutPath,
    }
  );
  await proxy.spawn(
    "rad",
    [
      "init",
      "--name",
      name,
      "--default-branch",
      defaultBranch,
      "--description",
      "",
    ],
    {
      cwd: checkoutPath,
    }
  );

  const { stdout: urn } = await proxy.spawn("rad", ["inspect"], {
    cwd: checkoutPath,
  });

  await proxy.spawn(
    "git",
    ["config", "--add", "rad.seed", PeerManager.SEED_URL],
    {
      cwd: checkoutPath,
    }
  );

  return { urn, checkoutPath, defaultBranch };
}

// Publish a project using the rad CLI and wait until the proxy registers the
// seed for the project.
export async function publishProject(
  proxy: PeerManager.UpstreamPeer,
  urn: string,
  checkoutPath: string
): Promise<void> {
  await proxy.spawn("rad", ["push"], {
    cwd: checkoutPath,
  });

  await retry(async () => {
    const project = await proxy.proxyClient.project.get(urn);
    if (project.seed === null) {
      throw new Error("Proxy hasn't set the project seed yet.");
    }
  });
}

// Create and publish a project using the rad CLI and return the Project ID.
// Wait until the proxy registers the seed for the project.
export async function createAndPublishProject(
  proxy: PeerManager.UpstreamPeer,
  name: string
): Promise<{ urn: string; checkoutPath: string }> {
  const { urn, checkoutPath } = await createProject(proxy, name);
  await publishProject(proxy, urn, checkoutPath);

  return { urn, checkoutPath };
}

// Create a project from the platinum fixture using the rad CLI.
export async function createProjectFromPlatinumFixture(
  proxy: PeerManager.UpstreamPeer
): Promise<{
  urn: string;
  name: string;
  description: string;
  defaultBranch: string;
  checkoutPath: string;
}> {
  const name = "git-platinum";
  const description = "Platinum files for testing radicle-upstream";
  const checkoutPath = Path.join(proxy.checkoutPath, name);
  const defaultBranch = "main";

  await proxy.spawn("git", [
    "clone",
    Path.join(__dirname, "fixtures", name),
    checkoutPath,
  ]);

  await proxy.spawn("git", ["checkout", "dev"], {
    cwd: checkoutPath,
  });

  await proxy.spawn("git", ["checkout", "main"], {
    cwd: checkoutPath,
  });

  await proxy.spawn(
    "rad",
    [
      "init",
      "--name",
      name,
      "--default-branch",
      defaultBranch,
      "--description",
      description,
    ],
    {
      cwd: checkoutPath,
    }
  );

  const { stdout: urn } = await proxy.spawn("rad", ["inspect"], {
    cwd: checkoutPath,
  });

  await proxy.spawn(
    "git",
    ["config", "--add", "rad.seed", PeerManager.SEED_URL],
    {
      cwd: checkoutPath,
    }
  );

  return { urn, name, description, defaultBranch, checkoutPath };
}

// Clone a project with the `rad` CLI and publish a branch
export async function cloneProject(
  peer: PeerManager.UpstreamPeer,
  projectId: string,
  projectName: string
): Promise<string> {
  const projectCheckoutPath = Path.join(peer.checkoutPath, projectName);

  await peer.spawn("rad", ["clone", projectId, "--seed", "127.0.0.1:8778"], {
    cwd: peer.checkoutPath,
  });
  // Publish the peer's default branch.
  // See <https://github.com/radicle-dev/radicle-upstream/issues/2795>.
  await peer.spawn("rad", ["push", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("rad", ["sync", "--self", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });

  return projectCheckoutPath;
}

// Fork a project by running the same commands as provided by the Fork button
// in the UI.
//
// Requires the project to be replicated. Return the project checkout path.
export async function forkProject(
  projectId: string,
  projectName: string,
  peer: PeerManager.UpstreamPeer
): Promise<string> {
  const projectCheckoutPath = Path.join(peer.checkoutPath, projectName);

  await peer.spawn("rad", ["checkout", projectId], {
    cwd: peer.checkoutPath,
  });
  // Publish the peer's default branch.
  // See <https://github.com/radicle-dev/radicle-upstream/issues/2795>.
  await peer.spawn("rad", ["push", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("rad", ["sync", "--self", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });

  return projectCheckoutPath;
}

// If no branch name is supplied, create patch using the upstream CLI.
// If a branch name is supplied, update an existing patch.
// Return the patch branch name.
export async function createOrUpdatePatch(
  title: string,
  description: string,
  peer: PeerManager.UpstreamPeer,
  projectCheckoutPath: string,
  commitMessage: string = "changes",
  branchName?: string
): Promise<string> {
  const branchName_ = branchName || `patch-branch-${randomTag()}`;
  const checkoutArgs = branchName ? [branchName_] : ["-b", branchName_];

  // Starting from the main branch allows us to create multiple
  // independent patches by running this function multiple times.
  await peer.spawn("git", ["checkout", "main"], {
    cwd: projectCheckoutPath,
  });

  await peer.spawn("git", ["checkout", ...checkoutArgs], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn(
    "git",
    ["commit", "--allow-empty", "--message", commitMessage],
    {
      cwd: projectCheckoutPath,
    }
  );

  const action = branchName ? "update" : "create";
  await peer.spawn(
    "upstream",
    ["patch", action, "-m", `${title}\n\n${description}`],
    {
      cwd: projectCheckoutPath,
    }
  );

  return branchName_;
}

export async function mergeOwnPatch(
  peer: PeerManager.UpstreamPeer,
  projectCheckoutPath: string,
  branchName: string
): Promise<void> {
  await peer.spawn("git", ["checkout", "main"], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("git", ["merge", "--ff-only", branchName], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("rad", ["push", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });
}

export async function mergePatch(
  peer: PeerManager.UpstreamPeer,
  projectCheckoutPath: string,
  patchId: string
): Promise<void> {
  await peer.spawn("git", ["checkout", "main"], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("upstream", ["patch", "fetch", patchId], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("git", ["merge", `radicle-patch/${patchId}`], {
    cwd: projectCheckoutPath,
  });
  await peer.spawn("rad", ["push", "--seed", "127.0.0.1:8778"], {
    cwd: projectCheckoutPath,
  });
}

// Generate string of 12 random characters with 8 bits of entropy.
export function randomTag(): string {
  return Crypto.randomBytes(8).toString("hex");
}
