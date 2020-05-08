const select = (...ids) => {
  const selectorString = ids.map((id) => `[data-cy="${id}"]`).join(" ");
  return cy.get(selectorString);
};

context("org registration", () => {
  beforeEach(() => {
    cy.createIdentity();
    cy.visit("public/index.html");
    select("sidebar");
    select("sidebar", "add-org-button").click();
  });

  context("modal navigation", () => {
    it("can be closed by pressing cancel", () => {
      select("org-reg-modal").contains("Register an org");
      select("org-reg-modal", "cancel-button").click();
      select("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      select("org-reg-modal").contains("Register an org");
      cy.get("body").type("{esc}");
      select("profile-screen").should("exist");
    });

    it("can be traversed with navigation buttons", () => {
      // form -> tx confirmation
      select("org-reg-modal", "name-input").type("coolname");
      select("org-reg-modal", "submit-button").click();
      select("org-reg-modal", "summary").should("exist");

      // tx confirmation -> form
      select("org-reg-modal", "cancel-button").click();
      select("org-reg-modal", "name-input").should("exist");

      // form -> tx confirmation -> submit
      select("org-reg-modal", "submit-button").click();
      select("org-reg-modal", "summary").should("exist");
      select("org-reg-modal", "submit-button").click();
      select("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    it("prevents the user from registering an invalid org name", () => {
      // no empty input
      select("org-reg-modal", "name-input").type("a_name");
      select("org-reg-modal", "name-input").clear();
      select("org-reg-modal").contains("Org name is required");
      select("org-reg-modal", "submit-button").should("be.disabled");

      // no spaces
      select("org-reg-modal", "name-input").type("no spaces");
      select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      select("org-reg-modal", "submit-button").should("be.disabled");

      // no special characters
      select("org-reg-modal", "name-input").clear();
      select("org-reg-modal", "name-input").type("^^^inVaLiD***");
      select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      select("org-reg-modal", "submit-button").should("be.disabled");

      // no starting with an underscore or dash
      select("org-reg-modal", "name-input").clear();
      select("org-reg-modal", "name-input").type("_nVaLiD");
      select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      select("org-reg-modal", "submit-button").should("be.disabled");

      select("org-reg-modal", "name-input").clear();
      select("org-reg-modal", "name-input").type("-alsoInVaLiD");
      select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      select("org-reg-modal", "submit-button").should("be.disabled");

      // must meet minimum length
      select("org-reg-modal", "name-input").clear();
      select("org-reg-modal", "name-input").type("x");
      select("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      select("org-reg-modal", "submit-button").should("be.disabled");
    });

    it("prevents the user from registering an unavailable org name", () => {
      select("org-reg-modal", "name-input").type("coolname");
      select("org-reg-modal").contains("Sorry, this name is already taken");
      select("org-reg-modal", "submit-button").should("be.disabled");
    });
  });
});
