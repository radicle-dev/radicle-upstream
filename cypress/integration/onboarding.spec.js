context("identity creation", () => {
  const validUser = {
    handle: "rafalca",
    passphrase: "curled",
  };

  beforeEach(() => {
    cy.nukeAllState();
    cy.visit("./public/index.html");
    cy.pick("welcome-screen").should("exist");
  });

  context("navigation", () => {
    it("is not possible to exit the flow by pressing escape key", () => {
      cy.pick("get-started-button").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("get-started-button").should("exist");
    });

    it("makes sure global hotkeys are disabled during onboarding", () => {
      cy.pick("get-started-button").should("exist");
      cy.get("body").type("?");
      cy.pick("hotkey-modal").should("not.exist");
    });

    it("is possible to use the keyboard for navigation", () => {
      // Intro screen.
      cy.get("body").type("{enter}");

      // Enter name screen.
      cy.focused().type(validUser.handle);
      cy.focused().type("{enter}");

      // Enter passphrase screen.
      cy.pick("enter-name-screen").should("exist");
      cy.focused().type(validUser.passphrase);
      cy.focused().type("{enter}");
      cy.focused().type(validUser.passphrase);
      cy.focused().type("{enter}");

      // Success screen.
      cy.pick("urn")
        .contains(/rafalca@/)
        .should("exist");

      // Land on profile screen.
      cy.get("body").type("{enter}");
      cy.pick("entity-name").contains(validUser.handle);
    });

    it("is possible to step through the identity creation flow", () => {
      // Intro screen.
      cy.pick("get-started-button").click();

      // Enter name screen.
      cy.pick("handle-input").type(validUser.handle);
      cy.pick("next-button").click();

      // Enter passphrase screen.
      cy.pick("passphrase-input").type(validUser.passphrase);
      cy.pick("repeat-passphrase-input").type(validUser.passphrase);
      cy.pick("set-passphrase-button").click();

      // Success screen.
      cy.pick("urn")
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

      cy.pick("handle-input").type(validUser.handle);
      cy.pick("next-button").click();

      cy.pick("passphrase-input").type(validUser.passphrase);
      cy.pick("repeat-passphrase-input").type(validUser.passphrase);
      cy.pick("set-passphrase-button").click();
      cy.pick("notification")
        .contains(
          /Could not create identity: the identity 'rad:git:[\w]{3}…[\w]{3}' already exits/
        )
        .should("exist");
      cy.pick("notification").contains("Close").click();
      cy.contains("what should we call you?").should("exist");

      // We can create a different identity with a new handle.
      cy.pick("handle-input").clear();
      cy.pick("handle-input").type("cloudhead");
      cy.pick("next-button").click();
      cy.pick("passphrase-input").type("1234");
      cy.pick("repeat-passphrase-input").type("1234");
      cy.pick("set-passphrase-button").click();
      cy.pick("urn")
        .contains(/cloudhead@/)
        .should("exist");
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains("cloudhead");
    });

    context("when clicking the back button on the passphrase screen", () => {
      it("sends the user back to the previous screen", () => {
        cy.pick("get-started-button").click();
        cy.pick("handle-input").type(validUser.handle);
        cy.pick("next-button").click();
        cy.pick("enter-passphrase-screen").should("exist");
        cy.pick("back-button").click();
        cy.contains("what should we call you?").should("exist");
        cy.pick("handle-input").should("have.value", validUser.handle);
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

        cy.pick("handle-input").type("_rafalca");
        cy.pick("next-button").click();

        // Handle is required.
        cy.pick("handle-input").clear();
        cy.pick("enter-name-screen").contains("You must provide a handle");

        // No spaces.
        cy.pick("handle-input").type("no spaces");
        cy.pick("enter-name-screen").contains(validationError);

        // No special characters.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("$bad");
        cy.pick("enter-name-screen").contains(validationError);

        // Can't start with an underscore.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("_nein");
        cy.pick("enter-name-screen").contains(validationError);

        // Can't start with a dash.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("-não");
        cy.pick("enter-name-screen").contains(validationError);

        // Has to be at least two characters long.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("x");
        cy.pick("enter-name-screen").contains(validationError);
      });
    });

    context("passphrase", () => {
      it("prevents the user from submitting an invalid passphrase", () => {
        cy.pick("handle-input").type("cloudhead");
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
