context("user registration", () => {
  before(() => {
    cy.nukeAllState();
    cy.nukeCache();
    cy.registerAlternativeUser("nope");
    cy.createProjectWithFixture();
  });

  beforeEach(() => {
    cy.nukeSessionState();
    cy.createIdentity();

    cy.visit("public/index.html");
    cy.pick("profile-context-menu").click();
    cy.pick("dropdown-menu", "register-handle").click();
  });

  context("modal navigation", () => {
    it("can be closed by pressing cancel", () => {
      cy.pick("page", "register-user").should("exist");
      cy.pick("register-user", "cancel-button").click();
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("page", "register-user").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
    });

    // navigation between pick handle (1) and submit tx (2) steps
    it("moves through the views by pressing navigation buttons", () => {
      // 1 -> 2
      cy.pick("handle").should("exist");
      cy.pick("page", "handle").type("testy");
      cy.pick("next-button").click();
      cy.pick("summary").should("exist");
      // 2 -> 1
      cy.pick("back-button").click();
      cy.pick("handle").should("exist");
      // 1 -> 2
      cy.pick("next-button").click();
      cy.pick("summary").should("exist");
      // 2 -> close modal
      cy.pick("submit-button").click();
      cy.pick("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    it("prevents the user from registering an invalid handle", () => {
      // shows a validation message when handle is not present
      cy.pick("handle").clear();
      cy.pick("page").contains("Id is required");

      // shows a validation message when handle contains invalid characters
      // spaces are not allowed
      cy.pick("handle").type("no spaces");
      cy.pick("page").contains("Id should match ^[a-z0-9][a-z0-9]+$");

      // special characters are disallowed
      cy.pick("handle").clear();
      cy.pick("handle").type("$bad");
      cy.pick("page").contains("Id should match ^[a-z0-9][a-z0-9]+$");

      // can't start with an underscore
      cy.pick("handle").clear();
      cy.pick("handle").type("_nein");
      cy.pick("page").contains("Id should match ^[a-z0-9][a-z0-9]+$");

      // can't start with a dash
      cy.pick("handle").clear();
      cy.pick("handle").type("-nope");
      cy.pick("page").contains("Id should match ^[a-z0-9][a-z0-9]+$");

      // has to be at least two characters long
      cy.pick("handle").clear();
      cy.pick("handle").type("x");
      cy.pick("page").contains("Id should match ^[a-z0-9][a-z0-9]+$");
    });

    it("prevents the user from registering an unavailable handle", () => {
      cy.pick("handle").clear();
      cy.pick("handle").type("nope");
      cy.pick("page").contains("Id already taken");
    });
  });

  context("transaction", () => {
    before(() => {
      // Clear everything again so transaction center is empty
      cy.nukeAllState();
      cy.nukeCache();
      cy.createProjectWithFixture();
    });

    it("shows the correct transaction details for confirmation", () => {
      cy.pick("next-button").click();

      cy.pick("message").contains("User registration");
      cy.pick("subject").contains("secretariat");

      cy.pick("subject-avatar", "emoji").should("have.class", "circle");
      cy.pick("subject", "emoji").find("img").should("have.attr", "alt", "🏯");
      cy.pick("subject", "emoji").should(
        "have.css",
        "background-color",
        "rgb(185, 118, 211)"
      );

      cy.pick("deposit", "rad-amount").contains("0.00001");
      cy.pick("deposit", "usd-amount").contains("$0.00001");

      cy.pick("transaction-fee", "rad-amount").contains("0.000001");
      cy.pick("transaction-fee", "usd-amount").contains("$0.000001");

      cy.pick("total", "rad-amount").contains("0.000011");
      cy.pick("total", "usd-amount").contains("$0.000011");
    });

    it("submits correct transaction details to proxy", () => {
      cy.pick("next-button").click();
      cy.pick("submit-button").click();

      cy.pick("transaction-center").click();

      // pick most recent transaction
      cy.pick("transaction-item").last().click();
      cy.pick("summary", "message").contains("User registration");
      cy.pick("summary", "subject").contains("secretariat");

      cy.pick("summary", "subject-avatar", "emoji").should(
        "have.class",
        "circle"
      );
      cy.pick("subject", "emoji").find("img").should("have.attr", "alt", "🏯");
      cy.pick("subject", "emoji").should(
        "have.css",
        "background-color",
        "rgb(185, 118, 211)"
      );

      cy.pick("deposit", "rad-amount").contains("0.00001");
      cy.pick("deposit", "usd-amount").contains("$0.00001");

      cy.pick("transaction-fee", "rad-amount").contains("0.000001");
      cy.pick("transaction-fee", "usd-amount").contains("$0.000001");

      cy.pick("total", "rad-amount").contains("0.000011");
      cy.pick("total", "usd-amount").contains("$0.000011");
    });
  });

  context("permissions", () => {
    before(() => {
      cy.nukeAllState();
      cy.nukeCache();
    });

    it("only allows to register a handle once", () => {
      cy.pick("next-button").click();
      cy.pick("submit-button").click();
      cy.pick("profile-context-menu").click();
      cy.pick("dropdown-menu", "register-handle").should("not.exist");
    });
  });
});
