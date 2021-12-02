#!/usr/bin/env -S node --require ts-node/register/transpile-only --require tsconfig-paths/register

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { strict as strictAssert } from "assert";

import { sleep } from "ui/src/sleep";
import { RadicleProxy, UpstreamSeed, runTestcase, withRetry } from "./lib/p2p";

// Test that all nodes see each other in their respective active lists in a
// network environment where each node can reach any other node.
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

  const contributor3 = new RadicleProxy({
    name: "contributor3",
    ipAddress: "10.0.0.104",
    seed: seed.seedAddress,
    dataPath,
  });

  const contributor4 = new RadicleProxy({
    name: "contributor4",
    ipAddress: "10.0.0.105",
    seed: seed.seedAddress,
    dataPath,
  });

  seed.start();
  maintainer.start();
  contributor.start();
  contributor2.start();
  contributor3.start();
  contributor4.start();

  await sleep(2000);

  await withRetry(async () => {
    const diagnostics = await maintainer.proxyClient.diagnosticsGet();

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (diagnostics as any).peer.membership.active.sort(),
      [
        seed.peerId,
        contributor.peerId,
        contributor2.peerId,
        contributor3.peerId,
        contributor4.peerId,
      ].sort()
    );
  });

  await withRetry(async () => {
    const diagnostics = await contributor.proxyClient.diagnosticsGet();

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (diagnostics as any).peer.membership.active.sort(),
      [
        seed.peerId,
        maintainer.peerId,
        contributor2.peerId,
        contributor3.peerId,
        contributor4.peerId,
      ].sort()
    );
  });

  await withRetry(async () => {
    const diagnostics = await contributor2.proxyClient.diagnosticsGet();

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (diagnostics as any).peer.membership.active.sort(),
      [
        seed.peerId,
        maintainer.peerId,
        contributor.peerId,
        contributor3.peerId,
        contributor4.peerId,
      ].sort()
    );
  });

  await withRetry(async () => {
    const diagnostics = await contributor3.proxyClient.diagnosticsGet();

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (diagnostics as any).peer.membership.active.sort(),
      [
        seed.peerId,
        maintainer.peerId,
        contributor.peerId,
        contributor2.peerId,
        contributor4.peerId,
      ].sort()
    );
  });

  await withRetry(async () => {
    const diagnostics = await contributor4.proxyClient.diagnosticsGet();

    strictAssert.deepStrictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (diagnostics as any).peer.membership.active.sort(),
      [
        seed.peerId,
        maintainer.peerId,
        contributor.peerId,
        contributor2.peerId,
        contributor3.peerId,
      ].sort()
    );
  });
}

runTestcase({
  testcase,
  networkScript: "mesh-topology.sh",
  dataDirName: path.basename(__filename).replace(".ts", ""),
});
