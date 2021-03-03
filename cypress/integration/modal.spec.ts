import * as commands from "../support/commands";

context("modal", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("./public/index.html");
  });

  it("disallows modal stacking", () => {
    cy.log(
      "click the New Project button and check the corresponding modal is opened."
    );
    commands.pick("new-project-button").click();
    commands.pick("modal", "create-project").should("exist");

    cy.log(
      "now open the shortcuts modal and verify that it is the only modal open"
    );
    cy.get("body").type("?");
    commands.pick("modal", "hotkey-modal").should("exist");
    commands.pick("modal", "create-project").should("not.exist");
  });

  context("when navigating to the settings screen", () => {
    it("closes any open modal", () => {
      cy.log(
        "click the New Project button and check the corresponding modal is opened."
      );
      commands.pick("new-project-button").click();
      commands.pick("modal", "create-project").should("exist");

      cy.log(
        "now go to the Settings screen and verify the New Project modal is closed"
      );
      cy.get("body").type(`{${commands.metaKey()}+,}`);
      commands.pick("settings-page").should("exist");
      commands.pick("modal", "create-project").should("not.exist");
    });
  });
});
