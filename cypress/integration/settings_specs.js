context("settings", () => {
  beforeEach(() => {
    cy.nukeCocoState();
    cy.nukeSessionState();
    cy.onboarding();

    cy.visit("public/index.html");
    cy.pick("sidebar", "settings").click();
  });

  context("theme", () => {
    it("is set to the default", () => {
      cy.get("[data-theme='light']").should("exist");
    });

    it("can be switched to dark", () => {
      cy.get("button[value='dark']").click();
      cy.get("[data-theme='dark']").should("exist");
    });
  });

  context("session", () => {
    it("state can be cleared", () => {
      cy.pick("clear-session-button").click();
      cy.pick("get-started-button").should("exist");
    });
  });
});
