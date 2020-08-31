context("identity creation", () => {
  const validUser = {
    handle: "rafalca",
    passphrase: "curled unexposed daisy defacing",
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
    });

    context("when clicking the cancel button", () => {
      it("sends the user back to the welcome screen", () => {
        // On the name entry screen.
        cy.pick("get-started-button").click();
        cy.contains("what should we call you?").should("exist");
        cy.pick("cancel-button").click();
        cy.contains("A free and open-source way to host").should("exist");

        // On the passphrase entry screen.
        cy.pick("get-started-button").click();
        cy.pick("form", "handle").type(validUser.handle);
        cy.pick("next-button").click();
        cy.contains("you'll enter a passphrase").should("exist");
        cy.pick("cancel-button").click();
        cy.contains("A free and open-source way to host").should("exist");
      });
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.pick("get-started-button").click();
      cy.pick("form", "handle").type("_rafalca");
      cy.pick("next-button").click();
    });

    context("handle", () => {
      const validationError = "Handle should match ^[a-z0-9][a-z0-9_-]+$";

      it("prevents the user from submitting an invalid handle", () => {
        // handle is required
        cy.pick("form", "handle").clear();
        cy.pick("form").contains("You must provide a handle");

        // no spaces
        cy.pick("form", "handle").type("no spaces");
        cy.pick("form").contains(validationError);

        // no special characters
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("$bad");
        cy.pick("form").contains(validationError);

        // can't start with an underscore
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("_nein");
        cy.pick("form").contains(validationError);

        // can't start with a dash
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("-não");
        cy.pick("form").contains(validationError);

        // has to be at least two characters long
        cy.pick("form", "handle").clear();
        cy.pick("form", "handle").type("x");
        cy.pick("form").contains(validationError);
      });
    });
  });
});
