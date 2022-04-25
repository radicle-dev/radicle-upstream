// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { FullConfig } from "@playwright/test";
import * as PeerRunner from "test/support/peerRunner";
import * as Support from "test/support";

async function globalSetup(_config: FullConfig): Promise<void> {
  await PeerRunner.buildProxy();
  await PeerRunner.buildUi();
  await Support.assertRadInstalled();
  await Support.assertGitServerRunning();
}

export default globalSetup;
