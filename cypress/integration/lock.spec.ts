// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("lock screen", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    commands.sealKeystore();
    cy.visit("./public/index.html");
  });

  it("opens on app start when an identity exists", () => {
    commands.pick("unlock-button").should("exist");
  });

  it("shows an error notification if the passphrase is wrong", () => {
    commands.pick("passphrase-input").should("have.focus");
    commands.pick("passphrase-input").type("wrong-pw");
    commands.pick("unlock-button").click();
    cy.contains(/That’s the wrong passphrase./).should("exist");
    commands.pick("passphrase-input").should("have.value", "");
    commands.pick("passphrase-input").should("have.focus");
  });

  it("routes to the profile page on successful unseal", () => {
    commands.pick("unlock-button").should("exist");
    cy.focused().type("radicle-upstream");
    cy.focused().type("{enter}");
    // opens the profile page
    commands.pick("entity-name").contains("secretariat");
    // checks that requests are successful
    commands.pick("sidebar", "settings").click();
    cy.get("button[value='dark']").click();
    cy.get("[data-theme='dark']").should("exist");
  });
});
