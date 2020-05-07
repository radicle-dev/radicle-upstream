context("routing", () => {
  context("session persistancy", () => {
    context("first time app start with no stored session data", () => {
      it("opens on the identity creation wizard", () => {
        cy.nukeSessionState();
        cy.visit("./public/index.html");
        cy.get('[data-cy="get-started-button"]').should("exist");
      });
    });

    context("when there is an identity stored in the session", () => {
      context(
        "when there is no additional routing information stored in the browser location",
        () => {
          it("opens the app on the profile screen", () => {
            cy.nukeSessionState();
            cy.createIdentity();
            cy.visit("./public/index.html");

            cy.location().should((loc) => {
              expect(loc.hash).to.eq("#/profile");
            });
          });
        }
      );

      context(
        "when there is additional routing information stored in the browser location",
        () => {
          it("resumes the app from the browser location", () => {
            cy.nukeSessionState();
            cy.createIdentity();
            cy.visit("./public/index.html");

            cy.get('[data-cy="sidebar"] [data-cy="search"]').click();

            cy.location().should((loc) => {
              expect(loc.hash).to.eq("#/search");
            });

            cy.reload();

            cy.location().should((loc) => {
              expect(loc.hash).to.eq("#/search");
            });
          });
        }
      );
    });
  });
});
