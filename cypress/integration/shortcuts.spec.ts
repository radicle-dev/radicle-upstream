// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";
const metaKey = commands.metaKey();

context("documented shortcuts", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    cy.visit("./public/index.html");
  });

  it("opens and closes the keyboard shortcuts help", () => {
    cy.get("body").type("?");
    commands.pick("hotkey-modal").should("exist");
    // Closing the modal
    cy.get("body").type("{esc}");
    commands.pick("hotkey-modal").should("not.exist");
  });

  it("opens and closes the search", () => {
    cy.get("body").type(`{${metaKey}+p}`);
    commands.pick("search-modal").should("exist");
    // Closing the modal
    cy.get("body").type("{esc}");
    commands.pick("search-modal").should("not.exist");
  });

  it("opens the settings", () => {
    cy.get("body").type(`{${metaKey}+,}`);
    commands.pick("settings-page").should("exist");
    // Esc does not close the settings
    cy.get("body").type("{esc}");
    commands.pick("settings-page").should("exist");
  });
});
