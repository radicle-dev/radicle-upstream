before(() => {
  cy.nukeAllState();
  cy.onboarding("cloudhead");
  cy.createProjectWithFixture("platinum", "Best project ever.", "master", [
    "ele",
    "abbey",
  ]);
});

context("visitor view profile page", () => {
  it("opens from the revision selector with the correct data", () => {
    // Go to the project source page
    cy.visit("./public/index.html#/profile/projects");
    cy.contains("platinum").click();
    cy.contains("Source").click();

    // Pick a user from the revision selector
    cy.pick("revision-selector").click();
    cy.get(".revision-dropdown").pick("abbey").click();

    cy.pick("header").should("exist");

    // Check for the correct data
    cy.pick("entity-name").contains("abbey");
    cy.pick("project-list").contains("platinum").should("exist");
  });
});
