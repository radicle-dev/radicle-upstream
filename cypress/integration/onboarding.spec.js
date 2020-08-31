context("identity creation", () => {
  const validUser = {
    handle: "rafalca",
    passphrase: "curled",
  };

  beforeEach(() => {
    cy.nukeCocoState();
    cy.nukeSessionState();
    cy.visit("./public/index.html");
  });

  context("modal", () => {
    it("can't be closed by pressing escape key", () => {
      cy.pick("get-started-button").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("get-started-button").should("exist");
    });
  });

  context("navigation", () => {
    it("is possible to step through the identity creation flow", () => {
      // Intro screen.
      cy.pick("get-started-button").click();

      // Enter name screen.
      cy.pick("form", "handle").type(validUser.handle);
      cy.pick("next-button").click();

      // Enter passphrase screen.
      cy.pick("passphrase-input").type(validUser.passphrase);
      cy.pick("repeat-passphrase-input").type(validUser.passphrase);
      cy.pick("set-passphrase-button").click();

      // Success screen.
      cy.pick("shareable-identifier")
        .contains(/rafalca@/)
        .should("exist");

      // Land on profile screen.
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains(validUser.handle);

      // Clear session to restart onboarding.
      cy.pick("sidebar", "settings").click();
      cy.pick("clear-session-button").click();
      cy.contains("A free and open-source way to host").should("exist");

      // When creating the same identity again without nuking all data, it
      // should show an error and return to the name entry screen.
      cy.pick("get-started-button").click();

      cy.pick("form", "handle").type(validUser.handle);
      cy.pick("next-button").click();

      cy.pick("passphrase-input").type(validUser.passphrase);
      cy.pick("repeat-passphrase-input").type(validUser.passphrase);
      cy.pick("set-passphrase-button").click();
      cy.pick("notification")
        .contains(
          /Could not create identity: the identity '[\w]*' already exits/
        )
        .should("exist");
      cy.pick("notification").contains("Close").click();
      cy.contains("what should we call you?").should("exist");

      // We can create a different identity with a new handle.
      cy.pick("form", "handle").clear();
      cy.pick("form", "handle").type("cloudhead");
      cy.pick("next-button").click();
      cy.pick("passphrase-input").type("1234");
      cy.pick("repeat-passphrase-input").type("1234");
      cy.pick("set-passphrase-button").click();
      cy.pick("shareable-identifier")
        .contains(/cloudhead@/)
        .should("exist");
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains("cloudhead");
    });

    context("when clicking the back button on the passphrase screen", () => {
      it("sends the user back to the previous screen", () => {
        cy.pick("get-started-button").click();
        cy.pick("form", "handle").type(validUser.handle);
        cy.pick("next-button").click();
        cy.contains("you'll enter a passphrase").should("exist");
        cy.pick("back-button").click();
        cy.contains("what should we call you?").should("exist");
        cy.pick("form", "handle").should("have.value", validUser.handle);
      });
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.pick("get-started-button").click();
    });

    context("handle", () => {
      it("prevents the user from submitting an invalid handle", () => {
        const validationError = "Handle should match ^[a-z0-9][a-z0-9_-]+$";

        cy.pick("form", "handle").type("_rafalca");
        cy.pick("next-button").click();

        // Handle is required.
        cy.pick("form", "handle").clear();
        cy.pick("form").contains("You must provide a handle");

        // No spaces.
        cy.pick("form", "handle").type("no spaces");
        cy.pick("form").contains(validationError);

        // No special characters.
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("$bad");
        cy.pick("form").contains(validationError);

        // Can't start with an underscore.
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("_nein");
        cy.pick("form").contains(validationError);

        // Can't start with a dash.
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("-não");
        cy.pick("form").contains(validationError);

        // Has to be at least two characters long.
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("x");
        cy.pick("form").contains(validationError);
      });
    });

    context("passphrase", () => {
      it("prevents the user from submitting an invalid passphrase", () => {
        cy.pick("handle").type("cloudhead");
        cy.pick("next-button").click();

        // Only entered once.
        cy.pick("passphrase-input").type("123");
        cy.pick("set-passphrase-button").should("be.disabled");

        // Too short.
        cy.pick("repeat-passphrase-input").type("123");
        cy.contains("Passphrase must be at least 4 characters").should("exist");

        // Does not match.
        cy.pick("passphrase-input").type("4");
        cy.pick("repeat-passphrase-input").type("5");
        cy.contains("Passphrases should match").should("exist");

        // Valid passphrase.
        cy.pick("passphrase-input").clear().type("abcd");
        cy.pick("repeat-passphrase-input").clear().type("abcd");
        cy.pick("set-passphrase-button").should("not.be.disabled");
      });
    });
  });
});
