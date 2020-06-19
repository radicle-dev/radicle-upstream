context("routing", () => {
  context("session persistancy", () => {
    context("first time app start with no stored session data", () => {
      it("opens on the identity creation wizard", () => {
        cy.nukeSessionState();
        cy.visit("./public/index.html");
        cy.pick("get-started-button").should("exist");
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

            cy.location().should(loc => {
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

            cy.pick("sidebar", "settings").click();

            cy.location().should(loc => {
              expect(loc.hash).to.eq("#/settings");
            });

            cy.reload();

            cy.location().should(loc => {
              expect(loc.hash).to.eq("#/settings");
            });
          });
        }
      );
    });
  });

  context("navigating between orgs", () => {
    it("goes to the respective org profile screen", () => {
      cy.nukeAllState();

      cy.createIdentity();
      cy.registerUser();
      cy.registerOrg("monadic");
      cy.registerOrg("github");

      cy.visit("./public/index.html");

      cy.pick("sidebar", "org-monadic").click();
      cy.pick("header").contains("monadic");

      cy.pick("sidebar", "org-github").click();
      cy.pick("header").contains("github");

      cy.pick("sidebar", "org-monadic").click();
      cy.pick("header").contains("monadic");
    });
  });
});
