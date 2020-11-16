import * as commands from "../../support/commands";

context("project source browsing", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");

    // TODO(sos): add fake peers again when we have a peer testnet
    commands.createProjectWithFixture();
  });

  beforeEach(() => {
    cy.visit("./public/index.html#/profile/projects");
    cy.contains("platinum").click();
  });

  context("repository stats", () => {
    it("shows the correct numbers", () => {
      commands.pick("header", "project-stats").contains("2 Branches");
      commands.pick("header", "project-stats").contains("4 Contributors");
      commands.pick("horizontal-menu", "Commits", "counter").contains("15");
    });
  });

  context("commit browsing", () => {
    context("commit history", () => {
      it("shows the commit history for the default branch", () => {
        // Wait for the commit tab to be updated
        commands.pick("horizontal-menu", "Commits", "counter").contains("15");
        commands.pick("horizontal-menu", "Commits").click();
        commands.pick("commits-page").should("exist");
        commands
          .pick("commit-teaser")
          .contains("Commit on the dev branch")
          .should("not.exist");
        commands
          .pick("commit-teaser")
          .contains("Merge pull request #4 from FintanH/fintan")
          .click();
        commands.pick("commit-page").should("exist");
        commands
          .pick("commit-header")
          .contains("Commit 223aaf87d6ea62eef0014857640fd7c8dd0f80b5")
          .should("exist");
      });

      it("shows the commit history for another branch", () => {
        commands.pick("revision-selector").click();
        cy.get('[data-branch="dev"]').click();
        // Wait for the commit tab to be updated
        commands.pick("horizontal-menu", "Commits", "counter").contains("8");
        commands.pick("horizontal-menu", "Commits").click();

        commands.pick("commits-page").should("exist");
        commands
          .pick("commit-teaser")
          .contains("Merge pull request #4 from FintanH/fintan")
          .should("not.exist");
        commands
          .pick("commit-teaser")
          .contains("Commit on the dev branch")
          .click();
        commands
          .pick("commit-header")
          .contains("Commit 27acd68c7504755aa11023300890bb85bbd69d45")
          .should("exist");
      });
    });
  });

  context("source code browsing", () => {
    context("relative timestamps", () => {
      context("when the timeframe is less than a day", () => {
        it("shows timeframe in hours", () => {
          cy.clock(Date.parse("5 dec 2019"));
          commands.pick("revision-selector").click();
          cy.get('[data-tag="v0.5.0"]').click();
          cy.contains("9 hours ago").should("exist");
        });
      });

      context("when the timeframe is less than 2 days", () => {
        it("shows timeframe in days", () => {
          cy.clock(Date.parse("6 dec 2019"));
          commands.pick("revision-selector").click();
          cy.get('[data-tag="v0.5.0"]').click();
          cy.contains("1 day ago").should("exist");
        });
      });

      context("when the timeframe is less than a week", () => {
        it("shows timeframe in days", () => {
          cy.clock(Date.parse("10 dec 2019"));
          commands.pick("revision-selector").click();
          cy.get('[data-tag="v0.5.0"]').click();
          cy.contains("5 days ago").should("exist");
        });
      });

      context("when the timeframe is more than a week", () => {
        it("shows timeframe in weeks", () => {
          cy.clock(Date.parse("15 dec 2019"));
          commands.pick("revision-selector").click();
          cy.get('[data-tag="v0.5.0"]').click();
          cy.contains("1 week ago").should("exist");
        });
      });

      context("when the timeframe is more than 2 weeks", () => {
        it("shows timeframe in weeks", () => {
          cy.clock(Date.parse("21 dec 2019"));
          commands.pick("revision-selector").click();
          cy.get('[data-tag="v0.5.0"]').click();
          cy.contains("2 weeks ago").should("exist");
        });
      });
    });

    context("when the Source menu item is selected in project top-bar", () => {
      it("expands a tree starting at the root of the repo", () => {
        commands.pick("source-tree").within(() => {
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
        commands
          .pick("commit-teaser")
          .contains("Rūdolfs Ošiņš")
          .should("exist");
        commands
          .pick("commit-teaser")
          .contains("Add files with special characters in their filenames (#5)")
          .should("exist");
        commands.pick("commit-teaser").contains("a0dd912").should("exist");

        // the readme is shown
        commands.pick("file-view").within(() => {
          cy.contains("README.md").should("exist");
        });
      });
    });

    context("page view", () => {
      context("when we're looking at the project root", () => {
        context("when there is a README file", () => {
          it("shows the README file", () => {
            // It contains the commit teaser for the latest commit.
            commands
              .pick("project-screen", "commit-teaser")
              .contains("a0dd912");
            commands
              .pick("project-screen", "commit-teaser")
              .contains(
                "Add files with special characters in their filenames (#5)"
              );
            commands
              .pick("project-screen", "commit-teaser")
              .contains("Rūdolfs Ošiņš");

            commands.pick("project-screen", "file-view").contains("README.md");
            commands
              .pick("project-screen", "file-view")
              .contains(
                "This repository is a data source for the Upstream front-end tests and the radicle-surf unit tests."
              );

            // Going to a different path and then switching back to the root path
            // shows the README again.
            commands.pick("source-tree").within(() => {
              cy.contains(".i-am-well-hidden").click();
            });
            commands
              .pick("project-screen", "file-view")
              .contains("platinum / .i-am-well-hidden");
            commands
              .pick("project-screen", "file-view")
              .contains("platinum")
              .click();
            commands.pick("project-screen", "file-view").contains("README.md");

            commands.pick("source-tree").within(() => {
              cy.contains(".i-too-am-hidden").click();
            });
            commands
              .pick("project-screen", "file-view")
              .contains("platinum / .i-too-am-hidden");
            commands.pick("project-screen", "file-view", "root-link").click();
            commands.pick("project-screen", "file-view").contains("README.md");

            // Switching between different revisions shows the correct README
            commands.pick("revision-selector").click();
            cy.get('.revision-dropdown [data-branch="dev"]').click();
            commands.pick("project-screen", "file-view").contains("README.md");
            commands
              .pick("project-screen", "file-view")
              .contains(
                "This repository is a data source for the Upstream front-end tests."
              );
          });
        });
      });

      context("revision selector", () => {
        it("allows switching to a different branch", () => {
          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-branch="dev"]').click();
          cy.contains("here-we-are-on-a-dev-branch.lol").should("exist");

          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-branch="master"]').click();
          cy.contains("here-we-are-on-a-dev-branch.lol").should("not.exist");
        });

        it("allows switching to a different tag", () => {
          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-tag="v0.4.0"]').click();
          cy.contains("test-file-deletion.txt").should("exist");

          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-tag="v0.5.0"]').click();
          cy.contains("test-file-deletion.txt").should("not.exist");
        });

        it("does not crash on a page reload", () => {
          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-branch="dev"]').click();

          cy.reload();

          // Make sure the revision selector still loads.
          cy.contains(".i-too-am-hidden").should("exist");
        });
      });

      context("peer selector", () => {
        // TODO(sos): unskip when we have a proxy testnet
        it.skip("highlights the selected peer", () => {
          commands.pick("peer-selector").click();
          // Default peer is highlighted.
          cy.get('.peer-dropdown [data-peer-handle="cloudhead"]').should(
            "have.class",
            "selected"
          );
          // Switch to another peer
          cy.get('.peer-dropdown [data-peer-handle="abbey"]').click();
          commands.pick("peer-selector").click();
          // Selected peer is highlighted.
          cy.get('.peer-dropdown [data-peer-handle="abbey"]').should(
            "have.class",
            "selected"
          );
        });

        // TODO(sos): unskip when we have a proxy testnet
        it.skip("updates the revision selector", () => {
          commands.pick("revision-selector").click();
          // Default revision is highlighted.
          cy.get('.revision-dropdown [data-branch="master"]').should(
            "have.class",
            "selected"
          );
          cy.get('.revision-dropdown [data-branch="dev"]').click();
          // Switch to another peer
          commands.pick("peer-selector").click();
          cy.get('.peer-dropdown [data-peer-handle="abbey"]').click();

          commands.pick("revision-selector").contains("master");
          commands.pick("revision-selector", "branch-icon").should("exist");

          commands.pick("peer-selector").click();
          cy.get('.peer-dropdown [data-peer-handle="cloudhead"]').click();
          commands.pick("revision-selector").click();
          cy.get('.revision-dropdown [data-tag="v0.1.0"]').click();

          commands.pick("revision-selector").contains("v0.1.0");
          commands.pick("revision-selector", "tag-icon").should("exist");

          commands.pick("revision-selector").click();
          // Previous selection is highlighted.
          cy.get('.revision-dropdown [data-tag="v0.1.0"]').should(
            "have.class",
            "selected"
          );
        });
      });

      context("when switching between projects", () => {
        it("opens the selected project on the default repository and branch", () => {
          commands.createProjectWithFixture("gold");
          commands.pick("revision-selector").click();
          cy.get('[data-branch="dev"]').click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list", "project-list-entry-gold").click();
          commands.pick("revision-selector").contains("master");
        });
      });
    });

    context("source-tree", () => {
      it("shows files and directories", () => {
        commands.pick("source-tree").within(() => {
          // directories
          cy.contains("bin").should("exist");

          // files
          cy.contains("README.md").should("exist");

          // hidden files
          cy.contains(".i-am-well-hidden").should("exist");
        });
      });

      it("doesn't interfere with the horizontal menu item active state", () => {
        commands
          .pick("horizontal-menu", "Files")
          .get("p")
          .should("have.class", "active");

        commands.pick("source-tree").within(() => {
          commands.pick("expand-text").click();
          commands.pick("file-text/arrows.txt").contains("arrows.txt").click();

          commands.pick("file-text/arrows.txt").should("have.class", "active");
        });

        commands
          .pick("horizontal-menu", "Files")
          .get("p")
          .should("have.class", "active");

        commands.pick("file-view", "file-header").contains("platinum").click();

        commands
          .pick("horizontal-menu", "Files")
          .get("p")
          .should("have.class", "active");
      });

      it("allows navigating the tree structure", () => {
        commands.pick("source-tree").within(() => {
          // Traverse deeply nested folders.
          commands.pick("expand-this").click();
          commands.pick("expand-is").click();
          commands.pick("expand-a").click();
          commands.pick("expand-really").click();
          commands.pick("expand-deeply").click();
          commands.pick("expand-nested").click();
          commands.pick("expand-directory").click();
          commands.pick("expand-tree").click();

          // Open a file within nested folders.
          commands
            .pick("file-this/is/a/really/deeply/nested/directory/tree/.gitkeep")
            .click();
          commands
            .pick("file-this/is/a/really/deeply/nested/directory/tree/.gitkeep")
            .should("have.class", "active");

          // Preserve expanded folder state when selecting a different file.
          cy.scrollTo("top");
          commands.pick("expand-text").click();
          commands.pick("file-text/arrows.txt").click();
          commands.pick("file-text/arrows.txt").should("have.class", "active");
          commands
            .pick("file-this/is/a/really/deeply/nested/directory/tree/.gitkeep")
            .should("not.have.class", "active");
        });
      });

      it("highlights the selected file", () => {
        commands.pick("source-tree").within(() => {
          commands
            .pick("file-.i-am-well-hidden")
            .should("not.have.class", "active");
          commands.pick("file-.i-am-well-hidden").click();
          commands
            .pick("file-.i-am-well-hidden")
            .should("have.class", "active");
        });
      });

      context("when clicking on a file name", () => {
        context("for non-binary files", () => {
          it("shows the contents of the file", () => {
            commands.pick("source-tree").within(() => {
              commands.pick("expand-src").click();
              cy.contains("Eval.hs").click();
            });

            // the file path is shown in the header
            cy.contains("src / Eval.hs").should("exist");

            // file contents are shown
            cy.contains("module Radicle.Lang.Eval").should("exist");

            // line numbers are shown
            cy.contains("1\n2\n3\n4\n5\n").should("exist");

            cy.scrollTo("bottom");
            // the scrollbar allows us to reach the bottom of the file
            cy.contains("callFn f' vs'").should("be.inViewport");
          });
        });

        context("for binary files", () => {
          it("does not render the binary content", () => {
            commands.pick("source-tree").within(() => {
              commands.pick("expand-bin").click();
              cy.contains("ls").click();
            });

            // the file path is shown in the header
            cy.contains("bin / ls").should("exist");

            // it instead shows a message
            cy.contains("Binary content").should("exist");
          });
        });

        context("for filenames with special characters", () => {
          it("does not break", () => {
            commands.pick("expand-special").click();

            commands.pick("source-tree").contains("-dash-").click();
            cy.contains("platinum / special / -dash-").should("exist");

            commands.pick("source-tree").contains("...").click();
            cy.contains("platinum / special / ...").should("exist");

            commands.pick("source-tree").contains(":colon:").click();
            cy.contains("platinum / special / :colon:").should("exist");

            commands.pick("source-tree").contains(";semicolon;").click();
            cy.contains("platinum / special / ;semicolon;").should("exist");

            commands.pick("source-tree").contains("@at@").click();
            cy.contains("platinum / special / @at@").should("exist");

            commands.pick("source-tree").contains("_underscore_").click();
            cy.contains("platinum / special / _underscore_").should("exist");

            commands.pick("source-tree").contains("c++").click();
            cy.contains("platinum / special / c++").should("exist");

            commands.pick("source-tree").contains("faux\\path").click();
            cy.contains("platinum / special / faux\\path").should("exist");

            commands.pick("source-tree").contains("i need some space").click();
            cy.contains("platinum / special / i need some space").should(
              "exist"
            );

            commands
              .pick("source-tree")
              .contains("qs?param1=value?param2=value2#hash")
              .click();
            cy.contains(
              "platinum / special / qs?param1=value?param2=value2#hash"
            ).should("exist");

            commands.pick("source-tree").contains("~tilde~").click();
            cy.contains("platinum / special / ~tilde~").should("exist");

            commands.pick("source-tree").contains("👹👹👹").click();
            cy.contains("platinum / special / 👹👹👹").should("exist");
          });
        });
      });
    });
  });
});
