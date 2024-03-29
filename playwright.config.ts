// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { PlaywrightTestConfig, devices } from "@playwright/test";
import { UI_PORT } from "./test/support/peerManager";

const config: PlaywrightTestConfig = {
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 3 : 0,
  // he default for the number of workers is half the number of cores.
  // We want to make use of all cores on Github CI.
  workers: process.env.CI ? 2 : undefined,
  reporter: process.env.CI ? "github" : undefined,
  use: {
    trace: "retain-on-failure",
    actionTimeout: process.env.CI ? 30000 : 5000,
  },
  expect: { timeout: process.env.CI ? 30000 : 10000 },
  globalSetup: require.resolve("test/support/playwright/globalSetup"),
  webServer: {
    port: UI_PORT,
    command: `webpack serve --config-name ui --no-live-reload --port ${UI_PORT} --no-open --no-hot`,
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
