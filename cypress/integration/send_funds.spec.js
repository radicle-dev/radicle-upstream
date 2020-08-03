before(() => {
  cy.nukeAllState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
  cy.registerOrg("coolorg");
  // user2 has the account id 5DjHdsNAL7L7UynCDUscMsbnYd1DW4DxqJrA8wX4aS3Moyz3
  cy.registerAlternativeUser("user2");
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
    cy.pick("recipient-input").type(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
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
    cy.pick("recipient-input").type(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
    cy.pick("amount-input").type("123");
    cy.pick("send-transaction-button").click();
    cy.pick("send-funds-modal").should("exist");
  });
  it("closes the modal by clicking X", () => {
    cy.pick("Wallet").click();
    cy.pick("send-receive").should("exist");
    cy.pick("send-tab").click();
    cy.pick("send").should("exist");
    cy.pick("recipient-input").type(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
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
    cy.pick("recipient-input").type(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
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
    cy.pick("recipient-input").type(
      "5DjHdsNAL7L7UynCDUscMsbnYd1DW4DxqJrA8wX4aS3Moyz3"
    );
    cy.pick("amount-input").type("123");
    cy.pick("send-transaction-button").click();
    cy.pick("send-funds-modal").should("exist");
    cy.pick("page", "preperation-step").should("exist");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").click();
    cy.pick("page", "review-step").should("exist");
    cy.pick("back-to-review-button").should("exist");
    cy.pick("back-to-review-button").click();
    cy.pick("preperation-step").should("exist");
  });
  it("is able to go to review step and submit", () => {
    cy.pick("Wallet").click();
    cy.pick("send-receive").should("exist");
    cy.pick("send-tab").click();
    cy.pick("send").should("exist");
    cy.pick("recipient-input").type(
      "5DjHdsNAL7L7UynCDUscMsbnYd1DW4DxqJrA8wX4aS3Moyz3"
    );
    cy.pick("amount-input").type("123");
    cy.pick("send-transaction-button").click();
    cy.pick("send-funds-modal").should("exist");
    cy.pick("page", "preperation-step").should("exist");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").click();
    cy.pick("page", "review-step").should("exist");
    cy.pick("submit-tranfer-button").should("exist");
    cy.pick("submit-tranfer-button").click();
    cy.pick("Wallet").should("exist");
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
    cy.pick("recipient-input").type(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
    cy.pick("amount-input").type("123");
    cy.pick("send-transaction-button").click();
    cy.pick("send-funds-modal").should("exist");
    cy.pick("preperation-step").should("exist");
  });
  it("checks if it does the validation and disables button when removing prefilled recipient", () => {
    cy.pick("modal-recipient-input").should("exist");
    cy.pick("modal-recipient-input").clear();
    cy.pick("page").contains("The recipient address is required");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if it does the validation and disables button when removing prefilled amount", () => {
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("page").contains("The amount is required");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if it does the validation and disables button when removing prefilled recipient and amount", () => {
    cy.pick("modal-recipient-input").should("exist");
    cy.pick("modal-recipient-input").clear();
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("page").contains("The recipient address is required");
    cy.pick("page").contains("The amount is required");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if the address is a valid address that exists", () => {
    cy.pick("modal-recipient-input").should("exist");
    cy.pick("modal-recipient-input").clear();
    cy.pick("modal-recipient-input").type("bla");
    cy.pick("page").contains("Cannot find this address");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if the amount is a valid", () => {
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("modal-amount-input").type(".0");
    cy.pick("page").contains("must be a valid number");
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if the amount is covered by the chosen wallet", () => {
    cy.pick("modal-amount-input").should("exist");
    cy.pick("modal-amount-input").clear();
    cy.pick("modal-amount-input").type("325");
    cy.pick("page").contains(
      "You don't have enough funds in this wallet for this transfer"
    );
    cy.pick("review-transfer-button").should("be.disabled");
  });
  it("checks if review step contains filled in information", () => {
    cy.pick("page", "preperation-step").should("exist");
    cy.pick("review-transfer-button").should("exist");
    cy.pick("review-transfer-button").click();
    cy.pick("page", "review-step").should("exist");
    cy.pick("page").contains(
      "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
    );
    cy.pick("page").contains("123");
    cy.pick("page").contains("coolorg");
  });
});
