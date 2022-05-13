// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "cypress/support/commands";
import * as ipcStub from "cypress/support/ipc-stub";
import * as ipcTypes from "native/ipc-types";
import * as nodeManager from "cypress/support/nodeManager";

const patchTitle = "Title";
const patchDescription = "Description.";

context("deep linking", () => {
  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    cy.visit("./public/index.html");
  });

  context("when passing in a valid patch url", () => {
    it("navigates to patch", () => {
      commands.resetProxyState();
      commands.withTempDir(tempDirPath => {
        nodeManager.withOneOnboardedNode(
          { dataDir: tempDirPath, handle: "rudolfs" },
          node => {
            nodeManager.asNode(node);
            cy.then(() => {
              node.client.project.create({
                repo: {
                  type: "new",
                  path: tempDirPath,
                  name: "new-project",
                },
                description: "",
                defaultBranch: "main",
              });
            });
            commands.pick("sidebar", "settings").click();
            commands.pick("sidebar", "profile").click();
            commands.pick("project-list-entry-new-project").should("exist");
            nodeManager.exec(
              `cd "${tempDirPath}/new-project"
            git checkout -b my-branch
            git commit --allow-empty -m "Adding something new"
            git tag -a radicle-patch/my-patch -m "${patchTitle}\n\n${patchDescription}"
            git push --tag rad;`,
              node
            );

            commands.pick("sidebar", "profile").click();
            commands.pick("project-list-entry-new-project").click();
            commands.pick("patches-tab").click();
            commands.pick("patch-list").trigger("mouseenter");
            commands.pick("patch-list", "copy-url").click();

            // Navigate away from the patch screen.
            commands.pick("sidebar", "settings").click();

            ipcStub.getStubs().then(stubs => {
              const patchUrl = stubs.getClipboard();
              ipcStub.getStubs().then(stubs => {
                stubs.sendMessage({
                  kind: ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
                  data: {
                    url: patchUrl,
                  },
                });
              });
            });

            commands
              .pickWithContent(["patch-page"], patchTitle, { timeout: 20000 })
              .should("exist");
            commands
              .pickWithContent(["patch-page"], patchDescription)
              .should("exist");
          }
        );
      });
    });
  });

  context("when passing in a valid project url", () => {
    it("opens the search modal and pre-fills the input field with the Project ID", () => {
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
      commands.pick("search-modal", "track-toggle").should("contain", "Track");
    });
  });

  context("when passing in an invalid url", () => {
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
    });
  });
});
