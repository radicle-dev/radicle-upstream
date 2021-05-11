import * as commands from "../support/commands";
import * as ipcStub from "../support/ipc-stub";
import * as ipcTypes from "../../native/ipc-types";

context("deep linking", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("./public/index.html");
  });

  context("when passing in a valid URL", () => {
    it("opens the search modal and pre-fills the input field with the Radicle ID", () => {
      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: {
            url: "radicle://link/v0/rad:git:hnrkjm5z3rwae9g3n6jhyo6kzh9eup5ku5odo",
          },
        });
      });

      commands
        .pick("search-modal", "search-input")
        .should("have.value", "rad:git:hnrkjm5z3rwae9g3n6jhyo6kzh9eup5ku5odo");
      commands
        .pick("search-modal", "follow-toggle")
        .should("contain", "Follow");
    });
  });

  context("when passing in an invalid URL", () => {
    it("shows an error notification", () => {
      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: {
            url: "radicle://THIS_IS_NOT_A_VALID_URN",
          },
        });
      });

      commands
        .pick("notification")
        .should("contain", "Could not parse the provided URL");

      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: {
            url: "radicle://ethereum/v0/",
          },
        });
      });

      commands
        .pick("notification")
        .should(
          "contain",
          `The custom protocol namespace "ethereum" is not supported`
        );

      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: {
            url: "radicle://link/v1/",
          },
        });
      });

      commands
        .pick("notification")
        .should("contain", "The custom protocol version v1 is not supported");

      ipcStub.getStubs().then(stubs => {
        stubs.sendMessage({
          kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: {
            url: "radicle://link/v0/",
          },
        });
      });

      commands
        .pick("notification")
        .should("contain", "The provided URL does not contain a Radicle ID");
    });
  });
});
