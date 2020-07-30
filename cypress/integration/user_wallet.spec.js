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
  context("balance", () => {
    // checks the balance component
    it("has a balance", () => {
      // balance is present
      cy.pick("balance").should("exist");
      cy.pick("amount").should("exist");
    });
  });
  context("transactions", () => {
    // checks the transactions component
    it("has transaction component", () => {
      // transactions are present
      cy.pick("transactions").should("exist");
    });
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
    });
    it("tests if the send tab validation and inputs work", () => {
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      cy.pick("recipient-input").should("exist");
      cy.pick("recipient-input").type(
        "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("amount-input").should("exist");
      cy.pick("amount-input").type("123");
      cy.pick("send-transaction-button").click();
    });
  });
  context("check if it opens the modal", () => {
    it("opens the send funds modal", () => {
      cy.pick("send-receive").should("exist");
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      cy.pick("recipient-input").should("exist");
      cy.pick("recipient-input").type(
        "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("amount-input").should("exist");
      cy.pick("amount-input").type("123");
      cy.pick("send-transaction-button").click();
      cy.pick("send-funds-modal").should("exist");
    });
  });
});
