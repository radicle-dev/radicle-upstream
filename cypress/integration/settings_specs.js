context("settings", () => {
  beforeEach(() => {
    cy.nukeCache();
    cy.nukeSessionState();
    cy.createIdentity();

    cy.visit("public/index.html");
    cy.select("sidebar", "settings").click();
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
      cy.get("[data-cy='clear-session-button']").click();
      cy.get('[data-cy="get-started-button"]').should("exist");
    });

    it("cache can be cleared", () => {
      // Prepare transaction center.
      cy.registerUser();
      cy.get("body").type("{shift}T");
      cy.get("[data-cy='transaction-center']").should("exist");

      // Clear cache.
      cy.get("[data-cy='clear-cache-button']").click();
      cy.get("[data-cy='transaction-center']").should("not.exist");
    });
  });
});
