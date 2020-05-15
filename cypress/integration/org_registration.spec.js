context("org registration", () => {
  beforeEach(() => {
    cy.nukeSessionState();
    cy.createIdentity();
    cy.registerUser();

    cy.visit("public/index.html");
    cy.pick("sidebar", "add-org-button").click();
  });

  context("navigation", () => {
    beforeEach(() => cy.pick("sidebar", "add-org-button").click());

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
    before(() => {
      cy.pick("sidebar", "add-org-button").click();
    });

    it("prevents the user from registering an invalid org id", () => {
      // no empty input
      cy.pick("org-reg-modal", "input").type("a_name");
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal").contains("Org id is required");
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no spaces
      cy.pick("org-reg-modal", "input").type("no spaces");
      cy.pick("org-reg-modal").contains(
        "Org name should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no special characters
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("^^^inVaLiD***");
      cy.pick("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // no starting with an underscore or dash
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("_nVaLiD");
      cy.pick("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("-alsoInVaLiD");
      cy.pick("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");

      // must meet minimum length
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("x");
      cy.pick("org-reg-modal").contains(
        "Org id should match [a-z0-9][a-z0-9_-]+"
      );
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");
    });

    it("prevents the user from registering an unavailable org id", () => {
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("coolname");
      cy.pick("org-reg-modal").contains("Sorry, this id is already taken");
      cy.pick("org-reg-modal", "submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("sick_org");
      cy.pick("org-reg-modal", "avatar").should("be.visible");

      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "avatar").should("not.be.visible");
    });
  });

  context("transaction", () => {
    before(() => {
      cy.nukeAllState();
      cy.createIdentity();
      cy.registerUser();
      cy.visit("public/index.html");
      cy.pick("sidebar", "add-org-button").click();
    });
    // TODO(sos): add tests for tx costs/wallet when it makes sense to do so
    it("shows correct transaction details for confirmation", () => {
      cy.pick("org-reg-modal", "input").clear();
      cy.pick("org-reg-modal", "input").type("mariposa");
      cy.pick("org-reg-modal", "submit-button").click();

      cy.pick("org-reg-modal", "message").contains("Org registration");
      cy.pick("org-reg-modal", "subject").contains("mariposa");
      cy.pick("org-reg-modal", "subject-avatar", "emoji").should(
        "have.class",
        "square"
      );
    });

    it("submits correct transaction details to proxy", () => {
      cy.pick("org-reg-modal", "submit-button").click();
      cy.pick("accordion").click();

      // pick most recent transaction
      cy.pick("accordion", "cards", "card").last().click();
      cy.pick("summary", "message").contains("Org registration");
      cy.pick("summary", "subject").contains("mariposa");
      cy.pick("summary", "subject-avatar", "emoji").should(
        "have.class",
        "square"
      );
    });
  });
});
