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
      cy.get("[data-theme='dark']").should("exist");
    });

    it("can be switched to light", () => {
      cy.get("button[value='light']").click();
      cy.get("[data-theme='light']").should("exist");
    });
  });
});
