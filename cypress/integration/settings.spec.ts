import * as commands from "../support/commands";
import * as ipcStub from "../support/ipc-stub";
import type { CyHttpMessages } from "cypress/types/net-stubbing";
const validSeedAddress =
  "hyy5s7ysg96fqa91gbe7h38yddh4mkokft7y4htt8szt9e17sxoe3h@seed.my.org:123";

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

    it("is persisted across app start", () => {
      cy.get("button[value='light']").click();
      cy.get("[data-theme='light']").should("exist");
      commands.restartAndUnlock();
      commands.pick("sidebar", "settings").click();
      cy.get("[data-theme='light']").should("exist");
    });
  });

  context("network", () => {
    it("validates the seed input", () => {
      cy.log("checks the format");
      commands.pasteInto(["seed-input"], "invalid-seed@seed.my.org:123");
      commands.pick("add-seed").click();
      commands
        .pick("seed-entry-form")
        .should("contain", "This is not a valid seed address");
      cy.get(".seeds").find(".seed").should("have.length", 0);

      cy.log("checks for duplication");
      commands.pasteInto(["seed-input"], validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);
      // add the same seed again
      commands.pasteInto(["seed-input"], validSeedAddress);
      commands.pick("add-seed").click();
      commands
        .pick("seed-entry-form")
        .should("contain", "This seed already exists");
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("adds and removes seeds", () => {
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pasteInto(["seed-input"], validSeedAddress);

      cy.log("adds a seed via button click");
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      cy.log("persists adding a seed across app start");
      commands.restartAndUnlock();
      commands.pick("sidebar", "settings").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      cy.log("adds a seed via button click");
      commands.pasteInto(["seed-input"], `${validSeedAddress}2`);
      commands.pick("seed-input").type("{enter}");
      cy.get(".seeds").find(".seed").should("have.length", 2);

      cy.log("adds new seeds to the end of the list");
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress}2`);
      commands.pasteInto(["seed-input"], `${validSeedAddress}3`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 3);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress}3`);

      cy.log("can delete seeds and persist the lists order");
      cy.get(".seeds")
        .find(".seed")
        .eq(1)
        .within(() => {
          commands.pick("remove-seed").click();
        });
      cy.get(".seeds").find(".seed").should("have.length", 2);
      cy.get(".seeds")
        .find(".seed")
        .first()
        .should("contain", `${validSeedAddress}`);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress}3`);

      cy.log("persists the removal across app start");
      commands.restartAndUnlock();
      commands.pick("sidebar", "settings").click();
      cy.get(".seeds").find(".seed").should("have.length", 2);
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
          ["notification"],
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
          ["notification-action"],
          `Check out Version ${NEW_VERSION}`
        )
        .click();

      ipcStub.getStubs().then(stubs => {
        assert.deepEqual(stubs.openUrl.args, [["ANNOUNCEMENT_URL"]]);
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
        assert.deepEqual(stubs.openUrl.args, [["ANNOUNCEMENT_URL"]]);
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

      commands
        .pickWithContent(
          ["notification"],
          "Want to check for new versions automatically"
        )
        .within(() => {
          commands.pickWithContent(["notification-action"], "Dismiss").click();
        });
      commands.pick("settings").click();
      commands.pick("version").find("button[value='on']").click();
      commands
        .pickWithContent(["notification"], `Check out Version ${NEW_VERSION}`)
        .within(() => {
          commands.pickWithContent(["notification-action"], "Dismiss").click();
        });

      cy.tick(VERSION_NOTIFY_SILENCE_INTERVAL - 500);

      commands
        .pickWithContent(
          ["notification-action"],
          `Check out Version ${NEW_VERSION}`
        )
        .should("not.exist");

      cy.tick(VERSION_CHECK_INTERVAL);
      commands
        .pickWithContent(
          ["notification-action"],
          `Check out Version ${NEW_VERSION}`
        )
        .should("exist");
    });
  });
});
