before(() => {
  cy.nukeRegistryState();
  cy.nukeSessionState();

  cy.createIdentity();
  cy.registerOrg("coolorg");
  cy.registerUser("coolname");

  cy.visit("public/index.html");
});

context("add member to org", () => {
  context("navigation", () => {
    beforeEach(() => {
      cy.pick("sidebar", "org").click();
      cy.pick("projects", "add-member-button").click();
    });

    it("can be closed by pressing cancel", () => {
      cy.pick("add-member-modal").contains("Register a member");
      cy.pick("add-member-modal", "cancel-button").click();
      cy.pick("projects").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("add-member-modal").contains("Register a member");
      cy.get("body").type("{esc}");
      cy.pick("projects").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.pick("add-member-modal", "name-input").type("coolname");
      cy.pick("add-member-modal", "submit-button").click();
      cy.pick("add-member-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.pick("add-member-modal", "cancel-button").click();
      cy.pick("add-member-modal", "name-input").should("exist");

      // form -> tx confirmation -> submit
      cy.pick("add-member-modal", "submit-button").click();
      cy.pick("add-member-modal", "summary").should("exist");
      cy.pick("add-member-modal", "submit-button").click();
      cy.pick("projects").should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.pick("sidebar", "org").click();
      cy.pick("projects", "add-member-button").click();
    });

    it("prevents the user from adding an invalid user", () => {
      // no empty input
      cy.pick("add-member-modal", "name-input").type("aname");
      cy.pick("add-member-modal", "name-input").clear();
      cy.pick("add-member-modal").contains("Member name is required");
      cy.pick("add-member-modal", "submit-button").should("be.disabled");

      // no non-existing users
      cy.pick("add-member-modal", "name-input").type("aname");
      cy.pick("add-member-modal").contains("Cannot find this user");
      cy.pick("add-member-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.pick("add-member-modal", "name-input").clear();
      cy.pick("add-member-modal", "name-input").type("sickhandle");
      cy.pick("add-member-modal", "avatar").should("be.visible");

      cy.pick("add-member-modal", "name-input").clear();
      cy.pick("add-member-modal", "avatar").should("not.be.visible");
    });
  });
});
