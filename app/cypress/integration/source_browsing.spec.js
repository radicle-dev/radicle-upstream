beforeEach(() => {
  cy.visit("./public/index.html#/projects");
  cy.contains("Monadic").click();
  cy.contains("Source").click();
});

context("source code browsing", () => {
  context("relative timestamps", () => {
    context("when the timeframe is less than a day", () => {
      it("shows timeframe in hours", () => {
        cy.clock(Date.parse("5 dec 2019"));
        cy.get("[data-cy=revision-selector]").select("v0.5.0");
        cy.contains("9 hours ago").should("exist");
      });
    });

    context("when the timeframe is less than 2 days", () => {
      it("shows timeframe in days", () => {
        cy.clock(Date.parse("6 dec 2019"));
        cy.get("[data-cy=revision-selector]").select("v0.5.0");
        cy.contains("1 day ago").should("exist");
      });
    });

    context("when the timeframe is less than a week", () => {
      it("shows timeframe in days", () => {
        cy.clock(Date.parse("10 dec 2019"));
        cy.get("[data-cy=revision-selector]").select("v0.5.0");
        cy.contains("5 days ago").should("exist");
      });
    });

    context("when the timeframe is more than a week", () => {
      it("shows timeframe in weeks", () => {
        cy.clock(Date.parse("15 dec 2019"));
        cy.get("[data-cy=revision-selector]").select("v0.5.0");
        cy.contains("1 week ago").should("exist");
      });
    });

    context("when the timeframe is more than 2 weeks", () => {
      it("shows timeframe in weeks", () => {
        cy.clock(Date.parse("21 dec 2019"));
        cy.get("[data-cy=revision-selector]").select("v0.5.0");
        cy.contains("2 weeks ago").should("exist");
      });
    });
  });

  context("when the 'source' section is selected in project sidebar", () => {
    it("expands a tree starting at the root of the repo", () => {
      cy.get("[data-cy=source-tree]").within(() => {
        cy.contains("src").should("exist");
        cy.contains("README.md").should("exist");
      });
    });

    it("shows contents of the root folder for the latest revision", () => {
      // the default revision is selected
      cy.get("[data-cy=revision-selector]").should("have.value", "master");

      // there is a commit teaser
      cy.get("[data-cy=commit-teaser]")
        .contains("Fintan Halpenny")
        .should("exist");
      cy.get("[data-cy=commit-teaser]")
        .contains("Extend the docs (#2)")
        .should("exist");
      cy.get("[data-cy=commit-teaser]")
        .contains("3873745")
        .should("exist");

      // it is the folder view
      cy.get("[data-cy=file-list]").should("exist");

      // some top-level files/folders exist
      cy.get("tbody")
        .contains("bin")
        .should("exist");
      cy.get("tbody")
        .contains("README.md")
        .should("exist");
    });
  });

  context("page view", () => {
    context("revision selector", () => {
      it("allows switching to a different branch", () => {
        cy.get("[data-cy=revision-selector]").should("have.value", "master");

        cy.get("[data-cy=revision-selector]").select("origin/dev");
        cy.contains("here-we-are-on-a-dev-branch.lol").should("exist");
      });

      it("allows switching to a different tag", () => {
        cy.get("[data-cy=revision-selector]").should("have.value", "master");

        cy.get("[data-cy=revision-selector]").select("v0.4.0");
        cy.contains("test-file-deletion.txt").should("exist");
      });
    });

    context("when clicking on a directory", () => {
      it("allows diving deep into directory structures", () => {
        cy.get("[data-cy=file-list] [data-cy=open-this]").click();
        cy.get("[data-cy=file-list] [data-cy=open-is]").click();
        cy.get("[data-cy=file-list] [data-cy=open-a]").click();
        cy.get("[data-cy=file-list] [data-cy=open-really]").click();
        cy.get("[data-cy=file-list] [data-cy=open-deeply]").click();
        cy.get("[data-cy=file-list] [data-cy=open-nested]").click();
        cy.get("[data-cy=file-list] [data-cy=open-directory]").click();
        cy.get("[data-cy=file-list] [data-cy=open-tree]").click();
        cy.get("[data-cy=file-list]")
          .contains(".gitkeep")
          .should("exist");
      });
    });

    context("when clicking on a file", () => {
      it("shows the file contents", () => {
        cy.get("[data-cy=file-list] [data-cy=open-src]").click();
        cy.get("[data-cy=file-list]")
          .contains("Eval.hs")
          .click();

        cy.get("[data-cy=file-source]").within(() => {
          cy.contains("Eval.hs").should("exist");
          cy.contains("module Radicle.Lang.Eval").should("exist");
        });
      });
    });
  });

  context("source-tree", () => {
    it("shows files and directories", () => {
      cy.get("[data-cy=source-tree]").within(() => {
        // directories
        cy.contains("bin").should("exist");

        // files
        cy.contains("README.md").should("exist");

        // hidden files
        cy.contains(".i-am-well-hidden").should("exist");
      });
    });

    context("when clicking on the carret icon next to the folder name", () => {
      it("allows diving deep into directory structures", () => {
        cy.get("[data-cy=source-tree]").within(() => {
          cy.get("[data-cy=expand-this]").click();
          cy.get("[data-cy=expand-is]").click();
          cy.get("[data-cy=expand-a]").click();
          cy.get("[data-cy=expand-really]").click();
          cy.get("[data-cy=expand-deeply]").click();
          cy.get("[data-cy=expand-nested]").click();
          cy.get("[data-cy=expand-directory]").click();
          cy.get("[data-cy=expand-tree]").click();
          cy.contains(".gitkeep").should("exist");
        });

        // the main view of the page stays unchanged and shows the top level
        // directory listing
        cy.get("[data-cy=file-list]").should("exist");
      });
    });

    context("when clicking on a directory name", () => {
      it("shows the contents of the directory", () => {
        cy.get("[data-cy=source-tree]").within(() => {
          cy.contains("bin").click();

          // the source tree is not expanded and the directory contents are
          // not visible in the sidebar
          cy.contains("cat").should("not.exist");
        });

        // the directory is listed in the main view
        cy.contains("cat").should("exist");
        cy.contains("ls").should("exist");
        cy.contains("test").should("exist");
      });
    });

    context("when clicking on a file name", () => {
      context("for non-binary files", () => {
        it("shows the contents of the file", () => {
          cy.get("[data-cy=source-tree]").within(() => {
            cy.get("[data-cy=expand-src]").click();
            cy.contains("Eval.hs").click();
          });

          // the file path is shown in the header
          cy.contains("src/Eval.hs").should("exist");

          // file contents are shown
          cy.contains("module Radicle.Lang.Eval").should("exist");

          // line numbers are shown
          cy.contains("1\n2\n3\n4\n5\n").should("exist");

          cy.get("[data-cy=page-container]").scrollTo("bottom");
          // the scrollbar allows us to reach the bottom of the file
          cy.contains("callFn f' vs'").should("be.inViewport");
        });
      });

      context("for binary files", () => {
        it("does not render the binary content", () => {
          cy.get("[data-cy=source-tree]").within(() => {
            cy.get("[data-cy=expand-bin]").click();
            cy.contains("ls").click();
          });

          // the file path is shown in the header
          cy.contains("bin/ls").should("exist");

          // it instead shows a message
          cy.contains("Binary content.").should("exist");
        });
      });
    });
  });
});
