import * as commands from "../support/commands";

context("user profile", () => {
  before(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    commands.createProjectWithFixture(
      "platinum",
      "Best project ever.",
      "master",
      ["ele", "abbey"]
    );
  });

  context("visitor view profile page", () => {
    // TODO(sos): unskip when we have a proxy testnet
    it.skip("opens from the peer selector with the correct data", () => {
      // Go to the project source page
      cy.visit("./public/index.html");
      cy.contains("platinum").click();
      cy.contains("Source").click();

      // Pick a user from the peer selector
      commands.pick("peer-selector").click();
      cy.get(".peer-dropdown").contains("abbey").click();

      commands.pick("header").should("exist");

      // Check for the correct data
      commands.pick("entity-name").contains("abbey");
      commands.pick("project-list").contains("platinum").should("exist");
    });
  });
});
