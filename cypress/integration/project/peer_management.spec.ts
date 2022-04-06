// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("project peer management", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");

    commands.pick("project-list-entry-platinum").click();
    commands.pick("peer-selector").click();
    commands.pick("manage-remotes").click();
  });

  it("shows our own peer", () => {
    commands
      .pick("tracked-peers")
      .contains("li", "secretariat")
      .within(() => {
        cy.contains("delegate").should("exist");
      });
  });

  it("allows adding a new peer track request", () => {
    // The track button is disabled when the input field is empty.
    commands.pick("track-button").should("have.class", "disabled");

    commands.pasteInto(
      ["peer-input"],
      "hynsejpdsftse6f9bczzf69c1im9ewanb5ajnqruq3cq19keiuzk4c"
    );

    commands.pick("track-button").should("not.have.class", "disabled");

    commands.pick("track-button").click();

    commands
      .pick("pending-peers")
      .contains("li", "hynsejpd…keiuzk4c")
      .within(() => {
        commands.pick("track-toggle").should("exist");
        commands.pick("track-toggle").trigger("mouseenter");
        commands.pick("track-toggle").contains("Untrack").should("exist");
        commands.pick("track-toggle").trigger("mouseleave");
      });

    // Disallows adding the same peer again
    commands.pasteInto(
      ["peer-input"],
      "hynsejpdsftse6f9bczzf69c1im9ewanb5ajnqruq3cq19keiuzk4c"
    );

    commands.pick("track-button").click();
    cy.contains("This remote is already being tracked").should("exist");

    // Clears the validation message when the input is cleared.
    commands.pick("peer-input").type("{selectall}{backspace}");
    cy.contains("This remote is already being tracked").should("not.exist");

    // Disallows adding an invalid peer.
    commands.pick("peer-input").type("123");
    commands.pick("track-button").click();
    cy.contains("This is not a valid remote").should("exist");

    // Allows deleting a peer track request.
    commands
      .pick("pending-peers")
      .contains("li", "hynsejpd…keiuzk4c")
      .within(() => {
        commands.pick("track-toggle").click();
      });

    cy.contains("li", "hynsejpd…keiuzk4c").should("not.exist");
  });
});
