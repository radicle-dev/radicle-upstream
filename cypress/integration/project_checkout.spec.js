import { DIALOG_SHOWOPENDIALOG, OPEN_PATH } from "../../native/ipc.js";

const withWorkspaceStub = callback => {
  cy.exec("pwd").then(result => {
    const pwd = result.stdout;
    const checkoutPath = `${pwd}/cypress/workspace/checkout`;

    // Clean up any left-overs from previous failed tests.
    cy.exec(`rm -rf ${checkoutPath}`);
    cy.exec(`mkdir -p ${checkoutPath}`);

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
  cy.onboarding();
  cy.createProjectWithFixture();
  cy.visit("./public/index.html");
});

context("project checkout", () => {
  context("git remote helper setup hints", () => {
    it("shows hints on how to set up the remote helper", () => {
      // The hint is visible in the project checkout modal.
      cy.pick("project-list-entry-platinum").click();
      cy.pick("checkout-modal-toggle").click();
      cy.pick("remote-helper-hint").should("be.visible");
      cy.pick("profile").click();

      // The hint is visible in the project creation modal.
      cy.pick("new-project-button").click();
      cy.pick("remote-helper-hint").should("be.visible");

      // Dismiss the hint.
      cy.pick("close-hint-button").click();
      cy.pick("remote-helper-hint").should("not.be.visible");
      cy.pick("cancel-button").click();

      // Hint is still hidden when re-entering project creation
      cy.pick("new-project-button").click();
      cy.pick("remote-helper-hint").should("not.be.visible");
      cy.pick("cancel-button").click();

      // The hint is also hidden in the project creation modal.
      cy.pick("new-project-button").click();
      cy.pick("remote-helper-hint").should("not.be.visible");
    });
  });

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

        // Check that the working directory has the rad remote.
        cy.exec(`git -C ${checkoutPath}/platinum remote show`).then(result => {
          expect(result.stdout).to.equal(`rad`);
        });

        // Make sure we can't check out a project to the same directory twice.
        cy.pick("checkout-modal-toggle").click();

        cy.pick("choose-path-button").click();
        // Make sure UI has time to update path value from stub,
        // prevents this spec from failing on CI.
        cy.wait(500);

        // Perform the checkout.
        cy.pick("checkout-button").click();

        // Notification should contain the full path to the working directory.
        cy.pick("notification")
          .contains(
            /Checkout failed: '.*checkout\/platinum' exists and is not an empty directory/
          )
          .should("exist");
      });
    });
  });
});
