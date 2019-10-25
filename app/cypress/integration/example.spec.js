context("session", () => {
  beforeEach(() => {
    cy.visit("./public/index.html#/projects");
  });

  context("when you open the app for the first time", () => {
    it("shows the projects screen", () => {
      cy.contains("Projects").should("exist");
    });
  });
});
