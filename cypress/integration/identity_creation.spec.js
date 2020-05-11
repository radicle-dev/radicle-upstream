context("identity creation", () => {
  beforeEach(() => {
    cy.nukeSessionState();
    cy.visit("./public/index.html");
  });

  context("modal", () => {
    it("can't be closed by pressing escape key", () => {
      cy.get('[data-cy="get-started-button"]').should("exist");
      cy.get("body").type("{esc}");
      cy.get('[data-cy="get-started-button"]').should("exist");
    });
  });

  context("navigation", () => {
    it("is possible to step through the identity creation flow", () => {
      // Intro screen
      cy.get('[data-cy="get-started-button"]').click();

      // Enter details screen
      cy.get('[data-cy="form"] [data-cy="handle"]').type("rafalca");
      cy.get('[data-cy="form"] [data-cy="display-name"]').type(
        "Rafalca Romney"
      );
      cy.get('[data-cy="create-id-button"]').click();

      // Confirmation screen
      // TODO(rudolfs): change the emoji once the backend returns the right one
      cy.get('[data-cy="identity-card"] img[alt="🐽"]').should("exist");
      cy.get('[data-cy="identity-card"]')
        .contains("Rafalca Romney")
        .should("exist");
      cy.get('[data-cy="identity-card"]')
        .contains("rafalca@123abcd.git")
        .should("exist");

      // Land on profile screen
      cy.get('[data-cy="go-to-profile-button"]').click();
      // TODO(rudolfs): change this to the actual handle that we
      // just created once the backend is wired up
      cy.get('[data-cy="profile-avatar"]').contains("cloudhead");
    });

    it("is possible to directly register your identity after creating it", () => {
      cy.get('[data-cy="get-started-button"]').click();

      cy.get('[data-cy="form"] [data-cy="handle"]').type("rafalca");
      cy.get('[data-cy="create-id-button"]').click();
      cy.get('[data-cy="register-identity-link"]').click();

      cy.contains("Register your handle").should("exist");

      cy.get('[data-cy="cancel-button"]').click();
      // TODO(rudolfs): change this to the actual handle that we
      // just created once the backend is wired up
      cy.get('[data-cy="profile-avatar"]').contains("cloudhead");
    });

    context(
      "when clicking cancel, close or hitting esc before the identity is created",
      () => {
        it("sends the user back to the intro screen", () => {
          cy.get('[data-cy="get-started-button"]').click();
          cy.get('[data-cy="cancel-button"]').click();

          // We should land back on the intro screen
          cy.get('[data-cy="get-started-button"]').click();

          // Now try to close the modal via the "x" button
          cy.get('[data-cy="modal-close-button"]').click();

          // We should land back on the intro screen
          cy.get('[data-cy="get-started-button"]').click();

          // Now try the escape key
          cy.get("body").type("{esc}");

          // We should land back on the intro screen
          cy.get('[data-cy="get-started-button"]').should("exist");
        });
      }
    );

    context(
      "when clicking the modal close button on the success screen",
      () => {
        it("lands the user on the profile screen", () => {
          cy.get('[data-cy="get-started-button"]').click();

          cy.get('[data-cy="form"] [data-cy="handle"]').type("rafalca");
          cy.get('[data-cy="create-id-button"]').click();

          cy.get('[data-cy="identity-card"]')
            .contains("rafalca@123abcd.git")
            .should("exist");

          // Land on profile screen
          cy.get('[data-cy="modal-close-button"]').click();
          // TODO(rudolfs): change this to the actual handle that we
          // just created once the backend is wired up
          cy.get('[data-cy="profile-avatar"]').contains("cloudhead");
        });
      }
    );

    context("when pressing escape on the success screen", () => {
      it("lands the user on the profile screen", () => {
        cy.get('[data-cy="get-started-button"]').click();

        cy.get('[data-cy="form"] [data-cy="handle"]').type("rafalca");
        cy.get('[data-cy="create-id-button"]').click();

        cy.get('[data-cy="identity-card"]')
          .contains("rafalca@123abcd.git")
          .should("exist");

        // Now try the escape key
        cy.get("body").type("{esc}");

        // Land on profile screen
        // TODO(rudolfs): change this to the actual handle that we
        // just created once the backend is wired up
        cy.get('[data-cy="profile-avatar"]').contains("cloudhead");
      });
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.get('[data-cy="get-started-button"]').click();
      cy.get('[data-cy="form"] [data-cy="handle"]').type("_rafalca");
      cy.get('[data-cy="form"] [data-cy="display-name"]').type(
        "Rafalca Romney"
      );
      cy.get('[data-cy="form"] [data-cy="avatar-url"]').type(
        "https://www.motherjones.com/wp-content/uploads/images/horsehop.jpg"
      );
      cy.get('[data-cy="create-id-button"]').click();
    });

    context("handle", () => {
      it("prevents the user from submitting an invalid handle", () => {
        // handle is required
        cy.get('[data-cy="form"] [data-cy="handle"]').clear();
        cy.get('[data-cy="form"]').contains("You must provide a handle");

        // no spaces
        cy.get('[data-cy="form"] [data-cy="handle"]').type("no spaces");
        cy.get('[data-cy="form"]').contains(
          "Handle should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // no special characters
        cy.get('[data-cy="form"] [data-cy="handle"]').clear();
        cy.get('[data-cy="form"] [data-cy="handle"]').type("$bad");
        cy.get('[data-cy="form"]').contains(
          "Handle should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // can't start with an underscore
        cy.get('[data-cy="form"] [data-cy="handle"]').clear();
        cy.get('[data-cy="form"] [data-cy="handle"]').type("_nein");
        cy.get('[data-cy="form"]').contains(
          "Handle should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // can't start with a dash
        cy.get('[data-cy="form"] [data-cy="handle"]').clear();
        cy.get('[data-cy="form"] [data-cy="handle"]').type("-não");
        cy.get('[data-cy="form"]').contains(
          "Handle should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // has to be at least two characters long
        cy.get('[data-cy="form"] [data-cy="handle"]').clear();
        cy.get('[data-cy="form"] [data-cy="handle"]').type("x");
        cy.get('[data-cy="form"]').contains(
          "Handle should match ^[a-z0-9][a-z0-9_-]+$"
        );
      });
    });

    context("display name", () => {
      it("prevents the user from submitting an invalid display name", () => {
        cy.get('[data-cy="form"] [data-cy="display-name"]').clear();
        cy.get('[data-cy="form"] [data-cy="display-name"]').type("_not good");
        cy.get('[data-cy="form"]').contains(
          "Display name should match ^[a-z0-9 ]+$"
        );
      });
    });

    context("avatar URL", () => {
      it("prevents the user from submitting an invalid avatar URL", () => {
        cy.get('[data-cy="form"] [data-cy="avatar-url"]').clear();
        cy.get('[data-cy="form"] [data-cy="avatar-url"]').type("randomwords");
        cy.get('[data-cy="form"]').contains("Not a valid image URL");
      });
    });
  });
});
