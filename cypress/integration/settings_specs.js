context("settings", () => {
  beforeEach(() => {
    cy.nukeCache();
    cy.nukeSessionState();
    cy.nukeRegistryState();
    cy.createIdentity();

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

    it("cache can be cleared", () => {
      // Prepare transaction center.
      cy.registerUser();
      // Force update transaction center.
      cy.reload();
      cy.pick("transaction-center").should("exist");

      // Clear cache.
      cy.pick("clear-cache-button").click();
      cy.pick("transaction-center").should("not.exist");
    });
  });
});
