context("org registration permission", () => {
  before(() => {
    cy.nukeAllState();
    cy.createIdentity();
    cy.visit("public/index.html");
  });

  it("disables the add org sidebar button before user registration", () => {
    cy.pick("add-org", "add-org-button").should("have.class", "disabled");
  });

  it("enables the add org sidebar button after user registration", () => {
    cy.registerUser();
    cy.visit("public/index.html");
    cy.pick("sidebar", "add-org").click();
    cy.pick("org-reg-modal").contains("Register an org");
  });
});

context("org registration", () => {
  beforeEach(() => {
    cy.nukeAllState();
    cy.createIdentity();
    cy.registerUser();
    cy.createProjectWithFixture();

    cy.visit("public/index.html");
    cy.pick("sidebar", "add-org").click();
  });

  context("navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.pick("org-reg-modal").contains("Register an org");
      cy.pick("cancel-button").click();
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("org-reg-modal").contains("Register an org");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
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
      cy.pick("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    // TODO: Fix validation bug in https://github.com/radicle-dev/radicle-upstream/issues/492
    it.skip("prevents the user from registering an invalid org id", () => {
      // no empty input
      cy.pick("input").type("a_name");
      cy.pick("input").clear();
      cy.pick("org-reg-modal").contains("Id is required");
      cy.pick("submit-button").should("be.disabled");

      // no spaces
      cy.pick("input").type("no spaces");
      cy.pick("org-reg-modal").contains("Id should match ^[a-z0-9][a-z0-9]+$");
      cy.pick("submit-button").should("be.disabled");

      // no special characters
      cy.pick("input").clear();
      cy.pick("input").type("^^^inVaLiD***");
      cy.pick("org-reg-modal").contains("Id should match ^[a-z0-9][a-z0-9]+$");
      cy.pick("submit-button").should("be.disabled");

      // no starting with an underscore or dash
      cy.pick("input").clear();
      cy.pick("input").type("_nVaLiD");
      cy.pick("org-reg-modal").contains("Id should match ^[a-z0-9][a-z0-9]+$");
      cy.pick("submit-button").should("be.disabled");

      cy.pick("input").clear();
      cy.pick("input").type("-alsoInVaLiD");
      cy.pick("org-reg-modal").contains("Id should match ^[a-z0-9][a-z0-9]+$");
      cy.pick("submit-button").should("be.disabled");

      // must meet minimum length
      cy.pick("input").clear();
      cy.pick("input").type("x");
      cy.pick("org-reg-modal").contains("Id should match ^[a-z0-9][a-z0-9]+$");
      cy.pick("submit-button").should("be.disabled");
    });

    // TODO: Fix validation bug in https://github.com/radicle-dev/radicle-upstream/issues/492
    it.skip("prevents the user from registering an unavailable org id", () => {
      cy.registerOrg("coolname");

      cy.pick("org-reg-modal", "input").type("coolname");
      cy.pick("org-reg-modal").contains("Sorry, this id is already taken");
      cy.pick("submit-button").should("be.disabled");
    });
  });

  context("aesthetics", () => {
    it("shows avatar when handle exists and hides otherwise", () => {
      cy.pick("input").clear();
      cy.pick("input").type("sick_org");
      cy.pick("avatar").should("be.visible");

      cy.pick("input").clear();
      cy.pick("avatar").should("not.be.visible");
    });
  });

  context("transaction", () => {
    // TODO(sos): add tests for wallet when it makes sense to do so
    it("shows correct transaction details for confirmation", () => {
      cy.pick("input").type("mariposa");
      cy.pick("submit-button").click();

      cy.pick("message").contains("Org registration");
      cy.pick("subject").contains("mariposa");
      cy.pick("subject-avatar", "emoji").should("have.class", "square");

      cy.pick("deposit", "rad-amount").contains("0.00001");
      cy.pick("deposit", "usd-amount").contains("$0.00001");

      cy.pick("transaction-fee", "rad-amount").contains("0.000001");
      cy.pick("transaction-fee", "usd-amount").contains("$0.000001");

      cy.pick("total", "rad-amount").contains("0.000011");
      cy.pick("total", "usd-amount").contains("$0.000011");
    });

    it("submits correct transaction details to proxy", () => {
      cy.pick("input").type("mariposa");
      cy.pick("submit-button").click();
      cy.pick("submit-button").click();

      // wait until routed back to main screen
      cy.pick("sidebar").should("exist");
      cy.pick("transaction-center").click();

      // pick most recent transaction
      cy.pick("transaction-center", "transaction-item").first().click();
      cy.pick("summary", "message").contains("Org registration");
      cy.pick("summary", "subject").contains("mariposa");
      cy.pick("summary", "subject-avatar", "emoji").should(
        "have.class",
        "square"
      );
      cy.pick("subject", "emoji").find("img").should("have.attr", "alt", "🐲");
      cy.pick("subject", "emoji").should(
        "have.css",
        "background-color",
        "rgb(186, 38, 114)"
      );

      cy.pick("deposit", "rad-amount").contains("0.00001");
      cy.pick("deposit", "usd-amount").contains("$0.00001");

      cy.pick("transaction-fee", "rad-amount").contains("0.000001");
      cy.pick("transaction-fee", "usd-amount").contains("$0.000001");

      cy.pick("total", "rad-amount").contains("0.000011");
      cy.pick("total", "usd-amount").contains("$0.000011");
    });
  });
});
