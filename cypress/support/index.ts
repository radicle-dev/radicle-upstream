import "./assertions";
import * as ipcStub from "./ipc-stub";
import * as nodeManager from "./nodeManager/commands";

export { ipcStub, nodeManager };

// Stub electron preloader to always enable `isDev` and `isExperimental` before executing tests.
Cypress.on("window:before:load", appWindow => {
  ipcStub.setup(appWindow);
});
