import "./commands";
import "./assertions";

// Stub electron preloader to always enable `isDev` and `isExperimental` before executing tests.
Cypress.on("window:before:load", appWindow => {
  appWindow.electron = {
    isDev: true,
    isExperimental: true,
  };
});
