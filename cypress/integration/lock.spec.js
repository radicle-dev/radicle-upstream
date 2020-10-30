// TODO We skip these tests because sealing the proxy in test mode
// currently resets all state including the identity.
context.skip("lock screen", () => {
  beforeEach(() => {
    cy.resetProxyState();
    cy.onboardUser();
    cy.sealKeystore();
    cy.visit("./public/index.html");
  });

  it("opens on app start when an identity exists", () => {
    cy.pick("unlock-button").should("exist");
  });

  it("reloads if the passphrase is wrong", () => {
    cy.pick("unlock-button").should("exist");
    cy.pick("passphrase-input").type("wrong-pw");
    cy.pick("unlock-button").click();
    cy.pick("notification")
      .contains(/Could not unlock the session/)
      .should("exist");
    cy.pick("passphrase-input").should("have.value", "");
    cy.pick("unlock-button").should("be.disabled");
  });

  it("routes to the profile page on successful unseal", () => {
    cy.pick("unlock-button").should("exist");
    cy.pick("passphrase-input").type("radicle-upstream");
    cy.pick("unlock-button").click();
    // opens the profile page
    cy.pick("entity-name").contains("secretariat");
  });
});
