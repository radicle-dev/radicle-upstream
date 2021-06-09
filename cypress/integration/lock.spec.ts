import * as commands from "../support/commands";

context("lock screen", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    commands.sealKeystore();
    cy.visit("./public/index.html");
  });

  it("opens on app start when an identity exists", () => {
    commands.pick("unlock-button").should("exist");
  });

  it("shows an error notification if the passphrase is wrong", () => {
    commands.pick("unlock-button").should("exist");
    commands.pick("passphrase-input").type("wrong-pw");
    commands.pick("unlock-button").click();
    cy.contains(/Couldn’t unlock the app. That’s the wrong passphrase./).should(
      "exist"
    );
    commands.pick("passphrase-input").should("have.value", "wrong-pw");
    commands.pick("unlock-button").should("not.be.disabled");
  });

  it("routes to the profile page on successful unseal", () => {
    commands.pick("unlock-button").should("exist");
    cy.focused().type("radicle-upstream");
    cy.focused().type("{enter}");
    // opens the profile page
    commands.pick("entity-name").contains("secretariat");
    // checks that requests are successful
    commands.pick("sidebar", "settings").click();
    cy.get("button[value='dark']").click();
    cy.get("[data-theme='dark']").should("exist");
  });
});
