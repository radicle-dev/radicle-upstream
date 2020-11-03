context("onboarding", () => {
  const validUser = {
    handle: "rafalca",
    passphrase: "curled",
  };

  const radIdRegex = /hwd[a-z0-9]{56}/;

  beforeEach(() => {
    cy.resetProxyState();
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
      cy.pick("urn").contains(radIdRegex).should("exist");

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
      cy.pick("urn").contains(radIdRegex).should("exist");

      // Land on profile screen.
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains(validUser.handle);
    });

    it("is possible to create a second identity", () => {
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
      cy.pick("urn").contains(radIdRegex).should("exist");

      // Land on profile screen.
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains(validUser.handle);

      // Clear session to restart onboarding.
      cy.pick("sidebar", "settings").click();
      cy.pick("clear-session-button").click();
      cy.contains("A free and open-source way to host").should("exist");

      cy.pick("get-started-button").click();
      cy.pick("handle-input").type("cloudhead");
      cy.pick("next-button").click();
      cy.pick("passphrase-input").type("1234");
      cy.pick("repeat-passphrase-input").type("1234");
      cy.pick("set-passphrase-button").click();
      cy.pick("urn").contains(radIdRegex).should("exist");
      cy.pick("go-to-profile-button").click();
      cy.pick("entity-name").contains("cloudhead");
    });

    it("is not possible to create the same identity again", () => {
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
      cy.pick("urn").contains(radIdRegex).should("exist");
      cy.pick("go-to-profile-button").click();

      cy.log("reset session");
      // Clear session to restart onboarding.
      cy.pick("sidebar", "settings").click();
      cy.pick("clear-session-button").click();
      cy.contains("A free and open-source way to host").should("exist");

      // When creating the same identity again without resetting all data, it
      // should show an error and return to the name entry screen.
      cy.pick("get-started-button").click();

      cy.pick("handle-input").type(validUser.handle);
      cy.pick("next-button").click();

      cy.pick("passphrase-input").type(validUser.passphrase);
      cy.pick("repeat-passphrase-input").type(validUser.passphrase);
      cy.pick("set-passphrase-button").click();
      cy.contains(
        /Could not create identity: the identity 'rad:git:[\w]{3}…[\w]{3}' already exists/
      ).should("exist");
      cy.pick("handle-input").should("exist");
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
      it("requires the user to input a handle", () => {
        cy.pick("handle-input").clear();
        cy.pick("next-button").click();
        cy.pick("enter-name-screen").contains(
          "You must provide a display name"
        );
      });

      it("starts validation after at least 2 characters are input", () => {
        cy.pick("handle-input").type("@");
        cy.pick("validation-error-icon").should("not.be.visible");
        cy.pick("handle-input").type("@");
        cy.pick("validation-error-icon").should("be.visible");
      });

      it("prevents the user from submitting an invalid handle", () => {
        // No spaces.
        cy.pick("handle-input").type("no spaces");
        cy.pick("handle-input").should("have.value", "no-spaces");

        // No special characters.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("bad$");
        cy.pick("enter-name-screen").contains(
          "Your display name has unsupported characters in it. You can only " +
            "use basic letters, numbers, and the _ and - characters."
        );

        // Can't start with a dash.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("-não");
        cy.pick("enter-name-screen").contains(
          "Your display name should start with a letter or a number."
        );

        // Has to be at least two characters long.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").type("x");
        cy.pick("enter-name-screen").contains(
          "Your display name should be at least 2 characters long."
        );

        // Has to be no more than 32 characters long.
        cy.pick("handle-input").clear();
        cy.pick("handle-input").invoke("val", "x".repeat(33)).trigger("input");
        cy.pick("enter-name-screen").contains(
          "Your display name should not be longer than 32 characters."
        );
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
