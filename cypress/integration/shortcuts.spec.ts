import * as commands from "../support/commands";
const metaKey = commands.metaKey();

context("documented shortcuts", () => {
  before(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    cy.visit("./public/index.html");
  });

  it("opens and closes the keyboard shortcuts help", () => {
    cy.get("body").type("?");
    commands.pick("hotkey-modal").should("exist");
    // Closing the modal
    cy.get("body").type("{esc}");
    commands.pick("hotkey-modal").should("not.exist");
  });

  it("opens and closes the project creation", () => {
    cy.get("body").type(`{${metaKey}+n}`);
    commands.pick("create-project").should("exist");
    // Closing the modal
    cy.get("body").type("{esc}");
    commands.pick("create-project").should("not.exist");
  });

  it("opens and closes the search", () => {
    cy.get("body").type(`{${metaKey}+p}`);
    commands.pick("search-modal").should("exist");
    // Closing the modal
    cy.get("body").type("{esc}");
    commands.pick("search-modal").should("not.exist");
  });

  it("opens the settings", () => {
    cy.get("body").type(`{${metaKey}+,}`);
    commands.pick("settings-page").should("exist");
    // Esc does not close the settings
    cy.get("body").type("{esc}");
    commands.pick("settings-page").should("exist");
  });
});
