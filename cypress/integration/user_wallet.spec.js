before(() => {
  cy.nukeAllState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
});

context("navigation", () => {
  beforeEach(() => {
    cy.visit("public/index.html");
    cy.pick("Wallet").click();
  });
  it("shows the wallet page", () => {
    // user wallet exists
    cy.pick("user-wallet").should("exist");
  });
  it("has a balance", () => {
    // balance is present
    cy.pick("balance").should("exist");
    cy.pick("amount").should("exist");
  });
});
