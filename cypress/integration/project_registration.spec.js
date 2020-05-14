const user = () => "bojack";
const project1 = () => "rx";
const project2 = () => "upstream";

beforeEach(() => {
  cy.nukeAllState();

  cy.createIdentity(user());
  cy.registerUser(user());

  cy.registerOrg("monadic");
  cy.registerOrg("github");

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
