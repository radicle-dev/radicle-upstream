before(() => {
  cy.nukeSessionState();
  cy.nukeRegistryState();

  cy.createIdentity("coolname");
  cy.registerUser("coolname");
  cy.registerOrg("coolorg");
});

context("add member to org", () => {
  beforeEach(() => {
    cy.visit("public/index.html");
    cy.pick("sidebar", "org-coolorg").click();
    cy.pick("org-screen", "add-member-button").click();
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.pick("add-member-modal").contains("Register a member");
      cy.pick("cancel-button").click();
      cy.pick("org-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("add-member-modal").contains("Register a member");
      cy.get("body").type("{esc}");
      cy.pick("org-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.pick("input").type("coolname");
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
    });
  });

  context("transaction", () => {
    it("shows correct transaction details for confirmation", () => {
      cy.pick("input").type("coolname");
      cy.pick("submit-button").click();

      cy.pick("message").contains("Org member registration");
      cy.pick("subject").contains("coolname");
    });

    // TODO(sos): add actual transaction details check once we can make this tx
    // it("submits correct transaction details to proxy", () => {
    //   cy.pick("input").type("coolname");
    //   cy.pick("submit-button").click();
    //   cy.pick("submit-button").click();

    // cy.pick("transaction-center").click();

    // pick most recent transaction
    // cy.pick("accordion", "cards", "card").last().click();
    // cy.pick("summary", "message").contains("Org member registration");
    // cy.pick("summary", "subject").contains("coolname");
    // cy.pick("summary", "subject-avatar", "emoji").should(
    //   "have.class",
    //   "circle"
    // );
    // });
  });
});
