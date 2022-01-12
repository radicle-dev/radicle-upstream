#!/usr/bin/env -S node --require ts-node/register/transpile-only --require tsconfig-paths/register

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { strict as strictAssert } from "assert";

import { sleep } from "ui/src/sleep";
import {
  RadicleProxy,
  UpstreamSeed,
  radCli,
  runTestcase,
  withRetry,
} from "./lib/p2p";

// Test that that the branches of contributors tracked by the maintainer are
// replicated by any other peer.
async function testcase(dataPath: string) {
  const project = {
    name: "my-fancy-project",
    urn: "rad:git:hnrkrhhs1goaawo7db1gpyct8hd7mif5q8c3o",
  };

  const seed = new UpstreamSeed({
    name: "seed",
    ipAddress: "10.0.0.1",
    project: project.urn,
    dataPath,
  });

  const maintainer = new RadicleProxy({
    name: "maintainer",
    ipAddress: "10.0.0.101",
    seed: seed.seedAddress,
    dataPath,
  });

  const contributor = new RadicleProxy({
    name: "contributor",
    ipAddress: "10.0.0.102",
    seed: seed.seedAddress,
    dataPath,
  });

  const contributor2 = new RadicleProxy({
    name: "contributor2",
    ipAddress: "10.0.0.103",
    seed: seed.seedAddress,
    dataPath,
  });

  seed.start();

  maintainer.start();
  contributor.start();
  contributor2.start();

  // Without this the test fails, not sure why.
  await sleep(3000);

  // Maintainer creates a new project.
  await withRetry(async () => {
    await maintainer.proxyClient.project.create({
      repo: {
        type: "new",
        path: maintainer.checkoutPath,
        name: project.name,
      },
      description: "",
      defaultBranch: "main",
    });
  });

  // Assert that the seed received the project.
  await withRetry(async () => {
    const result = radCli({
      radHome: seed.radHome,
      args: ["identities", "project", "get", "--urn", project.urn],
    });

    strictAssert.deepStrictEqual(result, {
      urn: project.urn,
      payload: {
        "https://radicle.xyz/link/identities/project/v1": {
          name: "my-fancy-project",
          description: "",
          default_branch: "main",
        },
      },
    });
  });

  // Without this the test fails, not sure why.
  await sleep(3000);

  // Contributor follows the project.
  await withRetry(async () => {
    await contributor.proxyClient.project.requestSubmit(project.urn);
  });

  // Assert that the contributor received the project.
  await withRetry(async () => {
    const result = await contributor.proxyClient.project.get(project.urn);

    strictAssert.deepStrictEqual(result.urn, project.urn);
  });

  // Contributor forks the project.
  await contributor.proxyClient.project.checkout(project.urn, {
    path: contributor.checkoutPath,
    peerId: maintainer.peerId,
  });

  // Assert that the seed received the contributor's fork.
  await withRetry(async () => {
    const result = radCli({
      radHome: seed.radHome,
      args: ["identities", "project", "tracked", "--urn", project.urn],
    });

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (result as any).find((x: any) => x.peerId === contributor.peerId).status
        .type,
      "replicated"
    );
  });

  // Maintainer adds contributor as a remote to the project.
  await withRetry(async () => {
    await maintainer.proxyClient.project.peerTrack(
      project.urn,
      contributor.peerId
    );
  });

  // Assert that the maintainer found the remote.
  await withRetry(async () => {
    const result = await maintainer.proxyClient.project.listPeers(project.urn);

    strictAssert.deepStrictEqual(
      result.find(x => x.peerId === contributor.peerId)?.status.type,
      "replicated"
    );
  });

  // Assert that the maintainer can view the contributor's branch.
  await withRetry(async () => {
    const branches = await maintainer.proxyClient.source.branchesGet({
      projectUrn: project.urn,
      peerId: contributor.peerId,
    });

    strictAssert.deepStrictEqual(branches, ["main"]);
  });

  // Contributor2 follows the project.
  await withRetry(async () => {
    await contributor2.proxyClient.project.requestSubmit(project.urn);
  });

  // Assert that contributor2 received the project.
  await withRetry(async () => {
    const result = await contributor2.proxyClient.project.get(project.urn);

    strictAssert.deepStrictEqual(result.urn, project.urn);
  });

  // Assert that contributor is in contributor2's remote list.
  await withRetry(async () => {
    const peers = await contributor2.proxyClient.project.listPeers(project.urn);

    console.log("maint: ", maintainer.peerId);
    console.log("cont2: ", contributor2.peerId);
    console.log("cont: ", contributor.peerId);
    console.log(peers);
    console.log(peers.find(x => x.peerId === contributor.peerId)?.status.type);
    strictAssert.deepStrictEqual(
      peers.find(x => x.peerId === contributor.peerId)?.status.type,
      "replicated"
    );
  });

  // Assert that contributor2 can view contributor's branch.
  await withRetry(async () => {
    const branches = await contributor2.proxyClient.source.branchesGet({
      projectUrn: project.urn,
      peerId: contributor.peerId,
    });

    strictAssert.deepStrictEqual(branches, ["main"]);
  });
}

runTestcase({
  testcase,
  networkScript: "star-topology.sh",
  dataDirName: path.basename(__filename).replace(".ts", ""),
});
