const user = () => "bojack";
const project1 = () => "rx";
const project2 = () => "upstream";
const org1 = () => "monadic";
const org2 = () => "github";

beforeEach(() => {
  cy.nukeAllState();

  cy.createIdentity(user());
  cy.registerUser(user());

  cy.registerOrg(org1());
  cy.registerOrg(org2());

  cy.createProjectWithFixture(project1());
  cy.createProjectWithFixture(project2());

  cy.visit("public/index.html");
});

context("project registration", () => {
  context("navigation", () => {
    it("can be accessed via profile project list project context menu", () => {
      cy.pick(`project-list-entry-${project2()}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2()}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown").contains(project2()).should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(user()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", project2());
    });

    it("can be accessed via project page context menu", () => {
      cy.pick(`project-list-entry-${project2()}`, "context-menu").click();
      cy.pick(`project-list-entry-${project2()}`).click();

      cy.pick("project-screen", "context-menu").click();
      cy.pick("dropdown-menu", "register-project").click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown").contains(project2()).should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(user()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", project2());
    });

    it("can be accessed via org onboarding page register button", () => {
      cy.pick("sidebar", `org-${org2()}`).click();
      cy.pick("add-project").click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown")
        .contains("Select project to register")
        .should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(org2()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", "");
    });

    it("can be accessed via org onboarding page context menu", () => {
      cy.pick("sidebar", `org-${org1()}`).click();
      cy.pick("context-menu").click();
      cy.pick("dropdown-menu", "add-project").click();

      cy.pick("project-registration-screen").should("exist");

      // The project is pre-selected to what we chose on the previous screen
      cy.pick("project-dropdown")
        .contains("Select project to register")
        .should("be.visible");
      // Registrar is pre-selected to our own identity
      cy.pick("registrar-dropdown").contains(org1()).should("be.visible");
      // The project name is pre-filled in the to-be-registered handle field
      cy.pick("name-input").should("have.value", "");
    });

    it("can be closed by pressing esc", () => {
      cy.pick(`project-list-entry-${project2()}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2()}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by clicking the 'x' icon", () => {
      cy.pick(`project-list-entry-${project2()}`, "context-menu").click();
      cy.pick(
        `project-list-entry-${project2()}`,
        "dropdown-menu",
        "register-project"
      ).click();

      cy.pick("project-registration-screen").should("exist");
      cy.pick("modal-close-button").click();
      cy.pick("profile-screen").should("exist");
    });
  });
});
