import { DIALOG_SHOWOPENDIALOG } from "../../native/ipc.js";

const withEmptyRepositoryStub = callback => {
  cy.exec("pwd").then(result => {
    const pwd = result.stdout;
    const emptyDirectoryPath = `${pwd}/cypress/workspace/empty-repo`;

    cy.exec(`rm -rf ${emptyDirectoryPath}`);
    cy.exec(`mkdir -p ${emptyDirectoryPath}`);

    // stub native call and return the directory path to the UI
    cy.window().then(appWindow => {
      appWindow.electron = {
        ipcRenderer: {
          invoke: msg => {
            if (msg === DIALOG_SHOWOPENDIALOG) {
              return emptyDirectoryPath;
            }
          },
        },
        isDev: true,
        isExperimental: true,
      };
    });

    callback();

    // clean up the fixture
    cy.exec(`rm -rf ${emptyDirectoryPath}`);
  });
};

const withPlatinumStub = callback => {
  cy.exec("pwd").then(result => {
    const pwd = result.stdout;
    const platinumPath = `${pwd}/cypress/workspace/git-platinum-copy`;

    cy.exec(`rm -rf ${platinumPath}`);
    cy.exec(
      `git clone ${pwd}/.git/modules/fixtures/git-platinum ${platinumPath}`
    );

    // stub native call and return the directory path to the UI
    cy.window().then(appWindow => {
      appWindow.electron = {
        ipcRenderer: {
          invoke: msg => {
            if (msg === DIALOG_SHOWOPENDIALOG) {
              return platinumPath;
            }
          },
        },
        isDev: true,
        isExperimental: true,
      };
    });

    callback();

    cy.exec(`rm -rf ${platinumPath}`);
  });
};

beforeEach(() => {
  cy.nukeAllState();
  cy.onboardUser();
  cy.visit("./public/index.html");
});

context("project creation", () => {
  context("project creation modal", () => {
    // TODO(rudolfs): test empty project listing has wording and button

    it("can be opened via the profile header action button and closed by pressing cancel", () => {
      cy.pick("new-project-button").click();
      cy.pick("page", "create-project").should("exist");
      cy.pick("create-project", "cancel-button").click();
      cy.pick("profile-screen").should("exist");
    });

    it("can be closed by pressing escape key", () => {
      cy.pick("new-project-button").click();
      cy.pick("page", "create-project").should("exist");
      cy.get("body").type("{esc}");
      cy.pick("profile-screen").should("exist");
    });
  });

  context("validations", () => {
    beforeEach(() => {
      cy.pick("new-project-button").click();

      // Set up minimal form input to show validations
      cy.pick("page", "name").type("this-name-is-valid");
      cy.pick("page", "new-project").click();
    });

    afterEach(() => {
      cy.get("body").type("{esc}", { force: true });
    });

    context.only("name", () => {
      it("prevents the user from creating a project with an invalid name", () => {
        // the submit button is disabled when name is not present
        cy.pick("page", "name").clear();
        cy.pick("create-project-button").should("be.disabled");

        // spaces should be changed into dashes
        cy.pick("page", "name").type("no spaces");
        cy.pick("page", "name").should("have.value", "no-spaces");

        // shows a validation message when name contains invalid characters

        // special characters are disallowed
        cy.pick("page", "name").clear();
        cy.pick("page", "name").type("$bad");
        cy.pick("page").contains(
          "Project name should match ^[a-z0-9][a-z0-9._-]+$"
        );

        // can't start with an underscore
        cy.pick("page", "name").clear();
        cy.pick("page", "name").type("_nein");
        cy.pick("page").contains(
          "Project name should match ^[a-z0-9][a-z0-9._-]+$"
        );

        // can't start with a dash
        cy.pick("page", "name").clear();
        cy.pick("page", "name").type("-nope");
        cy.pick("page").contains(
          "Project name should match ^[a-z0-9][a-z0-9._-]+$"
        );

        // has to be at least two characters long
        cy.pick("page", "name").clear();
        cy.pick("page", "name").type("x");
        cy.pick("page").contains(
          "Project name should match ^[a-z0-9][a-z0-9._-]+$"
        );
      });
    });

    context("new repository", () => {
      it("prevents the user from picking an invalid directory", () => {
        // shows a validation message when new project path is empty
        cy.pick("page", "new-project")
          .contains("Pick a directory for the new project")
          .should("exist");

        withPlatinumStub(() => {
          cy.pick("new-project", "choose-path-button").click();

          cy.pick("page", "new-project")
            .contains(
              "Please choose a directory that's not already a git repository."
            )
            .should("exist");
        });
      });
    });

    context("form", () => {
      it("clears name input when switching from new to existing project", () => {
        cy.pick("name").clear();
        cy.pick("name").type("this-will-be-a-new-project");
        cy.pick("new-project").click();
        cy.pick("name").should("have.value", "this-will-be-a-new-project");
        cy.pick("existing-project").click();
        cy.pick("name").should("have.value", "");
      });

      it("prevents the user from submitting invalid data", () => {
        // shows a validation message when new project path is empty
        cy.pick("page", "new-project")
          .contains("Pick a directory for the new project")
          .should("exist");
      });
    });
  });

  context("happy paths", () => {
    it("creates a new project from an empty directory", () => {
      withEmptyRepositoryStub(() => {
        cy.pick("new-project-button").click();

        cy.pick("name").type("new-fancy-project.xyz");
        cy.pick("description").type("My new fancy project");

        cy.pick("new-project").click();
        cy.pick("new-project", "choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // this prevents this spec from failing on CI.
        cy.wait(500);

        cy.pick("create-project-button").click();

        cy.pick("project-screen", "header").contains("new-fancy-project");

        cy.pick("notification").contains(
          "Project new-fancy-project.xyz successfully created"
        );

        cy.pick("profile").click();
        cy.pick("profile-screen", "project-list").contains(
          "new-fancy-project.xyz"
        );
        cy.pick("profile-screen", "project-list").contains(
          "My new fancy project"
        );
      });
    });

    it("creates a new project from an existing repository", () => {
      withPlatinumStub(() => {
        cy.pick("new-project-button").click();

        cy.pick("name").should("not.be.disabled");

        cy.pick("existing-project").click();
        cy.pick("name").should("be.disabled");

        cy.pick("existing-project", "choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // this prevents this spec from failing on CI.
        cy.wait(500);

        cy.pick("name").should("have.value", "git-platinum-copy");
        cy.pick("description").type("Best project");

        cy.pick("create-project-button").click();
        cy.pick("project-screen", "header").contains("git-platinum-copy");

        cy.pick("project-screen", "header").contains("Best project");

        cy.pick("notification").contains(
          "Project git-platinum-copy successfully created"
        );

        cy.pick("profile").click();
        cy.pick("profile-screen", "project-list").contains("git-platinum-copy");
        cy.pick("profile-screen", "project-list").contains("Best project");

        cy.pick("notification")
          .contains("Project git-platinum-copy successfully created")
          .should("exist");
        cy.pick("notification").contains("Close").click();

        // Make sure we can't add the same project twice.
        cy.pick("new-project-button").click();

        cy.pick("existing-project").click();

        cy.pick("existing-project", "choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // this prevents this spec from failing on CI.
        cy.wait(500);

        cy.pick("name").should("have.value", "git-platinum-copy");
        cy.pick("description").type("Best project");

        cy.pick("create-project-button").click();

        cy.pick("notification")
          .contains(
            /Could not create project: the identity 'rad:git:[\w]{3}…[\w]{3}' already exists/
          )
          .should("exist");
        cy.pick("notification").contains("Close").click();
      });
    });
  });
});
