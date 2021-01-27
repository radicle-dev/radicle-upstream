import * as commands from "../support/commands";
import * as ipcStub from "../support/ipc-stub";
import type { CyHttpMessages } from "cypress/types/net-stubbing";

context("settings", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("public/index.html");
    commands.pick("sidebar", "settings").click();
  });

  context("theme", () => {
    it("is set to the default", () => {
      cy.get("[data-theme='dark']").should("exist");
    });

    it("can be switched to light", () => {
      cy.get("button[value='light']").click();
      cy.get("[data-theme='light']").should("exist");
    });
  });

  context("app version", () => {
    // Set `ui/src/updateChecker` for these constants.
    const VERSION_CHECK_INTERVAL = 1000;
    const VERSION_NOTIFY_SILENCE_INTERVAL = 5000;

    // Current version is hardcoded to `v1.2.3` in
    // `ipc-stub.ts`
    const NEW_VERSION = "v1.2.4";

    beforeEach(() => {
      cy.window().then(win => {
        // By default, this is set to `false` for test.
        win.localStorage.removeItem("radicle.settings.updateChecker.isEnabled");
      });
    });

    it("checks for new version only if enabled", () => {
      const request = cy.spy((req: CyHttpMessages.IncomingHttpRequest) => {
        req.reply({
          statusCode: 200,
          body: JSON.stringify({
            version: NEW_VERSION,
            announcementUrl: "ANNOUNCEMENT_URL",
          }),
        });
      });

      cy.intercept("https://releases.radicle.xyz/latest.json", request).as(
        "fetchVersion"
      );

      // Stops advancing the clock
      cy.clock(Date.now(), ["setInterval", "Date"]);

      cy.visit("public/index.html");

      commands
        .pickWithContent(
          "notification",
          "Want to check for new versions automatically?"
        )
        .contains(".action", "Go to settings")
        .click();

      commands.pick("version").find("button[value='on']").click();

      cy.wait("@fetchVersion").then(() => {
        cy.tick(VERSION_CHECK_INTERVAL + 100);
      });
      cy.wait("@fetchVersion").then(() => {
        assert.equal(request.callCount, 2);
      });

      commands
        .pickWithContent(
          "notification-action",
          `Check out Version ${NEW_VERSION}`
        )
        .click();

      ipcStub.getStubs().then(stubs => {
        assert.deepEqual(stubs.IPC_OPEN_URL.args, [["ANNOUNCEMENT_URL"]]);
      });

      commands
        .pick("version")
        .find("button[value='off']")
        .click()
        .then(() => {
          cy.tick(2 * VERSION_CHECK_INTERVAL);
          assert.equal(request.callCount, 2);
        });
    });

    it("shows button if a new version becomes available", () => {
      const versionData = {
        version: "v1.2.3",
        announcementUrl: "ANNOUNCEMENT_URL",
      };
      const request = cy.spy((req: CyHttpMessages.IncomingHttpRequest) => {
        req.reply({
          statusCode: 200,
          body: JSON.stringify(versionData),
        });
      });

      cy.intercept("https://releases.radicle.xyz/latest.json", request);

      // Stops advancing the clock
      cy.clock(Date.now());

      cy.visit("public/index.html");

      commands.pick("settings").click();
      commands.pick("version").find("button[value='on']").click();
      commands.pick("checkout-new-version").should("not.exist");
      cy.tick(VERSION_CHECK_INTERVAL + 100).then(() => {
        versionData.version = NEW_VERSION;
      });
      cy.tick(VERSION_CHECK_INTERVAL + 100);

      commands.pick("checkout-new-version").click();

      ipcStub.getStubs().then(stubs => {
        assert.deepEqual(stubs.IPC_OPEN_URL.args, [["ANNOUNCEMENT_URL"]]);
      });
    });

    it("shows notification after two weeks", () => {
      cy.intercept("https://releases.radicle.xyz/latest.json", {
        statusCode: 200,
        body: JSON.stringify({
          version: NEW_VERSION,
          announcementUrl: "ANNOUNCEMENT_URL",
        }),
      });

      cy.clock(Date.now(), ["setInterval", "Date"]);

      cy.visit("public/index.html");

      commands.pickWithContent("notification-action", "Dismiss").click();
      commands.pick("settings").click();
      commands.pick("version").find("button[value='on']").click();
      commands.pickWithContent("notification-action", "Dismiss").click();

      cy.tick(VERSION_NOTIFY_SILENCE_INTERVAL - 100);

      commands
        .pickWithContent(
          "notification-action",
          `Check out Version ${NEW_VERSION}`
        )
        .should("not.exist");

      cy.tick(VERSION_CHECK_INTERVAL);
      commands
        .pickWithContent(
          "notification-action",
          `Check out Version ${NEW_VERSION}`
        )
        .should("exist");
    });
  });
});
