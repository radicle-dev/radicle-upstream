// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ipcStub from "cypress/support/ipc-stub";
import * as commands from "cypress/support/commands";
import * as project from "ui/src/project";

context.skip("project creation", () => {
  const withEmptyDirectoryStub = (callback: () => void) => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      // We’re deliberately using a path with spaces and non-ascii
      // characters.
      const emptyDirectoryPath = `${pwd}/cypress/workspace/empty directöry`;

      cy.exec(`rm -rf "${emptyDirectoryPath}"`);
      cy.exec(`mkdir -p "${emptyDirectoryPath}"`);

      ipcStub.getStubs().then(stubs => {
        stubs.selectDirectory.returns(emptyDirectoryPath);
      });

      callback();

      // clean up the fixture
      cy.exec(`rm -rf "${emptyDirectoryPath}"`);
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
        commands.pick("create-project-modal").should("exist");
        commands.pick("cancel-button").click();
        commands.pick("profile-screen").should("exist");
      });

      it("can be closed by pressing escape key", () => {
        commands.pick("new-project-button").click();
        commands.pick("create-project-modal").should("exist");
        cy.get("body").type("{esc}");
        commands.pick("profile-screen").should("exist");
      });
    });

    context("validations", () => {
      beforeEach(() => {
        commands.pick("new-project-button").click();

        // Set up minimal form input to show validations
        commands.pick("name").type("this-name-is-valid");
        commands.pick("new-project").click();
      });

      afterEach(() => {
        cy.get("body").type("{esc}", { force: true });
      });

      context("name", () => {
        it("prevents the user from creating a project with an invalid name", () => {
          // the submit button is disabled when name is not present
          commands.pick("name").clear();
          commands.pick("create-project-button").should("be.disabled");

          // spaces should be changed into dashes
          commands.pick("name").type("no spaces");
          commands.pick("name").should("have.value", "no-spaces");

          // shows a validation message when name contains invalid characters

          // special characters are disallowed
          commands.pick("name").clear();
          commands.pick("name").type("bad$");
          commands
            .pick("create-project-modal")
            .should(
              "contain",
              "Your project’s name has some characters that aren’t " +
                "supported. You can only use basic letters, numbers, " +
                "and the _ , - and . characters."
            );

          // can't start with a dash
          commands.pick("name").clear();
          commands.pick("name").type("-nope");
          commands
            .pick("create-project-modal")
            .should(
              "contain",
              "Your project name should start with a letter or a number."
            );

          // has to be at least two characters long
          commands.pick("name").clear();
          commands.pick("name").type("x");
          commands
            .pick("create-project-modal")
            .should(
              "contain",
              "Oops, your project’s name needs to be at least 2 characters long."
            );

          // has to be no more than 64 characters long
          commands.pick("name").clear();
          commands.pasteInto(["name"], "x".repeat(257));
          commands
            .pick("create-project-modal")
            .should(
              "contain",
              "Oh, your project’s name can’t have more than 64 characters."
            );
        });
      });

      context("description", () => {
        it("prevents the user from creating a project with an invalid description", () => {
          withEmptyDirectoryStub(() => {
            commands.pick("new-project", "choose-path-button").click();

            // entering a description is not mandatory and should not block
            // project creation
            commands.pick("name").type("rx");
            commands.pick("description").type("xxxx");
            commands.pick("create-project-button").should("be.enabled");
            commands.pick("description").clear();
            commands.pick("create-project-button").should("be.enabled");

            // the project description has to be no more than 256 characters long
            commands.pick("description").clear();
            commands.pasteInto(["description"], "x".repeat(257));
            commands
              .pick("create-project-modal")
              .should(
                "contain",
                "Whoa Shakespeare, your project’s description can’t be " +
                  "longer than 256 characters. Shorten it a bit!"
              );
            commands.pick("create-project-button").should("be.disabled");
          });
        });
      });

      context("new repository", () => {
        it("prevents the user from picking an invalid directory", () => {
          // shows a validation message when new project path is empty
          commands
            .pick("new-project")
            .contains("Pick a directory for the new project")
            .should("exist");

          withPlatinumStub(_ => {
            commands.pick("new-project", "choose-path-button").click();

            commands
              .pick("new-project")
              .should(
                "contain",
                "Please choose a directory that’s not already a git repository."
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
            .pick("new-project")
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
          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").should("contain", "master");
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
          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").should("contain", "main");
        });
      });
    });

    it("disallows creating a project from a repository without commits", () => {
      withNoCommitsRepositoryStub(() => {
        commands.pick("new-project-button").click();

        commands.pick("existing-project").click();

        commands.pick("existing-project", "choose-path-button").click();

        commands
          .pick("existing-project")
          .should(
            "contain",
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

            commands.pick("create-project-button").click();

            commands
              .pick("project-screen", "header")
              .should("contain", "new-fancy-project");

            commands
              .pick("project-screen", "revision-selector")
              .should("contain", `${expectedDefaultBranch} default`);

            commands
              .pick("notification")
              .should("contain", "Project new-fancy-project.xyz was created!");

            commands.pick("profile").click();
            commands
              .pick("profile-screen", "project-list")
              .should("contain", "new-fancy-project.xyz");
            commands
              .pick("profile-screen", "project-list")
              .should("contain", "My new fancy project");
          });
        }

        it("picks the user-defined git default branch", () => {
          go("trunk");
        });

        it("picks the Upstream default git branch when it can not obtain the git global config one", () => {
          ipcStub.getStubs().then(stubs => {
            stubs.getGitGlobalDefaultBranch.returns(undefined);
          });
          go(project.UPSTREAM_DEFAULT_BRANCH);
        });
      });

      it("creates a new project from an existing repository", () => {
        withPlatinumStub(repoName => {
          commands.pick("new-project-button").click();

          commands.pick("name").should("not.be.disabled");

          commands.pick("existing-project").click();
          commands.pick("name").should("be.disabled");

          commands.pick("existing-project", "choose-path-button").click();

          commands.pick("name").should("have.value", repoName);
          commands.pick("default-branch").should("contain", "master");
          commands.pick("description").type("Best project");

          commands.pick("create-project-button").click();
          commands.pick("project-screen", "header").should("contain", repoName);

          commands
            .pick("project-screen", "header")
            .should("contain", "Best project");

          commands
            .pick("notification")
            .contains(`Project ${repoName} was created!`)
            .should("exist");

          commands.pick("profile").click();
          commands
            .pick("profile-screen", "project-list")
            .should("contain", repoName);
          commands
            .pick("profile-screen", "project-list")
            .should("contain", "Best project");

          commands
            .pick("notification")
            .should("contain", `Project ${repoName} was created!`);
          commands.pick("notification").contains("Close").click();

          // Make sure we can't add the same project twice.
          commands.pick("new-project-button").click();

          commands.pick("existing-project").click();

          commands.pick("existing-project", "choose-path-button").click();

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
