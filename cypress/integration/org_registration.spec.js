context("org registration", () => {
  beforeEach(() => {
    cy.nukeRegistryState();
    cy.nukeSessionState();
    cy.createIdentity();
    cy.registerUser();

    cy.visit("public/index.html");
    cy.pick("sidebar", "add-org-button").click();
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.pick("org-reg-modal").contains("Register an org");
      cy.pick("org-reg-modal", "cancel-button").click();
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("org-reg-modal").contains("Register an org");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.pick("org-reg-modal", "name-input").type("coolname");
      cy.pick("org-reg-modal", "submit-button").click();
      cy.pick("org-reg-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.pick("org-reg-modal", "cancel-button").click();
      cy.pick("org-reg-modal", "name-input").should("exist");

      // form -> tx confirmation -> submit
      cy.pick("org-reg-modal", "submit-button").click();
      cy.pick("org-reg-modal", "summary").should("exist");
      cy.pick("org-reg-modal", "submit-button").click();
      cy.pick("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    it("prevents the user from registering an invalid org name", () => {
      // no empty input
      cy.pick("org-reg-modal", "name-input").type("a_name");
      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal").contains("Org name is required");
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no spaces
      cy.pick("org-reg-modal", "name-input").type("no spaces");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no special characters
      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "name-input").type("^^^inVaLiD***");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no starting with an underscore or dash
      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "name-input").type("_nVaLiD");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "name-input").type("-alsoInVaLiD");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // must meet minimum length
      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "name-input").type("x");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");
    });

    it("prevents the user from registering an unavailable org name", () => {
      cy.registerOrg("coolname");

      cy.pick("org-reg-modal", "name-input").type("coolname");
      cy.pick("org-reg-modal").contains("Sorry, this name is already taken");
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "name-input").type("sick_org");
      cy.pick("org-reg-modal", "avatar").should("be.visible");

      cy.pick("org-reg-modal", "name-input").clear();
      cy.pick("org-reg-modal", "avatar").should("not.be.visible");
    });
  });
});
