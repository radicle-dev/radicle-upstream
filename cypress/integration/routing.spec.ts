// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ipcTypes from "native/ipc-types";
import * as ipcStub from "cypress/support/ipc-stub";
import * as commands from "cypress/support/commands";

context("routing", () => {
  beforeEach(() => {
    commands.resetProxyState();
    cy.visit("./public/index.html");
  });

  context("session persistancy", () => {
    it("retains the navigation history after a browser refresh", () => {
      commands.onboardUser();
      commands.createProjectWithFixture();
      cy.visit("./public/index.html");
      commands.pick("project-list-entry-platinum").click();
      commands.pick("commits-tab").click();
      commands
        .pickWithContent(
          ["commit-teaser"],
          "Add files with special characters in their filenames (#5)"
        )
        .click();
      cy.reload();
      commands.pick("back-button").click();
      commands.pick("commits-page").should("exist");
    });

    context("first time app start with no stored session data", () => {
      it("opens on the identity creation wizard", () => {
        commands.pick("get-started-button").should("exist");
      });
    });

    context("when there is an identity stored in the session", () => {
      beforeEach(() => {
        commands.onboardUser();
      });

      context(
        "when there is no additional routing information stored in the browser location",
        () => {
          it("opens the app on the profile screen", () => {
            cy.visit("./public/index.html");
            cy.location().should(loc => {
              expect(loc.hash).to.eq("#/profile");
            });
          });
        }
      );

      context(
        "when there is additional routing information stored in the browser location",
        () => {
          it("resumes the app from the browser location", () => {
            cy.visit("./public/index.html");

            commands.pick("sidebar", "settings").click();

            cy.location().should(loc => {
              expect(loc.hash).to.eq("#/settings");
            });

            cy.reload();

            cy.location().should(loc => {
              expect(loc.hash).to.eq("#/settings");
            });
          });
        }
      );
    });
  });

  describe("blue screen of death", () => {
    it("shows blue screen of death if there is a proxy error before onboarding", () => {
      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.PROXY_ERROR,
          data: {
            status: 1,
            signal: null,
            output: "OUTPUT",
          },
        });
      });
      commands.pick("blue-screen-of-death").should("exist");
      commands.pick("proxy-log").should("contain", "OUTPUT");
      commands.pick("proxy-log-copy-clipboard").click();

      ipcStub.getStubs().then(stubs => {
        expect(stubs.getClipboard()).to.eq("OUTPUT");
      });
    });

    it("shows blue screen of death if there is a proxy error after onboarding", () => {
      commands.onboardUser();
      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.PROXY_ERROR,
          data: {
            status: 1,
            signal: null,
            output: "OUTPUT",
          },
        });
      });
      commands.pick("blue-screen-of-death").should("exist");
      commands.pick("proxy-log").should("contain", "OUTPUT");
      commands.pick("proxy-log-copy-clipboard").click();

      ipcStub.getStubs().then(stubs => {
        expect(stubs.getClipboard()).to.eq("OUTPUT");
      });
    });
  });
});
