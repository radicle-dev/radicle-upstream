import "./assertions";
import * as ipcStub from "./ipc-stub";
import * as nodeManager from "./nodeManager";

export { ipcStub, nodeManager };

// Prepare the application `window` instance for cypress test.
Cypress.on("window:before:load", appWindow => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (appWindow as any).isCypressTestEnv = true;

  // Stub electron preloader to always enable `isDev` and `isExperimental` before executing tests.
  ipcStub.setup(appWindow);
});

// Common setup for all tests.
beforeEach(() => {
  cy.window().then(win => {
    win.localStorage.setItem(
      "radicle.settings.updateChecker.isEnabled",
      "false"
    );
  });
});
