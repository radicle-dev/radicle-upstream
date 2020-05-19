import { DIALOG_SHOWOPENDIALOG } from "../../native/ipc.js";

const withEmptyRepositoryStub = (callback) => {
  cy.exec("pwd").then((result) => {
    const pwd = result.stdout;
    const emptyDirectoryPath = `${pwd}/fixtures/empty-repo`;

    cy.exec(`rm -rf ${emptyDirectoryPath}`);
    cy.exec(`mkdir ${emptyDirectoryPath}`);

    // stub native call and return the directory path to the UI
    cy.window().then((appWindow) => {
      appWindow.electron = {
        ipcRenderer: {
          invoke: (msg) => {
            if (msg === DIALOG_SHOWOPENDIALOG) {
              return emptyDirectoryPath;
            }
          },
        },
      };
    });

    callback();

    // clean up the fixture
    cy.exec(`rm -rf ${emptyDirectoryPath}`);
  });
};

const withPlatinumStub = (callback) => {
  cy.exec("pwd").then((result) => {
    const pwd = result.stdout;
    const platinumPath = `${pwd}/fixtures/git-platinum-copy`;

    cy.exec(`rm -rf ${platinumPath}`);
    cy.exec(
      `git clone ${pwd}/.git/modules/fixtures/git-platinum ${platinumPath}`
    );

    // stub native call and return the directory path to the UI
    cy.window().then((appWindow) => {
      appWindow.electron = {
        ipcRenderer: {
          invoke: (msg) => {
            if (msg === DIALOG_SHOWOPENDIALOG) {
              return platinumPath;
            }
          },
        },
      };
    });

    callback();

    cy.exec(`rm -rf ${platinumPath}`);
  });
};

beforeEach(() => {
  cy.nukeAllState();
  cy.createIdentity();
  cy.createProjectWithFixture();
  cy.visit("./public/index.html");
});

context("project creation", () => {
  context("project creation modal", () => {
    // TODO(rudolfs): test empty project listing has wording and button

    it("can be opened via the profile context menu and closed by pressing cancel", () => {
      cy.get('[data-cy="profile-context-menu"]').click();
      cy.get('[data-cy="dropdown-menu"] [data-cy="new-project"]').click();
      cy.get('[data-cy="page"] [data-cy="create-project"]').should("exist");
      cy.get('[data-cy="create-project"] [data-cy="cancel-button"]').click();
      cy.get('[data-cy="profile-screen"]').should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.get('[data-cy="profile-context-menu"]').click();
      cy.get('[data-cy="dropdown-menu"] [data-cy="new-project"]').click();
      cy.get('[data-cy="page"] [data-cy="create-project"]').should("exist");
      cy.get("body").type("{esc}");
      cy.get('[data-cy="profile-screen"]').should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.get('[data-cy="profile-context-menu"]').click();
      cy.get('[data-cy="dropdown-menu"] [data-cy="new-project"]').click();

      // Set up minimal form input to show validations
      cy.get('[data-cy="page"] [data-cy="name"]').type("this-name-is-valid");
      cy.get('[data-cy="page"] [data-cy="new-project"]').click();
      cy.get('[data-cy="page"] [data-cy="create-project-button"]').click();
    });

    afterEach(() => {
      cy.get("body").type("{esc}", { force: true });
    });

    context("name", () => {
      it("prevents the user from creating a project with an invalid name", () => {
        // shows a validation message when name is not present
        cy.get('[data-cy="page"] [data-cy="name"]').clear();
        cy.get('[data-cy="page"]').contains("Project name is required");

        // shows a validation message when name contains invalid characters
        // spaces are not allowed
        cy.get('[data-cy="page"] [data-cy="name"]').type("no spaces");
        cy.get('[data-cy="page"]').contains(
          "Project name should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // special characters are disallowed
        cy.get('[data-cy="page"] [data-cy="name"]').clear();
        cy.get('[data-cy="page"] [data-cy="name"]').type("$bad");
        cy.get('[data-cy="page"]').contains(
          "Project name should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // can't start with an underscore
        cy.get('[data-cy="page"] [data-cy="name"]').clear();
        cy.get('[data-cy="page"] [data-cy="name"]').type("_nein");
        cy.get('[data-cy="page"]').contains(
          "Project name should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // can't start with a dash
        cy.get('[data-cy="page"] [data-cy="name"]').clear();
        cy.get('[data-cy="page"] [data-cy="name"]').type("-nope");
        cy.get('[data-cy="page"]').contains(
          "Project name should match ^[a-z0-9][a-z0-9_-]+$"
        );

        // has to be at least two characters long
        cy.get('[data-cy="page"] [data-cy="name"]').clear();
        cy.get('[data-cy="page"] [data-cy="name"]').type("x");
        cy.get('[data-cy="page"]').contains(
          "Project name should match ^[a-z0-9][a-z0-9_-]+$"
        );
      });
    });

    context("new repository", () => {
      it("prevents the user from picking an invalid directory", () => {
        // shows a validation message when new project path is empty
        cy.get('[data-cy="page"] [data-cy="new-project"]')
          .contains("Pick a directory for the new project")
          .should("exist");

        withPlatinumStub(() => {
          cy.pick("new-project", "choose-path-button").click();
          cy.pick("create-project-button").click();

          cy.get('[data-cy="page"] [data-cy="new-project"]')
            .contains("The directory should be empty")
            .should("exist");
        });
      });
    });

    context("existing repository", () => {
      it("prevents the user from picking an invalid directory", () => {
        cy.get('[data-cy="page"] [data-cy="existing-project"]').click();

        // shows a validation message when existing project path is empty
        cy.get('[data-cy="page"] [data-cy="existing-project"]')
          .contains("Pick a directory with an existing repository")
          .should("exist");

        withEmptyRepositoryStub(() => {
          cy.pick("existing-project", "choose-path-button").click();

          // shows a validation message when an empty directory is chosen
          cy.get('[data-cy="page"] [data-cy="existing-project"]')
            .contains("The directory should contain a git repository")
            .should("exist");
        });

        withPlatinumStub(() => {
          cy.pick("existing-project", "choose-path-button").click();
          cy.pick("create-project-button").click();
          cy.pick("profile").click();

          cy.pick("profile-context-menu").click();
          cy.pick("dropdown-menu", "new-project").click();
          cy.get('[data-cy="page"] [data-cy="name"]').type("another-project");
          cy.get('[data-cy="page"] [data-cy="existing-project"]').click();
          cy.pick("existing-project", "choose-path-button").click();

          cy.get('[data-cy="page"] [data-cy="existing-project"]')
            .contains("This repository is already managed by Radicle")
            .should("exist");
        });
      });
    });

    context("form", () => {
      it("prevents the user from submitting invalid data", () => {
        // shows a validation message when new project path is empty
        cy.get('[data-cy="page"] [data-cy="new-project"]')
          .contains("Pick a directory for the new project")
          .should("exist");

        cy.get('[data-cy="page"] [data-cy="existing-project"]').click();
        // shows a validation message when existing project path is empty
        cy.get('[data-cy="page"] [data-cy="existing-project"]')
          .contains("Pick a directory with an existing repository")
          .should("exist");
      });
    });
  });

  context("happy paths", () => {
    it("creates a new project from an empty directory", () => {
      withEmptyRepositoryStub(() => {
        cy.pick("profile-context-menu").click();
        cy.pick("dropdown-menu", "new-project").click();

        cy.pick("name").type("new-fancy-project");
        cy.pick("description").type("My new fancy project");

        cy.pick("new-project").click();
        cy.pick("new-project", "choose-path-button").click();
        cy.pick("create-project-button").click();

        cy.pick("project-screen", "topbar", "project-avatar").contains(
          "new-fancy-project"
        );

        cy.pick("notification").contains(
          "Project new-fancy-project successfully created"
        );

        cy.pick("profile").click();
        cy.pick("profile-screen", "project-list").contains("new-fancy-project");
        cy.pick("profile-screen", "project-list").contains(
          "My new fancy project"
        );
      });
    });

    it("creates a new project from an existing repository", () => {
      withPlatinumStub(() => {
        cy.pick("profile-context-menu").click();
        cy.pick("dropdown-menu", "new-project").click();

        cy.pick("name").type("git-platinum-copy");
        cy.pick("description").type("Best project");

        cy.pick("existing-project").click();
        cy.pick("existing-project", "choose-path-button").click();
        cy.pick("create-project-button").click();
        cy.pick("project-screen", "topbar", "project-avatar").contains(
          "git-platinum-copy"
        );

        cy.pick("project-screen").contains(
          "This repository is a data source for the Upstream front-end tests"
        );

        cy.pick("notification").contains(
          "Project git-platinum-copy successfully created"
        );

        cy.pick("profile").click();
        cy.pick("profile-screen", "project-list").contains("git-platinum-copy");
        cy.pick("profile-screen", "project-list").contains("Best project");
      });
    });
  });
});
