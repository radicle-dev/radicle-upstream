context("settings", () => {
  beforeEach(() => {
    cy.resetProxyState();
    cy.onboardUser();
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
});
