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
      cy.select("sidebar", "org").click();
      cy.select("projects", "add-member-button").click();
    });

    it("can be closed by pressing cancel", () => {
      cy.select("add-member-modal").contains("Register a member");
      cy.select("add-member-modal", "cancel-button").click();
      cy.select("projects").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.select("add-member-modal").contains("Register a member");
      cy.get("body").type("{esc}");
      cy.select("projects").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.select("add-member-modal", "name-input").type("coolname");
      cy.select("add-member-modal", "submit-button").click();
      cy.select("add-member-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.select("add-member-modal", "cancel-button").click();
      cy.select("add-member-modal", "name-input").should("exist");

      // form -> tx confirmation -> submit
      cy.select("add-member-modal", "submit-button").click();
      cy.select("add-member-modal", "summary").should("exist");
      cy.select("add-member-modal", "submit-button").click();
      cy.select("projects").should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.select("sidebar", "org").click();
      cy.select("projects", "add-member-button").click();
    });

    it("prevents the user from adding an invalid user", () => {
      // no empty input
      cy.select("add-member-modal", "name-input").type("aname");
      cy.select("add-member-modal", "name-input").clear();
      cy.select("add-member-modal").contains("Member name is required");
      cy.select("add-member-modal", "submit-button").should("be.disabled");

      // no non-existing users
      cy.select("add-member-modal", "name-input").type("aname");
      cy.select("add-member-modal").contains("Cannot find this user");
      cy.select("add-member-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.select("add-member-modal", "name-input").clear();
      cy.select("add-member-modal", "name-input").type("sickhandle");
      cy.select("add-member-modal", "avatar").should("be.visible");

      cy.select("add-member-modal", "name-input").clear();
      cy.select("add-member-modal", "avatar").should("not.be.visible");
    });
  });
});
