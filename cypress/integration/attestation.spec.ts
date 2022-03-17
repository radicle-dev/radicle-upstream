// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

describe("attestation", () => {
  it.skip("links radicle identity with ethereum account", () => {
    commands.ethereumDevNode.start();

    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("./public/index.html");
    commands.pick("sidebar", "wallet").click();
    commands.pick("connect-wallet").click();
    commands
      .pick("eth-balance")
      .contains(/\d+(.\d+)? ETH/)
      .should("exist");
    commands.pick("link-button").click();
    commands.pick("confirm-button").click();
    commands
      .pickWithContent(["transaction"], "Claim Identity", {
        timeout: 8000,
      })
      .click();
    commands.pick("transaction-summary").should("contain", "Claim Identity");
    commands
      .pick("transaction-summary", "transaction-status")
      .should("contain", "Included");
  });
});
