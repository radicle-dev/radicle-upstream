context("navigation", () => {
  beforeEach(() => {
    cy.visit("./public/index.html#/projects");
  });

  context("when the app is opened for the first time", () => {
    it("shows the projects screen", () => {
      cy.contains("Projects").should("exist");
    });

    it("shows the four default mock projects", () => {
      cy.contains("monokel").should("exist");
      cy.contains("Monadic").should("exist");
      cy.contains("open source coin").should("exist");
      cy.contains("radicle").should("exist");
    });
  });

  context("first-level sidebar", () => {
    it("provides navigation to all main sections of the app", () => {
      cy.get('[data-cy="sidebar"] [data-cy="search"]').click();
      cy.get('[data-cy="page"]').should("contain", "Search");

      cy.get('[data-cy="sidebar"] [data-cy="feed"]').click();
      cy.get('[data-cy="page"]').should("contain", "Feed");

      cy.get('[data-cy="sidebar"] [data-cy="projects"]').click();
      cy.get('[data-cy="page"]').should("contain", "Projects");
      cy.get('[data-cy="page"]').should("contain", "New Project");

      cy.get('[data-cy="sidebar"] [data-cy="wallet"]').click();
      cy.get('[data-cy="page"]').should("contain", "Wallet");

      cy.get('[data-cy="sidebar"] [data-cy="profile"]').click();
      cy.get('[data-cy="page"]').should("contain", "Profile");

      cy.get('[data-cy="sidebar"] [data-cy="new-project"]').click();
      cy.get('[data-cy="page"] [data-cy="create-project"]').should("exist");
    });
  });

  context("project topbar", () => {
    it("opens the project overview by default", () => {
      cy.get('[data-cy="sidebar"] [data-cy="projects"]').click();
      cy.contains("monokel").click();

      cy.get("h2")
        .contains("Overview")
        .should("exist");
      cy.get("[data-cy=topbar]").within(() => {
        cy.contains("monokel").should("exist");
      });
    });

    it("provides navigation to all sub-sections of a project", () => {
      cy.get('[data-cy="sidebar"] [data-cy="projects"]').click();
      cy.contains("monokel").click();

      cy.get("h2")
        .contains("Overview")
        .should("exist");

      cy.get('[data-cy="project-topbar"]')
        .get('a[title="ProjectFeed"]')
        .click();
      cy.get("h2")
        .contains("Feed")
        .should("exist");
      cy.get("[data-cy=project-topbar]").within(() => {
        cy.contains("Feed").should("exist");
      });

      cy.get('[data-cy="project-topbar"]')
        .get('a[title="ProjectIssues"]')
        .click();
      cy.get("h2")
        .contains("Issues")
        .should("exist");
      cy.get("[data-cy=project-topbar]").within(() => {
        cy.contains("Issues").should("exist");
      });

      cy.get('[data-cy="project-topbar"]')
        .get('a[title="ProjectFund"]')
        .click();
      cy.get("h2")
        .contains("Fund")
        .should("exist");
      cy.get("[data-cy=project-topbar]").within(() => {
        cy.contains("Fund").should("exist");
      });

      cy.get('[data-cy="project-topbar"]')
        .get('a[title="ProjectSource"]')
        .click();
      cy.get("thead")
        .contains("Commit Message")
        .should("exist");
      cy.get("[data-cy=project-topbar]").within(() => {
        cy.contains("Source").should("exist");
      });

      cy.get('[data-cy="project-topbar"]')
        .get('a[title="ProjectRevisions"]')
        .click();
      cy.get("h2")
        .contains("Revisions")
        .should("exist");
      cy.get("[data-cy=project-topbar]").within(() => {
        cy.contains("Revisions").should("exist");
      });
    });
  });

  context("projects page", () => {
    context("clicking on the project name", () => {
      it("navigates to project overview", () => {
        cy.get('[data-cy="sidebar"] [data-cy="projects"]').click();
        cy.contains("Monadic").click();
        cy.get('[data-cy="project-topbar"]')
          .get('a[title="ProjectBranches"]')
          .click();

        cy.contains("Monadic").click();
        cy.contains("Overview").should("exist");
      });
    });

    context("when using the vertical scrollbar", () => {
      it("stays fixed at the top", () => {
        cy.get('[data-cy="sidebar"] [data-cy="projects"]').click();
        cy.contains("Monadic").click();
        cy.get('[data-cy="project-topbar"]')
          .get('a[title="ProjectSource"]')
          .click();

        cy.get("[data-cy=source-tree]").within(() => {
          cy.get("[data-cy=expand-src]").click();
          cy.contains("Eval.hs").click();
        });
        cy.get("[data-cy=page]").scrollTo("bottom");

        cy.get("[data-cy=project-topbar]").should("be.inViewport");
      });
    });
  });
});
