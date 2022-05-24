// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { FullConfig } from "@playwright/test";
import fetch from "node-fetch";
import * as PeerManager from "test/support/peerManager";
import * as Support from "test/support";

export default async function globalSetup(_config: FullConfig): Promise<void> {
  await PeerManager.buildProxy();
  await Support.assertRadInstalled();
  await Support.assertGitServerRunning();
  // Force webpack to build the UI for the first time
  await fetch(`http://localhost:${PeerManager.UI_PORT}`);
}
