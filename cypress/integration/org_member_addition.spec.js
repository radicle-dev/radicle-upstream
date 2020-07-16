before(() => {
  cy.nukeAllState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
  cy.registerOrg("coolorg");
  cy.registerAlternativeUser("user2");
});

context("add member to org", () => {
  beforeEach(() => {
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("org-screen", "empty-state", "primary-action").click();
  });

  context("validations", () => {
    it("prevents the user from adding an invalid user", () => {
      // no empty input
      cy.pick("input").type("aname");
      cy.pick("input").clear();
      cy.pick("add-member-modal").contains("Member handle is required");
      cy.pick("submit-button").should("be.disabled");

      // no non-existing users
      cy.pick("input").type("aname");
      cy.pick("add-member-modal").contains("Cannot find this user");
      cy.pick("submit-button").should("be.disabled");

      // no users that are already members
      cy.pick("input").clear();
      cy.pick("input").type("coolname");
      cy.pick("add-member-modal").contains("This user is already a member");
      cy.pick("submit-button").should("be.disabled");
    });
  });

  context("transaction confirmation", () => {
    it("shows correct transaction details", () => {
      cy.pick("input").type("user2");
      cy.pick("submit-button").click();

      // check the transaction details before submition
      cy.pick("message").contains("Member registration");
      cy.pick("subject").contains("user2");

      cy.pick("deposit", "amount").contains("0.00001");
      cy.pick("deposit", "amount").trigger("mouseover");
      cy.pick("tooltip").contains("$0.00001");
      cy.pick("deposit", "amount").trigger("mouseout");

      cy.pick("transaction-fee", "amount").contains("0.000001");
      cy.pick("transaction-fee", "amount").trigger("mouseover");
      cy.pick("tooltip").contains("$0.000001");
      cy.pick("transaction-fee", "amount").trigger("mouseout");

      cy.pick("total", "amount").contains("0.000011");
      cy.pick("total", "amount").trigger("mouseover");
      cy.pick("tooltip").contains("$0.000011");
      cy.pick("total", "amount").trigger("mouseout");
    });
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.pick("add-member-modal").contains("Member registration");
      cy.pick("cancel-button").click();
      cy.pick("org-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("add-member-modal").contains("Member registration");
      cy.get("body").type("{esc}");
      cy.pick("org-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.pick("input").type("user2");
      cy.pick("submit-button").click();
      cy.pick("summary").should("exist");

      // tx confirmation -> form
      cy.pick("cancel-button").click();
      cy.pick("input").should("exist");

      // form -> tx confirmation -> submit
      cy.pick("submit-button").click();
      cy.pick("summary").should("exist");
      cy.pick("submit-button").click();
      cy.pick("org-screen").should("exist");
    });
  });
});

context("after submitting the transaction", () => {
  it("shows correct transaction details", () => {
    // Register a new member
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();

    // pick most recent transaction to check the transaction details
    cy.pick("transaction-center").click();
    cy.pick("transaction-center", "transaction-item").first().click();

    cy.pick("deposit", "amount").contains("0.00001");
    cy.pick("deposit", "amount").trigger("mouseover");
    cy.pick("tooltip").contains("$0.00001");
    cy.pick("deposit", "amount").trigger("mouseout");

    cy.pick("transaction-fee", "amount").contains("0.000001");
    cy.pick("transaction-fee", "amount").trigger("mouseover");
    cy.pick("tooltip").contains("$0.000001");
    cy.pick("transaction-fee", "amount").trigger("mouseout");

    cy.pick("total", "amount").contains("0.000011");
    cy.pick("total", "amount").trigger("mouseover");
    cy.pick("tooltip").contains("$0.000011");
    cy.pick("total", "amount").trigger("mouseout");

    cy.pick("summary", "message").contains("Member registration");
    cy.pick("summary", "subject").contains("user2");
  });

  it("shows both users in the list", () => {
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("horizontal-menu", "Members").click();
    cy.pick("member-list").contains("coolname");
    cy.pick("member-list").contains("user2");
  });
});
