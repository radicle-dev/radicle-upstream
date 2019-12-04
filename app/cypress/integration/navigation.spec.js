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
      cy.get('[data-cy="sidebar"]')
        .get('a[title="Search"]')
        .click()
        .blur();
      cy.contains("Search").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-search-active"
      });

      cy.get('[data-cy="sidebar"]')
        .get('a[title="Feed"]')
        .click()
        .blur();
      cy.contains("Feed").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-feed-active"
      });

      cy.get('[data-cy="sidebar"]')
        .get('a[title="Projects"]')
        .click()
        .blur();
      cy.contains("Projects").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-projects-active"
      });

      cy.get('[data-cy="sidebar"]')
        .get('a[title="Create new project"]')
        .click()
        .blur();
      cy.contains("Design System").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-design-system-active"
      });

      cy.get('[data-cy="sidebar"]')
        .get('a[title="Profile"]')
        .click()
        .blur();
      cy.contains("Profile").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-profile-active"
      });

      cy.get('[data-cy="sidebar"]')
        .get('a[title="Fund"]')
        .click()
        .blur();
      cy.contains("Wallet").should("exist");

      // check whether the active icon is highlighted
      cy.get('[data-cy="sidebar"]').toMatchImageSnapshot({
        name: "sidebar-fund-active"
      });
    });
  });

  context("second-level project sidebar", () => {
    it("opens the project overview by default", () => {
      cy.get('[data-cy="sidebar"]')
        .get('a[title="Projects"]')
        .click();
      cy.contains("monokel").click();

      cy.get("h2")
        .contains("Overview")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("My Projects").should("exist");
        cy.contains("monokel").should("exist");
        cy.contains("Overview").should("exist");
      });
    });

    it("provides navigation to all sub-sections of a project", () => {
      cy.get('[data-cy="sidebar"]')
        .get('a[title="Projects"]')
        .click();
      cy.contains("monokel").click();

      cy.get("h2")
        .contains("Overview")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Overview").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-overview-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectFeed"]')
        .click()
        .blur();
      cy.get("h2")
        .contains("Feed")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Feed").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-feed-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectMembers"]')
        .click()
        .blur();
      cy.get("h2")
        .contains("Members")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Members").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-members-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectFund"]')
        .click()
        .blur();
      cy.get("h2")
        .contains("Fund")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Fund").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-fund-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectSource"]')
        .click()
        .blur();
      cy.get("thead")
        .contains("Commit Message")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Source").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-source-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectCommits"]')
        .click()
        .blur();
      cy.get("h2")
        .contains("Commits")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Commits").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-commits-active"
      });

      cy.get('[data-cy="project-sidebar"]')
        .get('a[title="ProjectBranches"]')
        .click()
        .blur();
      cy.get("h2")
        .contains("Branches")
        .should("exist");
      cy.get("[data-cy=breadcrumbs]").within(() => {
        cy.contains("Branches").should("exist");
      });

      // check whether the active icon is highlighted
      cy.get('[data-cy="project-sidebar"]').toMatchImageSnapshot({
        name: "project-sidebar-branches-active"
      });
    });
  });

  context("breadcrumb navigation", () => {
    context("clicking on 'My Projects'", () => {
      it("navigates back to project listing", () => {
        cy.get('[data-cy="sidebar"]')
          .get('a[title="Projects"]')
          .click();
        cy.contains("Monadic").click();
        cy.contains("My Projects").click();
        cy.contains("Projects").should("exist");
      });
    });

    context("clicking on the project name", () => {
      it("navigates to project overview", () => {
        cy.get('[data-cy="sidebar"]')
          .get('a[title="Projects"]')
          .click();
        cy.contains("Monadic").click();
        cy.get('[data-cy="project-sidebar"]')
          .get('a[title="ProjectBranches"]')
          .click();

        cy.contains("Monadic").click();
        cy.contains("Overview").should("exist");
      });
    });

    context("when using the vertical scrollbar", () => {
      it("stays fixed at the top", () => {
        cy.get('[data-cy="sidebar"]')
          .get('a[title="Projects"]')
          .click();
        cy.contains("Monadic").click();
        cy.get('[data-cy="project-sidebar"]')
          .get('a[title="ProjectSource"]')
          .click();

        cy.get("[data-cy=source-tree]").within(() => {
          cy.get("[data-cy=expand-src]").click();
          cy.contains("Eval.hs").click();
        });
        cy.window().scrollTo("bottom");

        cy.get("[data-cy=breadcrumbs]").should("be.inViewport");
      });
    });
  });
});
