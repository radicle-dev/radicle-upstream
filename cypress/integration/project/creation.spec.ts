import * as ipcStub from "../../support/ipc-stub";
import * as commands from "../../support/commands";
import * as config from "../../../ui/src/config";

context("project creation", () => {
  const withEmptyDirectoryStub = (callback: () => void) => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      const emptyDirectoryPath = `${pwd}/cypress/workspace/empty-directory`;

      cy.exec(`rm -rf ${emptyDirectoryPath}`);
      cy.exec(`mkdir -p ${emptyDirectoryPath}`);

      ipcStub.getStubs().then(stubs => {
        stubs.selectDirectory.returns(emptyDirectoryPath);
      });

      callback();

      // clean up the fixture
      cy.exec(`rm -rf ${emptyDirectoryPath}`);
    });
  };

  const withNoCommitsRepositoryStub = (callback: () => void) => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      const noCommitsRepoPath = `${pwd}/cypress/workspace/no-commits-repo`;

      cy.exec(`rm -rf ${noCommitsRepoPath}`);
      cy.exec(`mkdir -p ${noCommitsRepoPath}`);
      cy.exec(`git init ${noCommitsRepoPath}`);

      // stub native call and return the directory path to the UI
      ipcStub.getStubs().then(stubs => {
        stubs.selectDirectory.returns(noCommitsRepoPath);
      });

      callback();

      // clean up the fixture
      cy.exec(`rm -rf ${noCommitsRepoPath}`);
    });
  };

  // Create the given `branches` in the repo with name `repoName`,
  // expected to be found within cypress/workspace.
  const createBranches = (repoName: string, branches: string[]) => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      const repoPath = `${pwd}/cypress/workspace/${repoName}`;
      branches.forEach(branch =>
        cy.exec(`cd ${repoPath}; git checkout -b ${branch};`)
      );
    });
  };

  const withPlatinumStub = (callback: (repoName: string) => void) => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      const repoName = "git-platinum-copy";
      const platinumPath = `${pwd}/cypress/workspace/${repoName}`;

      cy.exec(`rm -rf ${platinumPath}`);
      cy.exec(
        `git clone ${pwd}/.git/modules/fixtures/git-platinum ${platinumPath}`
      );

      // stub native call and return the directory path to the UI
      ipcStub.getStubs().then(stubs => {
        stubs.selectDirectory.returns(platinumPath);
      });

      callback(repoName);

      cy.exec(`rm -rf ${platinumPath}`);
    });
  };

  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("./public/index.html");
  });

  context("project creation", () => {
    context("project creation modal", () => {
      // TODO(rudolfs): test empty project listing has wording and button

      it("can be opened via the profile header action button and closed by pressing cancel", () => {
        commands.pick("new-project-button").click();
        commands.pick("page", "create-project").should("exist");
        commands.pick("create-project", "cancel-button").click();
        commands.pick("profile-screen").should("exist");
      });

      it("can be closed by pressing escape key", () => {
        commands.pick("new-project-button").click();
        commands.pick("page", "create-project").should("exist");
        cy.get("body").type("{esc}");
        commands.pick("profile-screen").should("exist");
      });
    });

    context("validations", () => {
      beforeEach(() => {
        commands.pick("new-project-button").click();

        // Set up minimal form input to show validations
        commands.pick("page", "name").type("this-name-is-valid");
        commands.pick("page", "new-project").click();
      });

      afterEach(() => {
        cy.get("body").type("{esc}", { force: true });
      });

      context("name", () => {
        it("prevents the user from creating a project with an invalid name", () => {
          // the submit button is disabled when name is not present
          commands.pick("page", "name").clear();
          commands.pick("create-project-button").should("be.disabled");

          // spaces should be changed into dashes
          commands.pick("page", "name").type("no spaces");
          commands.pick("page", "name").should("have.value", "no-spaces");

          // shows a validation message when name contains invalid characters

          // special characters are disallowed
          commands.pick("page", "name").clear();
          commands.pick("page", "name").type("bad$");
          commands
            .pick("page")
            .contains(
              "Your project name has unsupported characters in it. You can " +
                "only use basic letters, numbers, and the _ , - and . characters."
            );

          // can't start with a dash
          commands.pick("page", "name").clear();
          commands.pick("page", "name").type("-nope");
          commands
            .pick("page")
            .contains(
              "Your project name should start with a letter or a number."
            );

          // has to be at least two characters long
          commands.pick("page", "name").clear();
          commands.pick("page", "name").type("x");
          commands
            .pick("page")
            .contains(
              "Your project name should be at least 2 characters long."
            );

          // has to be no more than 64 characters long
          commands.pick("page", "name").clear();
          commands.pasteInto(["page", "name"], "x".repeat(257));
          commands
            .pick("page")
            .contains(
              "Your project name should not be longer than 64 characters."
            );
        });
      });

      context("description", () => {
        it("prevents the user from creating a project with an invalid description", () => {
          withEmptyDirectoryStub(() => {
            commands.pick("new-project", "choose-path-button").click();

            // entering a description is not mandatory and should not block
            // project creation
            commands.pick("page", "name").type("rx");
            commands.pick("page", "description").type("xxxx");
            commands.pick("create-project-button").should("be.enabled");
            commands.pick("page", "description").clear();
            commands.pick("create-project-button").should("be.enabled");

            // the project description has to be no more than 256 characters long
            commands.pick("page", "description").clear();
            commands.pasteInto(["page", "description"], "x".repeat(257));
            commands
              .pick("page")
              .contains(
                "Your project description should not be longer than 256 characters."
              );
            commands.pick("create-project-button").should("be.disabled");
          });
        });
      });

      context("new repository", () => {
        it("prevents the user from picking an invalid directory", () => {
          // shows a validation message when new project path is empty
          commands
            .pick("page", "new-project")
            .contains("Pick a directory for the new project")
            .should("exist");

          withPlatinumStub(_ => {
            commands.pick("new-project", "choose-path-button").click();

            commands
              .pick("page", "new-project")
              .contains(
                "Please choose a directory that's not already a git repository."
              )
              .should("exist");
          });
        });
      });

      context("form", () => {
        it("clears name input when switching from new to existing project", () => {
          commands.pick("name").clear();
          commands.pick("name").type("this-will-be-a-new-project");
          commands.pick("new-project").click();
          commands
            .pick("name")
            .should("have.value", "this-will-be-a-new-project");
          commands.pick("existing-project").click();
          commands.pick("name").should("have.value", "");
        });

        it("prevents the user from submitting invalid data", () => {
          // shows a validation message when new project path is empty
          commands
            .pick("page", "new-project")
            .contains("Pick a directory for the new project")
            .should("exist");
        });
      });
    });

    context("importing existing repositories", () => {
      it("preselects master as the default branch", () => {
        withPlatinumStub(repoName => {
          commands.pick("new-project-button").click();
          commands.pick("name").should("not.be.disabled");
          commands.pick("existing-project").click();
          commands.pick("name").should("be.disabled");
          commands.pick("existing-project", "choose-path-button").click();
          // Make sure the UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);
          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").contains("master");
        });
      });

      it("preselects main as the default branch", () => {
        withPlatinumStub(repoName => {
          createBranches(repoName, ["main"]);

          commands.pick("new-project-button").click();
          commands.pick("name").should("not.be.disabled");
          commands.pick("existing-project").click();
          commands.pick("name").should("be.disabled");
          commands.pick("existing-project", "choose-path-button").click();
          // Make sure the UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);
          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").contains("main");
        });
      });
    });

    it("disallows creating a project from a repository without commits", () => {
      withNoCommitsRepositoryStub(() => {
        commands.pick("new-project-button").click();

        commands.pick("existing-project").click();

        commands.pick("existing-project", "choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // this prevents this spec from failing on CI.
        cy.wait(500);

        commands
          .pick("existing-project")
          .contains(
            "The directory should contain a git repository with at least one branch"
          )
          .should("exist");
      });
    });

    context("happy paths", () => {
      context("creates a new project from an empty directory", () => {
        function go(expectedDefaultBranch: string) {
          withEmptyDirectoryStub(() => {
            commands.pick("new-project-button").click();

            commands.pick("name").type("new-fancy-project.xyz");
            commands.pick("description").type("My new fancy project");

            commands.pick("new-project").click();
            commands.pick("new-project", "choose-path-button").click();
            // Make sure UI has time to update path value from stub,
            // this prevents this spec from failing on CI.
            cy.wait(500);

            commands.pick("create-project-button").click();

            commands
              .pick("project-screen", "header")
              .contains("new-fancy-project");

            commands
              .pick("project-screen", "revision-selector")
              .contains(`${expectedDefaultBranch} default`);

            commands
              .pick("notification")
              .contains("Project new-fancy-project.xyz successfully created");

            commands.pick("profile").click();
            commands
              .pick("profile-screen", "project-list")
              .contains("new-fancy-project.xyz");
            commands
              .pick("profile-screen", "project-list")
              .contains("My new fancy project");
          });
        }

        it("picks the user-defined git default branch", () => {
          go("trunk");
        });

        it("picks the Upstream default git branch when it can not obtain the git global config one", () => {
          ipcStub.getStubs().then(stubs => {
            stubs.getGitGlobalDefaultBranch.returns(undefined);
          });
          go(config.UPSTREAM_DEFAULT_BRANCH);
        });
      });

      it("creates a new project from an existing repository", () => {
        withPlatinumStub(repoName => {
          commands.pick("new-project-button").click();

          commands.pick("name").should("not.be.disabled");

          commands.pick("existing-project").click();
          commands.pick("name").should("be.disabled");

          commands.pick("existing-project", "choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);

          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").contains("master");
          commands.pick("description").type("Best project");

          commands.pick("create-project-button").click();
          commands.pick("project-screen", "header").contains(repoName);

          commands.pick("project-screen", "header").contains("Best project");

          commands
            .pick("notification")
            .contains(`Project ${repoName} successfully created`);

          commands.pick("profile").click();
          commands.pick("profile-screen", "project-list").contains(repoName);
          commands
            .pick("profile-screen", "project-list")
            .contains("Best project");

          commands
            .pick("notification")
            .contains(`Project ${repoName} successfully created`)
            .should("exist");
          commands.pick("notification").contains("Close").click();

          // Make sure we can't add the same project twice.
          commands.pick("new-project-button").click();

          commands.pick("existing-project").click();

          commands.pick("existing-project", "choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);

          commands.pick("name").should("have.value", repoName);
          commands.pick("description").type("Best project");

          commands.pick("create-project-button").click();

          commands
            .pick("notification")
            .contains(
              /Could not create project: the URN `rad:git:[1-9A-HJ-NP-Za-km-z]{37}` already exists/
            )
            .should("exist");
        });
      });
    });
  });
});
