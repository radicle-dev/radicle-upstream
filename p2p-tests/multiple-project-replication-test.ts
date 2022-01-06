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

async function testcase(dataPath: string) {
  const project1 = {
    name: "project1",
    urn: "rad:git:hnrkn4pax4rsgyxqrioxi9sypj8w8rwnz6tky",
  };

  const project2 = {
    name: "project2",
    urn: "rad:git:hnrkgr1yb8wkfhmd3sucz9zj77ji6ubospd9y",
  };

  const seed = new UpstreamSeed({
    name: "seed",
    ipAddress: "10.0.0.1",
    project: `${project1.urn},${project2.urn}`,
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

  seed.start();
  maintainer.start();

  // Maintainer creates first project.
  await withRetry(async () => {
    await maintainer.proxyClient.project.create({
      repo: {
        type: "new",
        path: maintainer.checkoutPath,
        name: project1.name,
      },
      description: "",
      defaultBranch: "main",
    });
  });

  // Maintainer creates second project.
  await withRetry(async () => {
    await maintainer.proxyClient.project.create({
      repo: {
        type: "new",
        path: maintainer.checkoutPath,
        name: project2.name,
      },
      description: "",
      defaultBranch: "main",
    });
  });

  // Assert that the seed received the first project.
  await withRetry(async () => {
    const result = radCli({
      radHome: seed.radHome,
      args: ["identities", "project", "get", "--urn", project1.urn],
    });

    strictAssert.deepStrictEqual(result, {
      urn: project1.urn,
      payload: {
        "https://radicle.xyz/link/identities/project/v1": {
          name: project1.name,
          description: "",
          default_branch: "main",
        },
      },
    });
  });

  // Assert that the seed received the second project.
  await withRetry(async () => {
    const result = radCli({
      radHome: seed.radHome,
      args: ["identities", "project", "get", "--urn", project2.urn],
    });

    strictAssert.deepStrictEqual(result, {
      urn: project2.urn,
      payload: {
        "https://radicle.xyz/link/identities/project/v1": {
          name: project2.name,
          description: "",
          default_branch: "main",
        },
      },
    });
  });

  // Without this the test fails, not sure why.
  await sleep(1000);

  await maintainer.stop();
  contributor.start();

  // Contributor follows the first project.
  await withRetry(async () => {
    await contributor.proxyClient.project.requestSubmit(project1.urn);
  });

  // Without this the test fails, not sure why.
  await sleep(3000);

  // Contributor follows the second project.
  await withRetry(async () => {
    await contributor.proxyClient.project.requestSubmit(project2.urn);
  });

  // Assert that the contributor received the first project.
  await withRetry(async () => {
    const result = await contributor.proxyClient.project.get(project1.urn);
    strictAssert.deepStrictEqual(result.urn, project1.urn);
  });

  // Assert that the contributor received the second project.
  await withRetry(async () => {
    const result = await contributor.proxyClient.project.get(project2.urn);
    strictAssert.deepStrictEqual(result.urn, project2.urn);
  });
}

runTestcase({
  testcase,
  networkScript: "star-topology.sh",
  dataDirName: path.basename(__filename).replace(".ts", ""),
});
