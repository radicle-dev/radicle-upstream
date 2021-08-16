// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import "./assertions";
import * as ipcStub from "./ipc-stub";
import * as commands from "./commands";

// Prepare the application `window` instance for cypress test.
Cypress.on("window:before:load", appWindow => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (appWindow as any).isCypressTestEnv = true;

  // Stub electron preloader to always enable `isDev` and `isExperimental` before executing tests.
  ipcStub.setup(appWindow);
});

// If a test was successful we unload the app so it stops running. If the test
// was failed we want to keep the app around so we can inspect it.
//
// This is to workaround https://github.com/cypress-io/cypress/issues/15247
afterEach(function () {
  if (this.currentTest && this.currentTest.state !== "failed") {
    cy.visit("./cypress/empty.html");
    commands.ethereumDevNode.stop();
  }
});

// Common setup for all tests.
beforeEach(() => {
  commands.ethereumDevNode.stop();
  cy.window().then(win => {
    win.localStorage.setItem(
      "radicle.settings.updateChecker.isEnabled",
      "false"
    );
  });
});
