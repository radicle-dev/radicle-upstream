import * as ipcTypes from "../../native/ipc-types";
import * as ipcStub from "../support/ipc-stub";
import * as commands from "../support/commands";

context("routing", () => {
  beforeEach(() => {
    commands.resetProxyState();
    cy.visit("./public/index.html");
  });

  context("session persistancy", () => {
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
              expect(loc.hash).to.eq("#/profile/projects");
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
