before(() => {
  cy.nukeAllState();
  cy.createIdentity("cloudhead");
  cy.createProjectWithFixture();
});

beforeEach(() => {
  cy.visit("./public/index.html#/profile/projects");
  cy.contains("platinum").click();
  cy.contains("Source").click();
});

context("repository stats", () => {
  it("shows the correct numbers", () => {
    cy.pick("repo-stats").contains("Commits 14");
    cy.pick("repo-stats").contains("Branches 2");
    cy.pick("repo-stats").contains("Contributors 4");
  });
});

context("commit browsing", () => {
  context("commit history", () => {
    it("shows the commit history for the default branch", () => {
      cy.pick("commits-button").click();
      cy.contains("27acd68").should("not.exist");
      cy.contains("91b69e0").click();
      cy.contains("91b69e00cd8e5a07e20942e9e4457d83ce7a3ff1").should("exist");
    });

    it("shows the commit history for another branch", () => {
      cy.pick("revision-selector").click();
      cy.get('[data-branch="dev"][data-repo-handle="cloudhead"]').click();
      cy.pick("commits-button").click();
      cy.contains("91b69e0").should("not.exist");
      cy.contains("27acd68").click();
      cy.contains("27acd68c7504755aa11023300890bb85bbd69d45").should("exist");
    });
  });
});

context("source code browsing", () => {
  context("relative timestamps", () => {
    context("when the timeframe is less than a day", () => {
      it("shows timeframe in hours", () => {
        cy.clock(Date.parse("5 dec 2019"));
        cy.pick("revision-selector").click();
        cy.get('[data-tag="v0.5.0"][data-repo-handle="cloudhead"]').click();
        cy.contains("9 hours ago").should("exist");
      });
    });

    context("when the timeframe is less than 2 days", () => {
      it("shows timeframe in days", () => {
        cy.clock(Date.parse("6 dec 2019"));
        cy.pick("revision-selector").click();
        cy.get('[data-tag="v0.5.0"][data-repo-handle="cloudhead"]').click();
        cy.contains("1 day ago").should("exist");
      });
    });

    context("when the timeframe is less than a week", () => {
      it("shows timeframe in days", () => {
        cy.clock(Date.parse("10 dec 2019"));
        cy.pick("revision-selector").click();
        cy.get('[data-tag="v0.5.0"][data-repo-handle="cloudhead"]').click();
        cy.contains("5 days ago").should("exist");
      });
    });

    context("when the timeframe is more than a week", () => {
      it("shows timeframe in weeks", () => {
        cy.clock(Date.parse("15 dec 2019"));
        cy.pick("revision-selector").click();
        cy.get('[data-tag="v0.5.0"][data-repo-handle="cloudhead"]').click();
        cy.contains("1 week ago").should("exist");
      });
    });

    context("when the timeframe is more than 2 weeks", () => {
      it("shows timeframe in weeks", () => {
        cy.clock(Date.parse("21 dec 2019"));
        cy.pick("revision-selector").click();
        cy.get('[data-tag="v0.5.0"][data-repo-handle="cloudhead"]').click();
        cy.contains("2 weeks ago").should("exist");
      });
    });
  });

  context("when the Source menu item is selected in project top-bar", () => {
    it("expands a tree starting at the root of the repo", () => {
      cy.pick("source-tree").within(() => {
        cy.contains("src").should("exist");
        cy.contains("README.md").should("exist");
      });
    });

    it("shows readme file for the latest revision", () => {
      // the default revision is selected
      cy.get('[data-cy=revision-selector][data-revision="master"]').should(
        "exist"
      );

      // there is a commit teaser
      cy.pick("commit-teaser").contains("Alexander Simmerl").should("exist");
      cy.pick("commit-teaser")
        .contains(
          "Merge pull request #4 from FintanH/fintan/update-readme-no-sig"
        )
        .should("exist");
      cy.pick("commit-teaser").contains("223aaf8").should("exist");

      // the readme is shown
      cy.pick("file-source").within(() => {
        cy.contains("README.md").should("exist");
      });
    });
  });

  context("page view", () => {
    context("when we're looking at the project root", () => {
      context("when there is a README file", () => {
        it("shows the README file", () => {
          // It contains the commit teaser for the latest commit.
          cy.pick("project-screen", "commit-teaser").contains("223aaf8");
          cy.pick("project-screen", "commit-teaser").contains(
            "Merge pull request #4"
          );
          cy.pick("project-screen", "commit-teaser").contains(
            "Alexander Simmerl"
          );

          cy.pick("project-screen", "file-source").contains("README.md");
          cy.pick("project-screen", "file-source").contains(
            "This repository is a data source for the Upstream front-end tests and the radicle-surf unit tests."
          );

          // Going to a different path and then switching back to the root path
          // shows the README again.
          cy.pick("source-tree").within(() => {
            cy.contains(".i-am-well-hidden").click();
          });
          cy.pick("project-screen", "file-source").contains(
            "platinum / .i-am-well-hidden"
          );
          cy.pick("project-screen", "file-source").contains("platinum").click();
          cy.pick("project-screen", "file-source").contains("README.md");

          cy.pick("source-tree").within(() => {
            cy.contains(".i-too-am-hidden").click();
          });
          cy.pick("project-screen", "file-source").contains(
            "platinum / .i-too-am-hidden"
          );
          cy.pick("topbar", "project-avatar").contains("platinum").click();
          cy.pick("project-screen", "file-source").contains("README.md");

          // Switching between different revisions shows the correct README
          cy.pick("revision-selector").click();
          cy.get(
            '.revision-dropdown [data-branch="dev"][data-repo-handle="cloudhead"]'
          ).click();
          cy.pick("project-screen", "file-source").contains("README.md");
          cy.pick("project-screen", "file-source").contains(
            "This repository is a data source for the Upstream front-end tests."
          );
        });
      });
    });

    context("revision selector", () => {
      it("allows switching to a different branch", () => {
        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-branch="dev"][data-repo-handle="cloudhead"]'
        ).click();
        cy.contains("here-we-are-on-a-dev-branch.lol").should("exist");

        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-branch="master"][data-repo-handle="cloudhead"]'
        ).click();
        cy.contains("here-we-are-on-a-dev-branch.lol").should("not.exist");
      });

      it("allows switching to a different tag", () => {
        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-tag="v0.4.0"][data-repo-handle="cloudhead"]'
        ).click();
        cy.contains("test-file-deletion.txt").should("exist");

        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-tag="v0.5.0"][data-repo-handle="cloudhead"]'
        ).click();
        cy.contains("test-file-deletion.txt").should("not.exist");
      });

      it("does not crash on a page reload", () => {
        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-branch="dev"][data-repo-handle="cloudhead"]'
        ).click();

        cy.reload();

        // Make sure the revision selector still loads.
        cy.contains(".i-too-am-hidden").should("exist");
      });
    });

    context("when switching between projects", () => {
      it("opens the selected project on the default repository and branch", () => {
        cy.createProjectWithFixture("gold");
        cy.pick("revision-selector").click();
        cy.get('[data-branch="dev"][data-repo-handle="cloudhead"]').click();
        cy.pick("sidebar", "profile").click();
        cy.pick("project-list", "project-list-entry-gold").click();
        cy.pick("revision-selector").contains("master");
      });
    });
  });

  context("source-tree", () => {
    it("shows files and directories", () => {
      cy.pick("source-tree").within(() => {
        // directories
        cy.contains("bin").should("exist");

        // files
        cy.contains("README.md").should("exist");

        // hidden files
        cy.contains(".i-am-well-hidden").should("exist");
      });
    });

    it("doesn't interfere with the top-bar menu item active state", () => {
      cy.pick("topbar", "horizontal-menu", "Source")
        .get("p")
        .should("have.class", "active");

      cy.pick("source-tree").within(() => {
        cy.pick("expand-text").click();
        cy.contains("arrows.txt").click();
        cy.contains("arrows.txt").should("have.class", "active");
      });

      cy.pick("topbar", "horizontal-menu", "Source")
        .get("p")
        .should("have.class", "active");

      cy.pick("file-source", "file-header").contains("platinum").click();

      cy.pick("topbar", "horizontal-menu", "Source")
        .get("p")
        .should("have.class", "active");
    });

    it("allows navigating the tree structure", () => {
      cy.pick("source-tree").within(() => {
        // Traverse deeply nested folders.
        cy.pick("expand-this").click();
        cy.pick("expand-is").click();
        cy.pick("expand-a").click();
        cy.pick("expand-really").click();
        cy.pick("expand-deeply").click();
        cy.pick("expand-nested").click();
        cy.pick("expand-directory").click();
        cy.pick("expand-tree").click();

        // Open a file within nested folders.
        cy.contains(".gitkeep").click();
        cy.contains(".gitkeep").should("have.class", "active");

        // Preserve expanded folder state when selecting a different file.
        cy.pick("expand-text").click();
        cy.contains("arrows.txt").click();
        cy.contains("arrows.txt").should("have.class", "active");
        cy.contains(".gitkeep").should("not.have.class", "active");
      });
    });

    it("highlights the selected file", () => {
      cy.pick("source-tree").within(() => {
        cy.contains(".i-am-well-hidden").should("not.have.class", "active");
        cy.contains(".i-am-well-hidden").click();
        cy.contains(".i-am-well-hidden").should("have.class", "active");
      });
    });

    context("when clicking on a file name", () => {
      context("for non-binary files", () => {
        it("shows the contents of the file", () => {
          cy.pick("source-tree").within(() => {
            cy.pick("expand-src").click();
            cy.contains("Eval.hs").click();
          });

          // the file path is shown in the header
          cy.contains("src / Eval.hs").should("exist");

          // file contents are shown
          cy.contains("module Radicle.Lang.Eval").should("exist");

          // line numbers are shown
          cy.contains("1\n2\n3\n4\n5\n").should("exist");

          cy.pick("scrollable-content").scrollTo("bottom");
          // the scrollbar allows us to reach the bottom of the file
          cy.contains("callFn f' vs'").should("be.inViewport");
        });
      });

      context("for binary files", () => {
        it("does not render the binary content", () => {
          cy.pick("source-tree").within(() => {
            cy.pick("expand-bin").click();
            cy.contains("ls").click();
          });

          // the file path is shown in the header
          cy.contains("bin / ls").should("exist");

          // it instead shows a message
          cy.contains("Binary content.").should("exist");
        });
      });
    });
  });
});
