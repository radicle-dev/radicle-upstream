import "./commands";
import "./assertions";
import * as ipcStub from "./ipc-stub";

export { ipcStub };

// Stub electron preloader to always enable `isDev` and `isExperimental` before executing tests.
Cypress.on("window:before:load", appWindow => {
  ipcStub.setup(appWindow);
});
