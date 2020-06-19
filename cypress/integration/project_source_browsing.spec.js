before(() => {
  cy.nukeAllState();
  cy.createProjectWithFixture();
  cy.createIdentity();
});

beforeEach(() => {
  cy.visit("./public/index.html#/profile");
  cy.contains("Monadic").click();
  cy.contains("Source").click();
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

  context("when the 'source' section is selected in project sidebar", () => {
    it("expands a tree starting at the root of the repo", () => {
      cy.pick("source-tree").within(() => {
        cy.contains("src").should("exist");
        cy.contains("README.md").should("exist");
      });
    });

    it("shows readme file for the latest revision", () => {
      // the default revision is selected
      cy.get('[data-cy=revision-selector][data-revision="master"]').should(
        "exist",
      );

      // there is a commit teaser
      cy.pick("commit-teaser").contains("Alexander Simmerl").should("exist");
      cy.pick("commit-teaser")
        .contains(
          "Merge pull request #4 from FintanH/fintan/update-readme-no-sig",
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
    context("revision selector", () => {
      it("allows switching to a different branch", () => {
        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-branch="dev"][data-repo-handle="cloudhead"]',
        ).click();
        cy.contains("here-we-are-on-a-dev-branch.lol").should("exist");

        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-branch="master"][data-repo-handle="cloudhead"]',
        ).click();
        cy.contains("here-we-are-on-a-dev-branch.lol").should("not.exist");
      });

      it("allows switching to a different tag", () => {
        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-tag="v0.4.0"][data-repo-handle="cloudhead"]',
        ).click();
        cy.contains("test-file-deletion.txt").should("exist");

        cy.pick("revision-selector").click();
        cy.get(
          '.revision-dropdown [data-tag="v0.5.0"][data-repo-handle="cloudhead"]',
        ).click();
        cy.contains("test-file-deletion.txt").should("not.exist");
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

    context("when clicking on folder names", () => {
      it("allows diving deep into directory structures", () => {
        cy.pick("source-tree").within(() => {
          cy.pick("expand-this").click();
          cy.pick("expand-is").click();
          cy.pick("expand-a").click();
          cy.pick("expand-really").click();
          cy.pick("expand-deeply").click();
          cy.pick("expand-nested").click();
          cy.pick("expand-directory").click();
          cy.pick("expand-tree").click();
          cy.contains(".gitkeep").should("exist");
        });
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
