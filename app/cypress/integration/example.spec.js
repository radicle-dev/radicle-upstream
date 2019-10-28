context("session", () => {
  beforeEach(() => {
    cy.visit("./public/index.html");
  });

  context("when you open the app for the first time", () => {
    it("shows the projects screen", () => {
      cy.contains("Radicle Upstream").should("exist");
      cy.contains("Projects").should("exist");
    });
  });
});
