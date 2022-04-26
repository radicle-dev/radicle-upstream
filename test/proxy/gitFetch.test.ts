// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Path from "node:path";
import { afterEach, beforeAll, test } from "@jest/globals";

import * as ProxyEvents from "proxy-client/events";
import { retryOnError } from "ui/src/retryOnError";
import * as ProxyRunner from "./support/proxyRunner";
import * as Support from "./support";

beforeAll(async () => {
  await Support.assertRadInstalled();
  await Support.assertGitServerRunning();
});

afterEach(async () => {
  ProxyRunner.killAllProcesses();
});

const seedUrl = "http://localhost:8778";

test("contributor follows", async () => {
  const stateDir = await Support.prepareStateDir();
  const sshAuthSock = await Support.startSshAgent();

  const maintainer = await ProxyRunner.RadicleProxy.create({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock,
  });
  await maintainer.start();

  const projectUrn = await Support.createProject(maintainer, "foo");

  const contributor = await ProxyRunner.RadicleProxy.create({
    dataPath: stateDir,
    name: "contributor",
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
  const stateDir = await Support.prepareStateDir();
  const sshAuthSock = await Support.startSshAgent();

  const maintainer = await ProxyRunner.RadicleProxy.create({
    dataPath: stateDir,
    name: "maintainer",
    gitSeeds: [seedUrl],
    sshAuthSock,
  });
  await maintainer.start();

  const projectUrn = await Support.createProject(maintainer, "foo");
  const contributor = await ProxyRunner.RadicleProxy.create({
    dataPath: stateDir,
    name: "contributor",
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
