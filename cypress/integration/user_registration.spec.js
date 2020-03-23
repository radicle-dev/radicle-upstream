context("user registration", () => {
  context("modal navigation", () => {
    beforeEach(() => {
      cy.visit("./public/index.html#/projects");
      cy.get("body").type("{shift}t");
    });

    // TODO(merle): Replace opening via hotkey

    it("can be closed by pressing cancel", () => {
      cy.get('[data-cy="page"] [data-cy="register-user"]').should("exist");
      cy.get('[data-cy="register-user"] [data-cy="cancel-button"]').click();
      cy.contains("My Projects").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.get('[data-cy="page"] [data-cy="register-user"]').should("exist");
      cy.get("body").type("{esc}");
      cy.contains("My Projects").should("exist");
    });

    // navigation between pick handle (1) and submit tx (2) steps
    it("moves through the views by pressing navigation buttons", () => {
      // 1 -> 2
      cy.get('[data-cy="register-user"] [data-cy="handle"]').should("exist");
      cy.get('[data-cy="register-user"] [data-cy="next-button"]').click();
      cy.get('[data-cy="register-user"] [data-cy="tx-summary"]').should(
        "exist"
      );
      // 2 -> 1
      cy.get('[data-cy="register-user"] [data-cy="back-button"]').click();
      cy.get('[data-cy="register-user"] [data-cy="handle"]').should("exist");
      // 1 -> 2
      cy.get('[data-cy="register-user"] [data-cy="next-button"]').click();
      cy.get('[data-cy="register-user"] [data-cy="tx-summary"]').should(
        "exist"
      );
      // 2 -> close modal
      cy.get('[data-cy="register-user"] [data-cy="submit-button"]').click();
      cy.contains("My Projects").should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.visit("./public/index.html#/user-registration");

      cy.nukeAllState();
      cy.registerUser();
    });

    context("handle", () => {
      it("prevents the user from registering an invalid handle", () => {
        // shows a validation message when handle is not present
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"]').contains("Handle is required");

        // shows a validation message when handle contains invalid characters
        // spaces are not allowed
        cy.get('[data-cy="page"] [data-cy="handle"]').type("no spaces");
        cy.get('[data-cy="page"]').contains(
          "Handle should match [a-z0-9][a-z0-9_-]+"
        );

        // special characters are disallowed
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"] [data-cy="handle"]').type("$bad");
        cy.get('[data-cy="page"]').contains(
          "Handle should match [a-z0-9][a-z0-9_-]+"
        );

        // can't start with an underscore
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"] [data-cy="handle"]').type("_nein");
        cy.get('[data-cy="page"]').contains(
          "Handle should match [a-z0-9][a-z0-9_-]+"
        );

        // can't start with a dash
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"] [data-cy="handle"]').type("-nope");
        cy.get('[data-cy="page"]').contains(
          "Handle should match [a-z0-9][a-z0-9_-]+"
        );

        // has to be at least two characters long
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"] [data-cy="handle"]').type("x");
        cy.get('[data-cy="page"]').contains(
          "Handle should match [a-z0-9][a-z0-9_-]+"
        );
      });

      // TODO(merle): Add test setup, when mocks are replaced
      it("prevents the user from registering an unavailable handle", () => {
        // shows a validation message when handle is not available
        cy.get('[data-cy="page"] [data-cy="handle"]').clear();
        cy.get('[data-cy="page"] [data-cy="handle"]').type("nope");
        cy.get('[data-cy="page"]').contains("Handle already taken");
      });
    });
  });
});
