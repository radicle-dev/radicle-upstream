#!/usr/bin/env -S node --require ts-node/register/transpile-only --require tsconfig-paths/register

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { strict as strictAssert } from "assert";

import * as source from "proxy-client/source";
import { sleep } from "ui/src/sleep";
import {
  RadicleProxy,
  UpstreamSeed,
  commit,
  getLatestCommitSha,
  pushRad,
  radCli,
  runTestcase,
  withRetry,
} from "./lib/p2p";

// Test that updates to a project from the maintainer are replicated to
// trackers via a seed.
//
// This testcase is a variation on maintainer-update-propagation-1-test.ts:
// we don't stop the nodes between issuing commands.
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
  const maintainerProjectCheckoutPath = path.join(
    maintainer.checkoutPath,
    project.name
  );

  const contributor = new RadicleProxy({
    name: "contributor",
    ipAddress: "10.0.0.102",
    seed: seed.seedAddress,
    dataPath,
  });

  seed.start();
  maintainer.start();
  contributor.start();

  // Without this the test fails, not sure why.
  await sleep(1000);

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

  // Contributor follows the project.
  await withRetry(async () => {
    await contributor.proxyClient.project.requestSubmit(project.urn);
  });

  // Assert that the contributor received the project.
  await withRetry(async () => {
    const result = await contributor.proxyClient.project.get(project.urn);

    strictAssert.deepStrictEqual(result, {
      urn: project.urn,
      metadata: {
        name: project.name,
        description: "",
        defaultBranch: "main",
        maintainers: [maintainer.identityUrn],
      },
      stats: { commits: 1, branches: 0, contributors: 1 },
    });
  });

  // Maintainer publishes a new commit.
  commit({
    author: maintainer.name,
    checkoutPath: maintainerProjectCheckoutPath,
  });
  pushRad({
    radHome: maintainer.radHome,
    checkoutPath: maintainerProjectCheckoutPath,
    keyPassphrase: maintainer.passphrase,
  });

  // Assert that the seed received the maintainer's latest commit.
  await withRetry(async () => {
    const result = radCli({
      radHome: seed.radHome,
      args: ["identities", "project", "tracked", "--urn", project.urn],
    });

    strictAssert.strictEqual(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (result as any).find((x: any) => x.peerId === maintainer.peerId).status
        .user.refs.heads.main,
      getLatestCommitSha(maintainerProjectCheckoutPath)
    );
  });

  // Assert that the contributor received the maintainer's latest commit.
  await withRetry(async () => {
    const commitList = await contributor.proxyClient.source.commitsGet({
      projectUrn: project.urn,
      peerId: maintainer.peerId,
      revision: { type: source.RevisionType.Branch, name: "main" },
    });

    strictAssert.strictEqual(
      commitList.headers[0].sha1,
      getLatestCommitSha(maintainerProjectCheckoutPath)
    );
  });
}

runTestcase({
  testcase,
  networkScript: "star-topology.sh",
  dataDirName: path.basename(__filename).replace(".ts", ""),
});
