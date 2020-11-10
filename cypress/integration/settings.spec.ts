import * as commands from "../support/commands";

context("settings", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("public/index.html");
    commands.pick("sidebar", "settings").click();
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
