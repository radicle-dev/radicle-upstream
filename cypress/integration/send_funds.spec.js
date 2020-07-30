before(() => {
  cy.nukeAllState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
  cy.registerOrg("coolorg");
});

context("navigation", () => {
  beforeEach(() => {
    cy.visit("public/index.html");
  });
  it("opens the modal from the user wallet", () => {
    // user wallet opens modal
    cy.pick("Wallet").click();
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
  it("opens the modal from the org wallet", () => {
    // org wallet opens modal
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("Wallet").click();
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
  it("closes the modal by clicking X", () => {
    cy.pick("Wallet").click();
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
    cy.pick("modal-close-button").should("exist");
    cy.pick("modal-close-button").click();
    cy.pick("Wallet").should("exist");
  });
  it("closes the modal by pressing esc", () => {
    cy.pick("Wallet").click();
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
    cy.get("body").type("{esc}");
    cy.pick("Wallet").should("exist");
  });
  it("is able to go to review step and back when everything is valid", () => {
    cy.pick("Wallet").click();
    cy.pick("send-receive").should("exist");
    cy.pick("send-tab").click();
    cy.pick("send").should("exist");
    cy.pick("recipient-input").should("exist");
    cy.pick("recipient-input").type("c");
    cy.pick("amount-input").should("exist");
    cy.pick("amount-input").type("123");
    cy.pick("send-transaction-button").click();
    cy.pick("send-funds-modal").should("exist");
    cy.pick("page", "preperation-step").should("exist");
    cy.pick("review-tranfer-button").should("exist");
    cy.pick("review-tranfer-button").click();
    cy.pick("page", "review-step").should("exist");
    cy.pick("back-to-review-button").should("exist");
    cy.pick("back-to-review-button").click();
    cy.pick("preperation-step").should("exist");
  });
});
context("validation", () => {
  beforeEach(() => {
    // start a transaction from an org
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("Wallet").click();
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
    cy.pick("preperation-step").should("exist");
  });
  it("checks if it does the validation and disables button when removing prefilled recipient", () => {
    cy.pick("modal-recipient-input").should("exist");
    cy.pick("modal-recipient-input").clear();
    cy.pick("page").contains("Receipient address is required");
    cy.pick("review-tranfer-button").should("exist");
    cy.pick("review-tranfer-button").should("be.disabled");
  });
  it("checks if it does the validation and disables button when removing prefilled amount", () => {
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("page").contains("Transfer amount is required");
    cy.pick("review-tranfer-button").should("exist");
    cy.pick("review-tranfer-button").should("be.disabled");
  });
  it("checks if it does the validation and disables button when removing prefilled recipient and amount", () => {
    cy.pick("modal-recipient-input").should("exist");
    cy.pick("modal-recipient-input").clear();
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("page").contains("Transfer amount is required");
    cy.pick("page").contains("Receipient address is required");
    cy.pick("review-tranfer-button").should("exist");
    cy.pick("review-tranfer-button").should("be.disabled");
  });
});
