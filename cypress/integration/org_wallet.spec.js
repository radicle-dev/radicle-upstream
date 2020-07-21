before(() => {
  cy.nukeAllState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
  cy.registerOrg("coolorg");
});

context("navigation", () => {
  beforeEach(() => {
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("Wallet").click();
  });
  it("shows the wallet page", () => {
    // org wallet exists
    cy.pick("org-wallet").should("exist");
  });
  it("has a balance", () => {
    // balance is present
    cy.pick("balance").should("exist");
    cy.pick("amount").should("exist");
  });
  context("send-receive tab", () => {
    // send receive is there and functional
    beforeEach(() => {
      cy.pick("send-receive").should("exist");
    });
    it("goes to receive tab and checks that everything is there", () => {
      cy.pick("receive-tab").click();
      cy.pick("receive").should("exist");
      cy.pick("qr-code").should("exist");
      cy.pick("urn").should("exist");
    });
    it("goes to back to send tab and checks that everything is there", () => {
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      // todo validate fields & test send functionality when implemented
    });
  });
});
