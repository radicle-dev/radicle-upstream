context("identity creation", () => {
  beforeEach(() => {
    openModal();
  });

  const openModal = () => {
    cy.visit("./public/index.html#/projects");
    cy.get("body").type("{shift}i");
  };

  context("modal", () => {
    it("can be closed by pressing escape key", () => {
      cy.get("body").type("{esc}");
      cy.contains("My Projects").should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      openModal();
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
