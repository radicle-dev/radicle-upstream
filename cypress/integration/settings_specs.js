import { GET_VERSION } from "../../native/ipc";

context("settings", () => {
  beforeEach(() => {
    cy.resetProxyState();
    cy.onboardUser();

    cy.visit("public/index.html");
    // stub native call and return the version number
    cy.window().then(appWindow => {
      appWindow.electron = {
        ipcRenderer: {
          invoke: msg => {
            if (msg === GET_VERSION) {
              return "v1.2.3";
            }
          },
        },
        isDev: true,
        isExperimental: true,
      };
    });
    cy.pick("sidebar", "settings").click();
  });

  context("theme", () => {
    it("is set to the default", () => {
      cy.get("[data-theme='light']").should("exist");
    });

    it("can be switched to dark", () => {
      cy.get("button[value='dark']").click();
      cy.get("[data-theme='dark']").should("exist");
    });
  });

  context("session", () => {
    it("state can be cleared", () => {
      cy.pick("clear-session-button").click();
      cy.pick("get-started-button").should("exist");
    });
  });
});
