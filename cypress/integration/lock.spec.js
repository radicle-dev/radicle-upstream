context("lock screen", () => {
  beforeEach(() => {
    cy.resetProxyState();
    cy.onboardUser();
    cy.sealKeystore();
    cy.visit("./public/index.html");
  });

  it("opens on app start when an identity exists", () => {
    cy.pick("unlock-button").should("exist");
  });

  it("shows an error notification if the passphrase is wrong", () => {
    cy.pick("unlock-button").should("exist");
    cy.pick("passphrase-input").type("wrong-pw");
    cy.pick("unlock-button").click();
    cy.contains(/Could not unlock the session: Passphrase incorrect/).should(
      "exist"
    );
    cy.pick("passphrase-input").should("have.value", "wrong-pw");
    cy.pick("unlock-button").should("not.be.disabled");
  });

  it("routes to the profile page on successful unseal", () => {
    cy.pick("unlock-button").should("exist");
    cy.focused().type("radicle-upstream");
    cy.focused().type("{enter}");
    // opens the profile page
    cy.pick("entity-name").contains("secretariat");
    // checks that requests are successful
    cy.pick("sidebar", "settings").click();
    cy.get("button[value='dark']").click();
    cy.get("[data-theme='dark']").should("exist");
  });
});
