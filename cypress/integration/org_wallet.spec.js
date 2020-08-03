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
      cy.pick("recipient-input").type(
        "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("amount-input").type("123");
      cy.pick("send-transaction-button").click();
    });
    it("disables the button when there is no amount filled in", () => {
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      cy.pick("recipient-input").type(
        "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("send-transaction-button").should("be.disabled");
    });
    it("disables the button when there is no recipient filled in", () => {
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      cy.pick("amount-input").type("123");
      cy.pick("send-transaction-button").should("be.disabled");
    });
  });
  context("sending funds flow", () => {
    it("send funds to another account", () => {
      // 0. check the state of the wallet before sending a transfer
      cy.pick("balance", "amount").contains("321");
      cy.pick("transactions").children(".item").should("have.length", 1);

      // 1. Fill the 'Send' form in 'SendReceive'
      cy.pick("send-receive").should("exist");
      cy.pick("send-tab").click();
      cy.pick("send").should("exist");
      cy.pick("recipient-input").type(
        "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("amount-input").type("123");
      cy.pick("send-transaction-button").click();

      // 2. Now in the send funds modal, change the inputed values.
      cy.pick("send-funds-modal").should("exist");
      cy.pick("modal-amount-input").clear();
      cy.pick("modal-amount-input").type("112");
      cy.pick("review-transfer-button").click();

      // 3. Now in the final review step, submit
      cy.pick("review-step").should("exist");
      cy.pick("transfer-amount", "amount").contains("-112");
      cy.pick("transaction-fee", "amount").contains("-0.000001");
      cy.pick("total", "amount").contains("-112.000001");
      cy.pick("funding-source").contains("coolorg");
      cy.pick("submit-transfer-button").click();
    });
    it("shows the expected updated wallet state after the transfer", () => {
      cy.pick("balance", "amount").contains("208.999999");
      cy.pick("transactions").should("exist");
      cy.pick("transactions").children(".item").should("have.length", 2);
      cy.pick("transactions")
        .children(".item")
        .eq(0)
        .get(".description")
        .pick("message")
        .contains("Outgoing transfer");
      cy.pick("transactions")
        .children(".item")
        .eq(0)
        .get(".description")
        .pick("subject")
        .contains("to 5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu");
      cy.pick("transactions")
        .children(".item")
        .eq(0)
        .get(".description")
        .pick("amount")
        .contains("-112");
    });
    it("shows all correct values when viewing the specific transfer", () => {
      cy.pick("transactions").should("exist");
      cy.pick("transactions").children(".item").should("have.length", 2);
      cy.pick("transactions").children(".item").eq(0).click();

      // Now in the transaction modal, check all relevant values.
      cy.pick("transaction", "summary", "message")
        .get("h2")
        .contains("Outgoing transfer");
      cy.pick("transaction", "summary", "message", "subject").contains(
        "to 5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      );
      cy.pick("transaction", "transfer-amount", "amount").contains("-112");
      cy.pick("transaction", "transaction-fee", "amount").contains("-0.000001");
      cy.pick("transaction", "total", "amount").contains("-112.000001");
      cy.pick("transaction", "funding-source").contains("coolorg");

      // Done
      cy.pick("modal-close-button").click();
    });
  });
});
