// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Os from "node:os";
import * as Fs from "node:fs/promises";
import * as Path from "node:path";
import * as Crypto from "node:crypto";
import execa from "execa";
import { afterEach, beforeAll, test } from "@jest/globals";
import waitOn from "wait-on";
import Semver from "semver";

import * as ProxyEvents from "proxy-client/events";
import { retryOnError } from "ui/src/retryOnError";
import * as ProxyRunner from "./support/proxyRunner";
import * as Process from "./support/process";

beforeAll(async () => {
  await assertRadInstalled();
  await assertGitServerRunning();
});

afterEach(async () => {
  ProxyRunner.killAllProcesses();
});

const seedUrl = "http://localhost:8778";

test("contributor follows", async () => {
  const seedUrl = "http://localhost:8778";
  const stateDir = await prepareStateDir();
  const sshAuthSock = await startSshAgent();
  // We need a random user handle so that the Radicle identity IDs
  // are different between runs
  const maintainerName = `maintainer-${randomTag()}`;

  const maintainer = new ProxyRunner.RadicleProxy({
    dataPath: stateDir,
    name: maintainerName,
    sshAuthSock,
  });
  await maintainer.start();

  const projectUrn = await createProject(maintainer, "foo");

  const contributor = new ProxyRunner.RadicleProxy({
    dataPath: stateDir,
    name: `contributor-${randomTag()}`,
    httpPort: 30001,
    gitSeeds: [seedUrl],
    sshAuthSock,
  });

  await contributor.start();

  const projectUpdated = contributor.proxyClient
    .events()
    .filter(ev => {
      return (
        ev.type === ProxyEvents.EventType.ProjectUpdated &&
        ev.urn === projectUrn
      );
    })
    .firstToPromise();
  await contributor.proxyClient.project.requestSubmit(projectUrn);
  await projectUpdated;

  const contributorProject = await contributor.proxyClient.project.get(
    projectUrn
  );
  expect(contributorProject.urn).toEqual(projectUrn);
  expect(contributorProject.metadata.defaultBranch).toEqual("main");
}, 10_000);

test("contributor patch replication", async () => {
  const stateDir = await prepareStateDir();
  const sshAuthSock = await startSshAgent();
  // We need a random user handle so that the Radicle identity IDs
  // are different between runs
  const maintainerName = `maintainer-${randomTag()}`;

  const maintainer = new ProxyRunner.RadicleProxy({
    dataPath: stateDir,
    name: maintainerName,
    gitSeeds: [seedUrl],
    sshAuthSock,
  });
  await maintainer.start();

  const projectUrn = await createProject(maintainer, "foo");
  const contributor = new ProxyRunner.RadicleProxy({
    dataPath: stateDir,
    name: `contributor-${randomTag()}`,
    httpPort: 30001,
    gitSeeds: [seedUrl],
    sshAuthSock,
  });

  await contributor.start();

  const contributorProjectPath = Path.join(contributor.checkoutPath, "foo");
  await contributor.spawn(
    "rad",
    ["clone", projectUrn, "--seed", "127.0.0.1:8778"],
    { cwd: contributor.checkoutPath }
  );
  await contributor.spawn("git", ["checkout", "-b", "my-patch"], {
    cwd: contributorProjectPath,
  });
  await contributor.spawn(
    "git",
    ["commit", "--allow-empty", "--message", "patch changes"],
    {
      cwd: contributorProjectPath,
    }
  );
  await contributor.spawn(
    "upstream",
    ["patch", "create", "--message", "my patch"],
    {
      cwd: contributorProjectPath,
    }
  );
  await contributor.spawn("rad", ["sync"], {
    cwd: contributorProjectPath,
  });

  await maintainer.proxyClient.project.peerTrack(
    projectUrn,
    contributor.peerId
  );
  await retryOnError(
    async () => {
      const patches = await maintainer.proxyClient.project.patchList(
        projectUrn
      );
      expect(patches.length).toBe(1);
      expect(patches[0]?.id).toBe("my-patch");
      expect(patches[0]?.peer.peerId).toBe(contributor.peerId);
      expect(patches[0]?.peer.type).toBe("remote");
    },
    () => true,
    10,
    200
  );
}, 10_000);

// Assert that the docker container with the test git-server is
// running. If it is not running, throw an error that explains how to
// run it.
async function assertGitServerRunning() {
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
async function assertRadInstalled() {
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
async function prepareStateDir(): Promise<string> {
  const testPath = expect.getState().testPath;
  const stateDir = Path.resolve(
    `${testPath}--state`,
    expect.getState().currentTestName
  );
  await Fs.rm(stateDir, { recursive: true, force: true });
  await Fs.mkdir(stateDir, { recursive: true });
  return stateDir;
}

async function startSshAgent(): Promise<string> {
  // We’re not using the state directory because of the size limit on
  // the socket path.
  const dir = await Fs.mkdtemp(Path.join(Os.tmpdir(), "upstream-test"));
  const sshAuthSock = Path.join(dir, "ssh-agent.sock");
  Process.spawn("ssh-agent", ["-D", "-a", sshAuthSock], {
    stdio: "inherit",
  });
  await waitOn({ resources: [sshAuthSock], timeout: 5000 });
  return sshAuthSock;
}

// Generate string of 12 random characters with 8 bits of entropy.
function randomTag(): string {
  return Crypto.randomBytes(8).toString("hex");
}

async function createProject(
  proxy: ProxyRunner.RadicleProxy,
  name: string
): Promise<string> {
  const maintainerProjectPath = Path.join(proxy.checkoutPath, name);
  await proxy.spawn("git", [
    "init",
    maintainerProjectPath,
    "--initial-branch",
    "main",
  ]);
  await proxy.spawn(
    "git",
    ["commit", "--allow-empty", "--message", "initial commit"],
    {
      cwd: maintainerProjectPath,
    }
  );
  await proxy.spawn(
    "rad",
    ["init", "--name", name, "--default-branch", "main", "--description", ""],
    {
      cwd: maintainerProjectPath,
    }
  );

  await proxy.spawn("git", ["config", "--add", "rad.seed", seedUrl], {
    cwd: maintainerProjectPath,
  });

  await proxy.spawn("rad", ["push"], {
    cwd: maintainerProjectPath,
  });

  const { stdout: projectUrn } = await proxy.spawn("rad", ["inspect"], {
    cwd: maintainerProjectPath,
  });

  return projectUrn;
}
