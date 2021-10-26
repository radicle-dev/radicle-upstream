// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("search", () => {
  const projectId = "hnrkfr9g6gxymefc3hto37bgmq3eo86sfckky";
  const peerId = "hyd6gy5asxdf39a5thwt46ncfp3bzzhtk47ytmie51c9z7p8hc48mq";

  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");
  });

  context("when a project is not yet followed", () => {
    it("follows the project by clicking the follow button", () => {
      commands.pick("sidebar", "search").click();
      commands.pasteInto(["search-input"], `rad:git:${projectId}`);
      commands.pick("follow-toggle").click();
      commands.pick("show-requests").click();
      commands
        .pickWithContent(["undiscovered-project"], projectId.slice(-5))
        .should("exist");

      // Make sure the search input is cleared after the search.
      commands.pick("sidebar", "search").click();
      commands.pick("search-input").should("have.value", "");
    });

    it("follows the project by pressing the [enter] hotkey", () => {
      commands.pick("sidebar", "search").click();
      commands.pasteInto(["search-input"], `rad:git:${projectId}`);
      commands.pick("search-modal", "follow-toggle").should("exist");
      cy.get("body").type("{enter}");
      commands.pick("show-requests").click();
      commands
        .pickWithContent(["undiscovered-project"], projectId.slice(-5))
        .should("exist");

      // Make sure the search input is cleared after the search.
      commands.pick("sidebar", "search").click();
      commands.pick("search-input").should("have.value", "");
    });

    context("when the Radicle ID is not valid", () => {
      it("does not follow the project when the [enter] key is pressed", () => {
        cy.intercept(
          "http://localhost:17246/v1/projects/requests/invalid-project-id",
          cy.spy().as("projectRequest")
        );

        commands.pick("sidebar", "search").click();
        commands.pasteInto(["search-input"], "invalid-project-id");
        cy.get("body").type("{enter}");

        commands
          .pick("search-modal")
          .should("contain", "That’s not a valid Radicle ID.");

        commands.pasteInto(["search-input"], peerId);
        cy.get("body").type("{enter}");

        commands
          .pick("search-modal")
          .should(
            "contain",
            "You’ve entered a Device ID instead of a Project ID."
          );

        cy.get("@projectRequest").should("not.have.been.called");
      });
    });
  });

  context("when a project is already followed", () => {
    it("opens the project by clicking the project card", () => {
      commands.pick("project-list-entry-platinum").click();
      commands.pick("project-screen", "header", "radicleId").then(el => {
        const urn = el.attr("data");
        if (!urn) {
          throw new Error("Could not find URN");
        }
        commands.pick("sidebar", "profile").click();
        commands.pick("profile-screen").should("exist");

        commands.pick("sidebar", "search").click();
        commands.pasteInto(["search-input"], urn);
        commands.pick("project-name").click();

        commands.pick("project-screen").should("exist");

        // Make sure the search input is cleared after the search.
        commands.pick("sidebar", "search").click();
        commands.pick("search-input").should("have.value", "");
      });
    });

    it("opens the project by pressing the [enter] hotkey", () => {
      commands.pick("project-list-entry-platinum").click();
      commands.pick("project-screen", "header", "radicleId").then(el => {
        const urn = el.attr("data");
        if (!urn) {
          throw new Error("Could not find URN");
        }
        commands.pick("sidebar", "profile").click();
        commands.pick("profile-screen").should("exist");

        commands.pick("sidebar", "search").click();
        commands.pasteInto(["search-input"], urn);
        commands
          .pick("search-modal", "project-name")
          .should("contain", "platinum");
        cy.get("body").type("{enter}");

        commands.pick("project-screen").should("exist");

        // Make sure the search input is cleared after the search.
        commands.pick("sidebar", "search").click();
        commands.pick("search-input").should("have.value", "");
      });
    });
  });
});
