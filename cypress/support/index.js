import "./commands";
import "./assertions";

// Stub electron preloader to always set isDev to true during tests.
Cypress.on("window:before:load", appWindow => {
  appWindow.electron = {
    isDev: true,
  };
});
