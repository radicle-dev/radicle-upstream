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
      commands.pick("seed-input").type("invalid-seed@seed.my.org:123");
      commands.pick("add-seed").click();
      cy.get(".seed-entry-form").contains("This is not a valid seed address");
      cy.get(".seeds").find(".seed").should("have.length", 0);
    });

    it("can add a seed via button", () => {
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("can add a seed via enter", () => {
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("seed-input").type("{enter}");
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("persists adding a seed across app start", () => {
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);
      commands.restartAndUnlock();
      commands.pick("sidebar", "settings").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("prevents adding the same seed", () => {
      // add a seed
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      // add the same seed again
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seed-entry-form").contains("This seed already exists");
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("adds new seeds to the end of the list", () => {
      commands.pick("seed-input").type(`${validSeedAddress  }1`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      // add a second seed
      commands.pick("seed-input").type(`${validSeedAddress  }2`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 2);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .contains(`${validSeedAddress  }2`);

      // add a third seed
      commands.pick("seed-input").type(`${validSeedAddress  }3`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 3);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .contains(`${validSeedAddress  }3`);
    });

    it("can delete a seed and persists the change across app start", () => {
      // add a seed
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pick("seed-input").type(validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      // remove the seed
      commands.pick("remove-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 0);

      // the change is persisted across app start
      commands.restartAndUnlock();
      commands.pick("sidebar", "settings").click();
      cy.get(".seeds").find(".seed").should("have.length", 0);
    });

    it("persists the order of the seed list when removing one", () => {
      // add 3 seeds
      commands.pick("seed-input").type(`${validSeedAddress  }1`);
      commands.pick("add-seed").click();
      commands.pick("seed-input").type(`${validSeedAddress  }2`);
      commands.pick("add-seed").click();
      commands.pick("seed-input").type(`${validSeedAddress  }3`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 3);

      // remove the second seed
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
        .contains(`${validSeedAddress  }1`);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .contains(`${validSeedAddress  }3`);
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

      commands.pickWithContent(["notification-action"], "Dismiss").click();
      commands.pick("settings").click();
      commands.pick("version").find("button[value='on']").click();
      commands.pickWithContent(["notification-action"], "Dismiss").click();

      cy.tick(VERSION_NOTIFY_SILENCE_INTERVAL - 100);

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
