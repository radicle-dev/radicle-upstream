// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Path from "node:path";
import { afterEach, beforeAll, test } from "@jest/globals";

import * as ProxyEvents from "proxy-client/events";
import {
  createPeerManager,
  PeerManager,
  buildProxy,
} from "../support/peerManager";
import * as Support from "../support";

let peerManager: PeerManager;

beforeAll(async () => {
  await buildProxy();
}, 10 * 60 * 1000);

beforeAll(async () => {
  await Support.assertRadInstalled();
  await Support.assertGitServerRunning();
});

beforeEach(async () => {
  const stateDir = await Support.prepareStateDir(
    expect.getState().testPath,
    expect.getState().currentTestName
  );
  peerManager = await createPeerManager({ dataPath: stateDir });
});

afterEach(async () => {
  await peerManager.teardown();
});

test("contributor follows", async () => {
  const maintainer = await peerManager.startPeer({
    name: "maintainer",
  });

  const { urn: projectUrn } = await Support.createAndPublishProject(
    maintainer,
    "foo"
  );

  const contributor = await peerManager.startPeer({
    name: "contributor",
  });

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
}, 20_000);

test("contributor patch replication", async () => {
  const maintainer = await peerManager.startPeer({
    name: "maintainer",
  });

  const { urn: projectUrn } = await Support.createAndPublishProject(
    maintainer,
    "foo"
  );
  const contributor = await peerManager.startPeer({
    name: "contributor",
  });

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

  await maintainer.proxyClient.project.peerTrack(
    projectUrn,
    contributor.peerId
  );
  await Support.retry(async () => {
    const patches = await maintainer.proxyClient.project.patchList(projectUrn);
    expect(patches.length).toBe(1);
    expect(patches[0]?.id).toBe("my-patch");
    expect(patches[0]?.peer.peerId).toBe(contributor.peerId);
    expect(patches[0]?.peer.type).toBe("remote");

    // Make sure the contributor identity is available on the maintainer node.
    const contributorSession = await contributor.proxyClient.sessionGet();
    const peer = await maintainer.proxyClient.personGet(
      contributorSession.identity.urn
    );
    expect(peer).not.toBeNull();
  });
}, 20_000);
