context("org registration", () => {
  beforeEach(() => {
    cy.createIdentity();
    cy.visit("public/index.html");
    cy.select("sidebar", "add-org-button").click();
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.select("org-reg-modal").contains("Register an org");
      cy.select("org-reg-modal", "cancel-button").click();
      cy.select("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.select("org-reg-modal").contains("Register an org");
      cy.get("body").type("{esc}");
      cy.select("profile-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      cy.select("org-reg-modal", "name-input").type("coolname");
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("org-reg-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.select("org-reg-modal", "cancel-button").click();
      cy.select("org-reg-modal", "name-input").should("exist");

      // form -> tx confirmation -> submit
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("org-reg-modal", "summary").should("exist");
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    it("prevents the user from registering an invalid org name", () => {
      // no empty input
      cy.select("org-reg-modal", "name-input").type("a_name");
      cy.select("org-reg-modal", "name-input").clear();
      cy.select("org-reg-modal").contains("Org name is required");
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no spaces
      cy.select("org-reg-modal", "name-input").type("no spaces");
      cy.select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no special characters
      cy.select("org-reg-modal", "name-input").clear();
      cy.select("org-reg-modal", "name-input").type("^^^inVaLiD***");
      cy.select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no starting with an underscore or dash
      cy.select("org-reg-modal", "name-input").clear();
      cy.select("org-reg-modal", "name-input").type("_nVaLiD");
      cy.select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      cy.select("org-reg-modal", "name-input").clear();
      cy.select("org-reg-modal", "name-input").type("-alsoInVaLiD");
      cy.select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // must meet minimum length
      cy.select("org-reg-modal", "name-input").clear();
      cy.select("org-reg-modal", "name-input").type("x");
      cy.select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");
    });

    it("prevents the user from registering an unavailable org name", () => {
      cy.select("org-reg-modal", "name-input").type("coolname");
      cy.select("org-reg-modal").contains("Sorry, this name is already taken");
      cy.select("org-reg-modal", "submit-button").should("be.disabled");
    });
  });
});
