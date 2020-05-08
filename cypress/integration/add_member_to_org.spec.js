before(() => {
  cy.nukeAllState();
  cy.createIdentity();
  cy.visit("public/index.html");

  // create an org
  cy.select("sidebar", "add-org-button").click();
  cy.select("org-reg-modal", "name-input").type("thatorg");
  cy.select("org-reg-modal", "submit-button").click();
  cy.select("org-reg-modal", "submit-button").click();

  cy.registerUser("coolname");
});

context("add member to org", () => {
  beforeEach(() => {
    cy.select("sidebar", "org").click();
    cy.select("projects", "add-member-button").click();
  });
  context("navigation", () => {
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
    it("prevents the user from adding an invalid user", () => {
      // no empty input
      cy.select("add-member-modal", "name-input").type("a_name");
      cy.select("add-member-modal", "name-input").clear();
      cy.select("add-member-modal").contains("Member name is required");
      cy.select("add-member-modal", "submit-button").should("be.disabled");

      // no non-existing users
      cy.select("add-member-modal", "name-input").type("aname");
      cy.select("add-member-modal").contains("Cannot find this user");
      cy.select("add-member-modal", "submit-button").should("be.disabled");
    });
  });
});
