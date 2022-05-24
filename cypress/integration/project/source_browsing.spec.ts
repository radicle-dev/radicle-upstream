// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("project source browsing", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser("cloudhead");
    commands.createProjectWithFixture();
  });

  beforeEach(() => {
    cy.visit("./public/index.html");
    cy.contains("platinum").click();
  });

  context("repository stats", () => {
    it("shows the correct numbers", () => {
      commands
        .pickWithContent(["tab-bar", "commits-tab", "counter"], "15")
        .should("exist");
    });
  });

  context("commit browsing", () => {
    context("commit history", () => {
      it("shows the commit history for the default branch", () => {
        // Wait for the commit tab to be updated
        commands
          .pickWithContent(["tab-bar", "commits-tab", "counter"], "15")
          .should("exist");
        commands.pick("tab-bar", "commits-tab").click();
        commands.pick("commits-page").should("exist");
        commands
          .pickWithContent(["commit-teaser"], "Commit on the dev branch")
          .should("not.exist");
        commands
          .pick("commit-teaser")
          .contains("Merge pull request #4 from FintanH/fintan")
          .click();
        commands.pick("commit-page").should("exist");
        commands
          .pick("commit-header")
          .contains("Commit 223aaf8")
          .should("exist");
        commands.pick("commit-branch").should("contain", "master");
      });

      it("shows the commit history for another branch", () => {
        commands.pick("revision-selector").click();
        commands.pick("revision-dropdown", "revision-branch-dev").click();
        // Wait for the commit tab to be updated
        commands
          .pickWithContent(["tab-bar", "commits-tab", "counter"], "8")
          .should("exist");
        commands.pick("tab-bar", "commits-tab").click();

        commands.pick("commits-page").should("exist");
        commands
          .pickWithContent(
            ["commit-teaser"],
            "Merge pull request #4 from FintanH/fintan"
          )
          .should("not.exist");
        commands
          .pickWithContent(["commit-teaser"], "Commit on the dev branch")
          .click();
        commands
          .pickWithContent(["commit-header"], "Commit 27acd68")
          .should("exist");

        commands.pick("commit-branch").should("contain", "dev");
      });
    });
  });

  context("source code browsing", () => {
    context("relative timestamps", () => {
      context("when the timeframe is less than a day", () => {
        it("shows timeframe in hours", () => {
          cy.clock(Date.parse("5 dec 2019"));
          commands.pick("revision-selector").click();
          commands.pick("revision-tag-v0.5.0").click();
          cy.contains("9 hours ago").should("exist");
        });
      });

      context("when the timeframe is less than 2 days", () => {
        it("shows timeframe in days", () => {
          cy.clock(Date.parse("6 dec 2019"));
          commands.pick("revision-selector").click();
          commands.pick("revision-tag-v0.5.0").click();
          cy.contains("1 day ago").should("exist");
        });
      });

      context("when the timeframe is less than a week", () => {
        it("shows timeframe in days", () => {
          cy.clock(Date.parse("10 dec 2019"));
          commands.pick("revision-selector").click();
          commands.pick("revision-tag-v0.5.0").click();
          cy.contains("5 days ago").should("exist");
        });
      });

      context("when the timeframe is more than a week", () => {
        it("shows timeframe in weeks", () => {
          cy.clock(Date.parse("15 dec 2019"));
          commands.pick("revision-selector").click();
          commands.pick("revision-tag-v0.5.0").click();
          cy.contains("1 week ago").should("exist");
        });
      });

      context("when the timeframe is more than 2 weeks", () => {
        it("shows timeframe in weeks", () => {
          cy.clock(Date.parse("21 dec 2019"));
          commands.pick("revision-selector").click();
          commands.pick("revision-tag-v0.5.0").click();
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
              .pickWithContent(["project-screen", "commit-teaser"], "a0dd912")
              .should("exist");
            commands
              .pickWithContent(
                ["project-screen", "commit-teaser"],
                "Add files with special characters in their filenames (#5)"
              )
              .should("exist");
            commands
              .pickWithContent(
                ["project-screen", "commit-teaser"],
                "Rūdolfs Ošiņš"
              )
              .should("exist");

            commands
              .pickWithContent(["project-screen", "file-view"], "README.md")
              .should("exist");
            commands
              .pickWithContent(
                ["project-screen", "file-view"],
                "This repository is a data source for the Upstream front-end tests"
              )
              .should("exist");

            // Going to a different path and then clicking on 'Files'
            // sends us back to the project root, showing the README again.
            commands.pick("source-tree").within(() => {
              cy.contains(".i-am-well-hidden").click();
            });
            commands.pick("project-screen", "files-tab").click();
            commands
              .pickWithContent(["project-screen", "file-view"], "README.md")
              .should("exist");
            commands.pick("source-tree").within(() => {
              cy.contains(".i-too-am-hidden").click();
            });
            commands
              .pickWithContent(
                ["project-screen", "file-view"],
                ".i-too-am-hidden"
              )
              .should("exist");
            commands.pick("project-screen", "files-tab").click();
            commands
              .pickWithContent(["project-screen", "file-view"], "README.md")
              .should("exist");

            // Going to a different path and then clicking on the project name
            // sends us back to the project root, showing the README again.
            commands.pick("project-screen", "commits-tab").click();
            commands.pick("project-screen", "file-view").should("not.exist");
            commands.pick("header", "entity-name").click();
            commands
              .pickWithContent(["project-screen", "file-view"], "README.md")
              .should("exist");

            // Switching between different revisions shows the correct README
            commands.pick("revision-selector").click();
            commands.pick("revision-dropdown", "revision-branch-dev").click();
            commands
              .pickWithContent(["project-screen", "file-view"], "README.md")
              .should("exist");
            commands
              .pickWithContent(
                ["project-screen", "file-view"],
                "This repository is a data source for the Upstream front-end tests."
              )
              .should("exist");
          });
        });
      });

      context("revision selector", () => {
        it("allows switching to a different branch", () => {
          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-branch-dev").click();
          cy.contains("here-we-are-on-a-dev-branch.lol").should("exist");

          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-branch-master").click();
          cy.contains("here-we-are-on-a-dev-branch.lol").should("not.exist");
        });

        it("allows switching to a different tag", () => {
          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-tag-v0.4.0").click();
          cy.contains("test-file-deletion.txt").should("exist");

          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-tag-v0.5.0").click();
          cy.contains("test-file-deletion.txt").should("not.exist");
        });

        it("does not crash on a page reload", () => {
          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-branch-dev").click();

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
          commands
            .pick("revision-dropdown", "revision-branch-master")
            .should("have.class", "selected");
          commands.pick("revision-dropdown", "revision-branch-dev").click();
          // Switch to another peer
          commands.pick("peer-selector").click();
          cy.get('.peer-dropdown [data-peer-handle="abbey"]').click();

          commands
            .pickWithContent(["revision-selector"], "master")
            .should("exist");
          commands.pick("revision-selector", "branch-icon").should("exist");

          commands.pick("peer-selector").click();
          cy.get('.peer-dropdown [data-peer-handle="cloudhead"]').click();
          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-tag-v0.1.0").click();

          commands
            .pickWithContent(["revision-selector"], "v0.1.0")
            .should("exist");
          commands.pick("revision-selector", "tag-icon").should("exist");

          commands.pick("revision-selector").click();
          // Previous selection is highlighted.
          commands
            .pick("revision-dropdown", "revision-tag-v0.1.0")
            .should("have.class", "selected");
        });
      });

      context("when switching between projects", () => {
        it("opens the selected project on the default repository and branch", () => {
          commands.createProjectWithFixture("gold");
          commands.pick("revision-selector").click();
          commands.pick("revision-dropdown", "revision-branch-dev").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list", "project-list-entry-gold").click();
          commands
            .pickWithContent(["revision-selector"], "master")
            .should("exist");
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

      it("doesn't interfere with the tab active state", () => {
        commands
          .pick("tab-bar", "files-tab")
          .get("p")
          .should("have.class", "active");

        commands.pick("source-tree").within(() => {
          commands.pick("expand-text").click();
          commands.pick("file-text/arrows.txt").contains("arrows.txt").click();

          commands.pick("file-text/arrows.txt").should("have.class", "active");
        });

        commands
          .pick("tab-bar", "files-tab")
          .get("p")
          .should("have.class", "active");

        commands.pick("file-view", "file-header").click();

        commands
          .pick("tab-bar", "files-tab")
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
            cy.contains("special / -dash-").should("exist");

            commands.pick("source-tree").contains("...").click();
            cy.contains("special / ...").should("exist");

            commands.pick("source-tree").contains(":colon:").click();
            cy.contains("special / :colon:").should("exist");

            commands.pick("source-tree").contains(";semicolon;").click();
            cy.contains("special / ;semicolon;").should("exist");

            commands.pick("source-tree").contains("@at@").click();
            cy.contains("special / @at@").should("exist");

            commands.pick("source-tree").contains("_underscore_").click();
            cy.contains("special / _underscore_").should("exist");

            commands.pick("source-tree").contains("c++").click();
            cy.contains("special / c++").should("exist");

            commands.pick("source-tree").contains("faux\\path").click();
            cy.contains("special / faux\\path").should("exist");

            commands.pick("source-tree").contains("i need some space").click();
            cy.contains("special / i need some space").should("exist");

            commands
              .pick("source-tree")
              .contains("qs?param1=value?param2=value2#hash")
              .click();
            cy.contains("special / qs?param1=value?param2=value2#hash").should(
              "exist"
            );

            commands.pick("source-tree").contains("~tilde~").click();
            cy.contains("special / ~tilde~").should("exist");

            commands.pick("source-tree").contains("👹👹👹").click();
            cy.contains("special / 👹👹👹").should("exist");
          });
        });
      });
    });
  });
});
