// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";

context("onboarding", () => {
  const validUser = {
    handle: "rafalca",
    passphrase: "curled",
  };

  beforeEach(() => {
    commands.resetProxyState();
    cy.visit("./public/index.html");
    commands.pick("welcome-screen").should("exist");
  });

  context("navigation", () => {
    it("is not possible to exit the flow by pressing escape key", () => {
      commands.pick("get-started-button").should("exist");
      cy.get("body").type("{esc}");
      commands.pick("get-started-button").should("exist");
    });

    it("makes sure global hotkeys are disabled during onboarding", () => {
      commands.pick("get-started-button").should("exist");
      cy.get("body").type("?");
      commands.pick("hotkey-modal").should("not.exist");
    });

    it("is possible to use the keyboard for navigation", () => {
      // Intro screen.
      cy.get("body").type("{enter}");

      // Enter name screen.
      cy.focused().type(validUser.handle);
      cy.focused().type("{enter}");

      // Enter passphrase screen.
      commands.pick("enter-name-screen").should("exist");
      cy.focused().type(validUser.passphrase);
      cy.focused().type("{enter}");
      cy.focused().type(validUser.passphrase);
      cy.focused().type("{enter}");

      // Success screen.
      commands.pick("deviceId").should("exist");

      // Land on profile screen.
      cy.get("body").type("{enter}");
      commands
        .pickWithContent(["primary-action"], "Start your first project")
        .should("exist");
    });

    it("is possible to step through the identity creation flow", () => {
      // Intro screen.
      commands.pick("get-started-button").click();

      // Enter name screen.
      commands.pick("handle-input").type(validUser.handle);
      commands.pick("next-button").click();

      // Enter passphrase screen.
      commands.pick("passphrase-input").type(validUser.passphrase);
      commands.pick("repeat-passphrase-input").type(validUser.passphrase);
      commands.pick("set-passphrase-button").click();

      // Success screen.
      commands.pick("deviceId").should("exist");

      // Land on profile screen.
      commands.pick("go-to-profile-button").click();
      commands.pick("entity-name").contains(validUser.handle);
    });

    context("when clicking the back button on the passphrase screen", () => {
      it("sends the user back to the previous screen", () => {
        commands.pick("get-started-button").click();
        commands.pick("handle-input").type(validUser.handle);
        commands.pick("next-button").click();
        commands.pick("enter-passphrase-screen").should("exist");
        commands.pick("back-button").click();
        cy.contains("what should we call you?").should("exist");
        commands.pick("handle-input").should("have.value", validUser.handle);
      });
    });
  });

  context("validations", () => {
    beforeEach(() => {
      commands.pick("get-started-button").click();
    });

    context("handle", () => {
      it("requires the user to input a handle", () => {
        commands.pick("handle-input").clear();
        commands.pick("next-button").click();
        commands
          .pick("enter-name-screen")
          .contains("You must provide a display name");
      });

      it("starts validation after at least 2 characters are input", () => {
        commands.pick("handle-input").type("@");
        commands.pick("validation-error-icon").should("not.exist");
        commands.pick("handle-input").type("@");
        commands.pick("validation-error-icon").should("be.visible");
      });

      it("prevents the user from submitting an invalid handle", () => {
        // No spaces.
        commands.pick("handle-input").type("no spaces");
        commands.pick("handle-input").should("have.value", "no-spaces");

        // No special characters.
        commands.pick("handle-input").clear();
        commands.pick("handle-input").type("bad$");
        commands
          .pick("enter-name-screen")
          .contains(
            "Your display name has unsupported characters in it. You can only " +
              "use basic letters, numbers, and the _ and - characters."
          );

        // Can't start with a dash.
        commands.pick("handle-input").clear();
        commands.pick("handle-input").type("-não");
        commands
          .pick("enter-name-screen")
          .contains(
            "Your display name should start with a letter or a number."
          );

        // Has to be at least two characters long.
        commands.pick("handle-input").clear();
        commands.pick("handle-input").type("x");
        commands
          .pick("enter-name-screen")
          .contains("Your display name should be at least 2 characters long.");

        // Has to be no more than 32 characters long.
        commands.pick("handle-input").clear();
        commands.pasteInto(["handle-input"], "x".repeat(33));
        commands
          .pick("enter-name-screen")
          .contains(
            "Your display name should not be longer than 32 characters."
          );
      });
    });

    context("passphrase", () => {
      it("prevents the user from submitting an invalid passphrase", () => {
        commands.pick("handle-input").type("cloudhead");
        commands.pick("next-button").click();

        // Only entered once.
        commands.pick("passphrase-input").type("123");
        commands.pick("set-passphrase-button").should("be.disabled");

        // Too short.
        commands.pick("repeat-passphrase-input").type("123");
        cy.contains("Passphrase must be at least 4 characters").should("exist");

        // Does not match.
        commands.pick("passphrase-input").type("4");
        commands.pick("repeat-passphrase-input").type("5");
        cy.contains("Passphrases should match").should("exist");

        // Valid passphrase.
        commands.pick("passphrase-input").clear().type("abcd");
        commands.pick("repeat-passphrase-input").clear().type("abcd");
        commands.pick("set-passphrase-button").should("not.be.disabled");
      });
    });
  });
});
