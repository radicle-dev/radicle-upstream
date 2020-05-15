context("org registration", () => {
  before(() => {
    cy.nukeAllState();
    cy.createIdentity();
    cy.registerUser();
    cy.visit("public/index.html");
  });

  context("navigation", () => {
    beforeEach(() => cy.select("sidebar", "add-org-button").click());

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
      cy.select("org-reg-modal", "input").type("coolname");
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("org-reg-modal", "summary").should("exist");

      // tx confirmation -> form
      cy.select("org-reg-modal", "cancel-button").click();
      cy.select("org-reg-modal", "input").should("exist");

      // form -> tx confirmation -> submit
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("org-reg-modal", "summary").should("exist");
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    before(() => {
      cy.select("sidebar", "add-org-button").click();
    });

    it("prevents the user from registering an invalid org id", () => {
      // no empty input
      cy.select("org-reg-modal", "input").type("a_name");
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal").contains("Org id is required");
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no spaces
      cy.select("org-reg-modal", "input").type("no spaces");
      cy.select("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no special characters
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("^^^inVaLiD***");
      cy.select("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // no starting with an underscore or dash
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("_nVaLiD");
      cy.select("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("-alsoInVaLiD");
      cy.select("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");

      // must meet minimum length
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("x");
      cy.select("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.select("org-reg-modal", "submit-button").should("be.disabled");
    });

    it("prevents the user from registering an unavailable org id", () => {
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("coolname");
      cy.select("org-reg-modal").contains("Sorry, this id is already taken");
      cy.select("org-reg-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("sick_org");
      cy.select("org-reg-modal", "avatar").should("be.visible");

      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "avatar").should("not.be.visible");
    });
  });

  context("transaction", () => {
    before(() => {
      cy.nukeAllState();
      cy.createIdentity();
      cy.registerUser();
      cy.visit("public/index.html");
      cy.select("sidebar", "add-org-button").click();
    });
    // TODO(sos): add tests for tx costs/wallet when it makes sense to do so
    it("shows correct transaction details for confirmation", () => {
      cy.select("org-reg-modal", "input").clear();
      cy.select("org-reg-modal", "input").type("mariposa");
      cy.select("org-reg-modal", "submit-button").click();

      cy.select("org-reg-modal", "message").contains("Org registration");
      cy.select("org-reg-modal", "subject").contains("mariposa");
      cy.select("org-reg-modal", "subject-avatar", "emoji").should(
        "have.class",
        "square"
      );
    });

    it("submits correct transaction details to proxy", () => {
      cy.select("org-reg-modal", "submit-button").click();
      cy.select("accordion").click();

      // select most recent transaction
      cy.select("accordion", "cards", "card").last().click();
      cy.select("summary", "message").contains("Org registration");
      cy.select("summary", "subject").contains("mariposa");
      cy.select("summary", "subject-avatar", "emoji").should(
        "have.class",
        "square"
      );
    });
  });
});
