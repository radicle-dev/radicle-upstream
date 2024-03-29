// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { afterEach, beforeAll, test } from "@jest/globals";

import { EventEnvelope } from "proxy-client/project";
import * as Support from "../support";
import {
  createPeerManager,
  PeerManager,
  buildProxy,
} from "../support/peerManager";
import { sleep } from "ui/src/sleep";

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

test("event log replication", async () => {
  const maintainer = await peerManager.startPeer({
    name: "maintainer",
  });

  const contributor = await peerManager.startPeer({
    name: "contributor",
  });

  const { urn: projectUrn } = await Support.createAndPublishProject(
    maintainer,
    "foo"
  );
  const topic = "asdf";

  const maintainerEvent = {
    type: "foo",
    data: 1,
  } as const;
  await maintainer.proxyClient.project.eventPublish(
    projectUrn,
    topic,
    maintainerEvent
  );

  const expectedEventEnvelopes: EventEnvelope[] = [
    {
      peer_id: maintainer.peerId,
      event: maintainerEvent,
    },
  ];

  {
    const eventsFromMaintainer = await maintainer.proxyClient.project.eventList(
      projectUrn,
      topic
    );
    expect(eventsFromMaintainer).toEqual(expectedEventEnvelopes);
  }

  await contributor.proxyClient.project.requestSubmit(projectUrn);
  await maintainer.proxyClient.project.peerTrack(
    projectUrn,
    contributor.peerId
  );

  await Support.retry(async () => {
    const eventsFromContributor =
      await contributor.proxyClient.project.eventList(projectUrn, topic);
    expect(eventsFromContributor).toEqual(expectedEventEnvelopes);
  });

  // We wait so that the contributor event gets a more recent timestamp
  // than the maintainer event. (Event timestamps have a resolution of
  // one second.) Otherwise the event list returned by the proxy may
  // not be correctly ordered
  await sleep(1000);

  const contributorEvent = {
    type: "bar",
    data: 2,
  } as const;
  await contributor.proxyClient.project.eventPublish(
    projectUrn,
    topic,
    contributorEvent
  );

  expectedEventEnvelopes.unshift({
    peer_id: contributor.peerId,
    event: contributorEvent,
  });

  await Support.retry(async () => {
    const eventsFromContributor =
      await contributor.proxyClient.project.eventList(projectUrn, topic);
    expect(eventsFromContributor).toEqual(expectedEventEnvelopes);
  });

  await Support.retry(async () => {
    const eventsFromMaintainer = await maintainer.proxyClient.project.eventList(
      projectUrn,
      topic
    );
    expect(eventsFromMaintainer).toEqual(expectedEventEnvelopes);
  });
}, 20_000);
