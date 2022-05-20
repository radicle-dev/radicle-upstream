// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { PlaywrightTestConfig, devices } from "@playwright/test";
import { UI_PORT } from "./test/support/peerManager";

const config: PlaywrightTestConfig = {
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 2 : 1,
  use: {
    trace: "retain-on-failure",
    actionTimeout: 5000,
  },
  globalSetup: require.resolve("test/support/playwright/globalSetup"),
  webServer: {
    port: UI_PORT,
    command: `webpack serve --open --config-name ui --no-live-reload --port ${UI_PORT} --no-open`,
  },
  testDir: "test/e2e",
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
};
export default config;
