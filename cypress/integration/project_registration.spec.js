const user = () => "bojack";
const project1 = () => {
  return {
    name: "rx",
    description: "best pixel editor",
    defaultBranch: "master",
  };
};
const project2 = () => {
  return {
    name: "upstream",
    description: "code collab",
    defaultBranch: "master",
  };
};
const org1 = () => "monadic";
const org2 = () => "github";

beforeEach(() => {
  cy.nukeAllState();

  cy.createIdentity(user());
  cy.registerUser(user());

  cy.registerOrg(org1());
  cy.registerOrg(org2());

  cy.createProjectWithFixture(
    project1().name,
    project1().description,
    project1().defaultBranch
  );
  cy.createProjectWithFixture(
    project2().name,
    project2().description,
    project2().defaultBranch
  );

  cy.visit("public/index.html");
});

context("project registration", () => {
  context("navigation", () => {
    it("can be accessed via profile project list project context menu", () => {
      cy.pick(`project-list-entry-${project2().name}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2().name}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown")
        .contains(project2().name)
        .should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(user()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", project2().name);
    });

    it("can be accessed via project page context menu", () => {
      cy.pick(`project-list-entry-${project2().name}`, "context-menu").click();
      cy.pick(`project-list-entry-${project2().name}`).click();

      cy.pick("project-screen", "context-menu").click();
      cy.pick("dropdown-menu", "register-project").click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown")
        .contains(project2().name)
        .should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(user()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", project2().name);
    });

    it("can be accessed via org onboarding page register button", () => {
      cy.pick("sidebar", `org-${org2()}`).click();
      cy.pick("add-project").click();

      cy.pick("project-registration-screen").should("exist");

      cy.pick("project-dropdown")
        .contains("Select project to register")
        .should("be.visible");
      // Registrar is pre-selected to the org we chose
      cy.pick("registrar-dropdown").contains(org2()).should("be.visible");
      cy.pick("name-input").should("have.value", "");
    });

    it("can be accessed via org onboarding page context menu", () => {
      cy.pick("sidebar", `org-${org1()}`).click();
      cy.pick("context-menu").click();
      cy.pick("dropdown-menu", "add-project").click();

      cy.pick("project-registration-screen").should("exist");

      cy.pick("project-dropdown")
        .contains("Select project to register")
        .should("be.visible");
      // Registrar is pre-selected to the org we chose
      cy.pick("registrar-dropdown").contains(org1()).should("be.visible");
      cy.pick("name-input").should("have.value", "");
    });

    it("can be closed by pressing esc", () => {
      cy.pick(`project-list-entry-${project2().name}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2().name}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by clicking the 'x' icon", () => {
      cy.pick(`project-list-entry-${project2().name}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2().name}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");
      cy.pick("modal-close-button").click();
      cy.pick("profile-screen").should("exist");
    });
  });

  context("summary screen", () => {
    context("when registering under a user", () => {
      it("shows the selected subject and payer information", () => {
        cy.pick(
          `project-list-entry-${project1().name}`,
          "context-menu"
        ).click();
        cy.pick(`project-list-entry-${project1().name}`).click();

        cy.pick("project-screen", "context-menu").click();
        cy.pick("dropdown-menu", "register-project").click();

        cy.pick("project-registration-screen").should("exist");
        cy.pick("submit-button").click();

        cy.get('[data-cy="subject-avatar"] img[alt="🐅"]').should("exist");
        cy.pick("subject-avatar").contains(`${user()} / ${project1().name}`);

        cy.get('[data-cy="payer-avatar"] img[alt="🐅"]').should("exist");
        cy.pick("payer-avatar").contains(user());
      });
    });

    context("when registering under an org", () => {
      it("shows the selected subject and payer information", () => {
        cy.pick("sidebar", `org-${org1()}`).click();
        cy.pick("context-menu").click();
        cy.pick("dropdown-menu", "add-project").click();

        cy.pick("project-registration-screen").should("exist");

        cy.pick("project-dropdown").click().contains(project1().name).click();
        // Registrar is pre-selected to the org we chose
        cy.pick("registrar-dropdown").contains(org1()).should("be.visible");
        cy.pick("name-input").should("have.value", project1().name);

        cy.pick("submit-button").click();

        cy.get('[data-cy="subject-avatar"] img[alt="🥂"]').should("exist");
        cy.pick("subject-avatar").contains(`${org1()} / ${project1().name}`);

        cy.get('[data-cy="payer-avatar"] img[alt="🥂"]').should("exist");
        cy.pick("payer-avatar").contains(org1());
      });
    });
  });

  context("happy path", () => {
    context("when registering under an org", () => {
      it("registers the project and shows it in the org project list", () => {
        cy.pick("sidebar", `org-${org1()}`).click();
        cy.pick("context-menu").click();
        cy.pick("dropdown-menu", "add-project").click();

        cy.pick("project-registration-screen").should("exist");

        cy.pick("project-dropdown").click().contains(project1().name).click();

        // go to summary screen
        cy.pick("submit-button").click();
        // submit the transaction
        cy.pick("submit-button").click();

        cy.pick("project-list", `project-${project1().name}`).contains(
          project1().name
        );
        cy.pick("project-list", `project-${project1().name}`).contains(
          project1().description
        );
        cy.pick(
          "project-list",
          `project-${project1().name}`,
          "registered"
        ).should("exist");
      });
    });
  });
});
