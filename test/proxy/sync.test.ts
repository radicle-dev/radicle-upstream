// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { expect, test, afterEach } from "@jest/globals";
import * as Fs from "fs/promises";
import * as Path from "path";
import * as P2P from "p2p-tests/lib/p2p";
import * as proxyEvents from "proxy-client/events";

afterEach(async () => {
  P2P.killAllProcesses();
});

test("fetch initial from seed", async () => {
  const workdir = await createWorkdir("fetch-initial-from-seed");

  const proxy = new P2P.RadicleProxy({
    dataPath: workdir,
    name: "foo",
    networkNamespace: false,
    gitSeeds: ["https://willow.radicle.garden"],
  });
  await proxy.start();
  const urn = "rad:git:hnrkmg77m8tfzj4gi4pa4mbhgysfgzwntjpao";
  const updated = proxy.proxyClient
    .events()
    .filter(ev => {
      return ev.type === proxyEvents.EventType.ProjectUpdated && ev.urn === urn;
    })
    .firstToPromise();
  await proxy.proxyClient.project.requestSubmit(urn);
  await updated;
  expect(await proxy.proxyClient.project.get(urn)).toMatchObject({ urn });
}, 10000);

async function createWorkdir(testName: string): Promise<string> {
  const workdir = Path.resolve(__dirname, "..", "workdir", testName);
  await Fs.rm(workdir, { recursive: true, force: true });
  await Fs.mkdir(workdir, { recursive: true });
  return workdir;
}
