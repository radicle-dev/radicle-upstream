import { DIALOG_SHOWOPENDIALOG, OPEN_PATH } from "../../native/ipc.js";

const withWorkspaceStub = callback => {
  cy.exec("pwd").then(result => {
    const pwd = result.stdout;
    const checkoutPath = `${pwd}/cypress/workspace/checkout`;

    // Clean up any left-overs from previous failed tests.
    cy.exec(`rm -rf ${checkoutPath}`);
    cy.exec(`mkdir ${checkoutPath}`);

    // Stub Electron native calls
    cy.window().then(appWindow => {
      appWindow.mockOpenPathCalled = false;

      appWindow.electron = {
        ipcRenderer: {
          invoke: msg => {
            if (msg === DIALOG_SHOWOPENDIALOG) {
              return checkoutPath;
            } else if (msg === OPEN_PATH) {
              appWindow.mockOpenPathCalled = true;
            }
          },
        },
      };
    });

    callback(checkoutPath);

    // Clean up the cypress workspace.
    cy.exec(`rm -rf ${checkoutPath}`);
  });
};

beforeEach(() => {
  cy.nukeAllState();
  cy.createIdentity();
  cy.createProjectWithFixture();
  cy.visit("./public/index.html");
});

context("project checkout", () => {
  context("happy path", () => {
    it("checks out the project into a working directory", () => {
      cy.pick("project-list-entry-platinum").click();
      cy.pick("checkout-modal-toggle").click();

      withWorkspaceStub(checkoutPath => {
        cy.pick("choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // prevents this spec from failing on CI.
        cy.wait(500);

        // Make sure mock is set up correctly.
        cy.window().then(appWindow => {
          expect(appWindow.mockOpenPathCalled).to.be.false;
        });

        // Perform the checkout.
        cy.pick("checkout-button").click();

        // Notification should contain the full path to the working directory.
        cy.pick("notification")
          .contains("platinum checked out to")
          .should("exist");
        cy.pick("notification")
          .contains("cypress/workspace/checkout/platinum")
          .should("exist");
        cy.pick("notification").contains("Open folder").should("exist");
        cy.pick("notification").contains("Open folder").click();

        // Make sure we do the electron call for opening the folder in the OS
        // file browser.
        cy.window().then(appWindow => {
          expect(appWindow.mockOpenPathCalled).to.be.true;
        });

        // Make sure the notification gets closed after we open the folder in
        // the OS file browser.
        cy.contains("platinum checked out to").should("not.exist");

        // Get project URN and check that the working directory has the
        // appropriate rad:// remotes set up.
        cy.pick("project-screen", "urn")
          .trigger("mouseover")
          .pick("full-urn")
          .invoke("text")
          .then(urn => {
            cy.exec(`git -C ${checkoutPath}/platinum remote -v`).then(
              result => {
                expect(result.stdout).to.equal(
                  `rad\trad://${urn}.git (fetch)\n` +
                    `rad\trad://${urn}.git (push)`
                );
              }
            );
          });
      });
    });
  });
});
