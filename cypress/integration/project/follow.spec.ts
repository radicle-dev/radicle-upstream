import * as commands from "../../support/commands";

context("project following", () => {
  const projectId = "hnrkfr9g6gxymefc3hto37bgmq3eo86sfckky";

  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");
  });

  it("follows and unfollows with mouse click", () => {
    commands.pick("following-tab").click();
    commands.pick("primary-action").contains("Look for a project").click();
    // The extra whitespace is intentional to check that the input is
    // trimmed.
    commands.pick("search-input").type(`  rad:git:${projectId}  `);
    commands.pick("follow-toggle").should("contain", "Follow");
    commands.pick("follow-toggle").click();
    commands
      .pick("notification")
      .should("contain", "You’ll be notified when this project has been found");

    commands
      .pickWithContent(["undiscovered-project"], projectId)
      .trigger("mouseenter")
      .within(() => {
        commands.pick("follow-toggle").should("contain", "Following");
        commands.pick("follow-toggle").click();
      });

    commands.pick("empty-state").should("exist");
  });

  context("when the project is not yet followed", () => {
    it("follows the project when the [enter] hotkey is pressed", () => {
      commands.pick("sidebar", "search").click();
      commands.pasteInto(["search-input"], `rad:git:${projectId}`);
      cy.get("body").type("{enter}");
      commands
        .pickWithContent(["undiscovered-project"], projectId)
        .should("exist");
    });
  });

  context("when the project is already followed", () => {
    it("opens the project when the [enter] hotkey is pressed", () => {
      commands.pick("project-list-entry-platinum").click();
      commands.pick("project-screen", "header", "urn").then(el => {
        const urn = el.attr("title");
        if (!urn) {
          throw new Error("Could not find URN");
        }
        commands.pick("sidebar", "profile").click();
        commands.pick("profile-screen").should("exist");

        commands.pick("sidebar", "search").click();
        commands.pick("search-input").type(urn);
        cy.get("body").type("{enter}");

        commands.pick("project-screen").should("exist");
      });
    });
  });
});
