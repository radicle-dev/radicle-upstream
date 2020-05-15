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
    cy.select("sidebar", "org").click();
    cy.select("org-screen", "add-member-button").click();
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.select("add-member-modal").contains("Register a member");
      cy.select("add-member-modal", "cancel-button").click();
      cy.select("org-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.select("add-member-modal").contains("Register a member");
      cy.get("body").type("{esc}");
      cy.select("org-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.select("add-member-modal", "input").type("coolname");
      cy.select("add-member-modal", "submit-button").click();
      cy.select("add-member-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.select("add-member-modal", "cancel-button").click();
      cy.select("add-member-modal", "input").should("exist");

      // form -> tx confirmation -> submit
      cy.select("add-member-modal", "submit-button").click();
      cy.select("add-member-modal", "summary").should("exist");
      cy.select("add-member-modal", "submit-button").click();
      cy.select("org-screen").should("exist");
    });
  });

  context("validations", () => {
    it("prevents the user from adding an invalid user", () => {
      // no empty input
      cy.select("add-member-modal", "input").type("aname");
      cy.select("add-member-modal", "input").clear();
      cy.select("add-member-modal").contains("Member handle is required");
      cy.select("add-member-modal", "submit-button").should("be.disabled");

      // no non-existing users
      cy.select("add-member-modal", "input").type("aname");
      cy.select("add-member-modal").contains("Cannot find this user");
      cy.select("add-member-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.select("add-member-modal", "input").clear();
      cy.select("add-member-modal", "input").type("sickhandle");
      cy.select("add-member-modal", "avatar").should("be.visible");

      cy.select("add-member-modal", "input").clear();
      cy.select("add-member-modal", "avatar").should("not.be.visible");
    });
  });
});
