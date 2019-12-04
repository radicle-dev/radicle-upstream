context("source code browsing", () => {
  beforeEach(() => {
    cy.visit("./public/index.html#/projects/rad/Monadic/source");
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
      cy.get("[data-cy=revision-selector]").should(
        "have.value",
        "refs/heads/master"
      );

      // there is a commit teaser
      cy.get("[data-cy=commit-teaser]")
        .contains("Rūdolfs Ošiņš")
        .should("exist");
      cy.get("[data-cy=commit-teaser]")
        .contains("Add dotfiles")
        .should("exist");
      cy.get("[data-cy=commit-teaser]")
        .contains("1820cb0")
        .should("exist");

      // it is the folder view
      cy.get("thead")
        .contains("Name")
        .should("exist");
      cy.get("thead")
        .contains("Commit Message")
        .should("exist");
      cy.get("thead")
        .contains("Last Update")
        .should("exist");

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

  context("sidebar source-tree", () => {
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

    it("shows a vertical scrollbar if the content doesn't fit", () => {
      cy.get("[data-cy=source-tree]").within(() => {
        cy.get("[data-cy=expand-bin]").click();
      });
      cy.contains("Branches").should("not.be.inViewport");

      // the scrollbar allows us to reach all the content in the sidebar
      cy.contains("Branches").scrollIntoView();
      cy.contains("Branches").should("be.inViewport");
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
        cy.contains("Initial commit FTW!").should("exist");
      });
    });

    context("relative timestamps", () => {
      context("when the timeframe is less than a day", () => {
        it("shows timeframe in hours", () => {
          cy.clock(Date.parse("3 dec 2019"));
          cy.contains("13 hours ago").should("exist");
        });
      });

      context("when the timeframe is less than a week", () => {
        it("shows timeframe in days", () => {
          cy.clock(Date.parse("4 dec 2019"));
          cy.contains("1 day ago").should("exist");
        });
      });

      context("when the timeframe is less than a week", () => {
        it("shows timeframe in weeks", () => {
          cy.clock(Date.parse("15 dec 2019"));
          cy.contains("1 week ago").should("exist");
        });
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

          cy.window().scrollTo("bottom");
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
