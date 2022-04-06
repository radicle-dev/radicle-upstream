// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("project tracking", () => {
  const projectId = "hnrkfr9g6gxymefc3hto37bgmq3eo86sfckky";

  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");
  });

  it("tracks and untracks", () => {
    commands.pick("search-box").contains("Look for a project").click();
    // The extra whitespace is intentional to check that the input is
    // trimmed.
    commands.pick("search-input").type(`  rad:git:${projectId}  `);
    commands.pick("track-toggle").should("contain", "Track");
    commands.pick("track-toggle").click();
    commands
      .pick("notification")
      .should("contain", "You’ll be notified when this project has been found");

    commands.pick("show-requests").click();
    commands
      .pickWithContent(["undiscovered-project"], projectId.slice(-5))
      .within(() => {
        commands.pick("track-toggle").click();
      });

    commands
      .pickWithContent(["undiscovered-project"], projectId.slice(-5))
      .should("not.exist");
  });
});
