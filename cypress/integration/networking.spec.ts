// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";

import * as commands from "cypress/support/commands";
import * as ipcStub from "cypress/support/ipc-stub";
import * as nodeManager from "cypress/support/nodeManager";

const validSeedAddress =
  "hyy5s7ysg96fqa91gbe7h38yddh4mkokft7y4htt8szt9e17sxoe3h@seed.my.org:123";

context("p2p networking", () => {
  context.skip("network status indicator", () => {
    it("reacts to network state changes", () => {
      commands.withTempDir(tempDirPath => {
        nodeManager.withTwoOnboardedNodes(
          { dataDir: tempDirPath, node1User: {}, node2User: {} },
          (node1, node2) => {
            nodeManager.asNode(node1);
            commands.pick("connection-status-offline").should("exist");
            nodeManager.connectTwoNodes(node1, node2);
            commands.pick("connection-status-online").should("exist");
            nodeManager.asNode(node2);
            commands.pick("connection-status-online").should("exist");
          }
        );
      });
    });
  });

  it.skip(
    "replicates a project from one node to another",
    {
      // Tests involving two nodes are extremely flaky
      retries: {
        runMode: 3,
        openMode: 0,
      },
    },
    () => {
      const maintainer = {
        handle: "rudolfs",
        passphrase: "1111",
      };
      const contributor = {
        handle: "abbey",
        passphrase: "2222",
      };

      commands.withTempDir(tempDirPath => {
        nodeManager.withTwoOnboardedNodes(
          {
            dataDir: tempDirPath,
            node1User: maintainer,
            node2User: contributor,
          },
          (maintainerNode, contributorNode) => {
            nodeManager.connectTwoNodes(maintainerNode, contributorNode);
            nodeManager.asNode(maintainerNode);

            const maintainerProjectsDir = path.join(
              tempDirPath,
              "maintainer-projects"
            );
            cy.exec(`mkdir -p "${maintainerProjectsDir}"`);

            const projectName = "new-fancy-project.xyz";
            cy.log("Create a project via API");
            commands.createEmptyProject(
              maintainerNode.client,
              projectName,
              maintainerProjectsDir
            );

            cy.log("refresh the UI for the project to show up");
            commands.pick("sidebar", "settings").click();
            commands.pick("sidebar", "profile").click();
            commands.pick("project-list-entry-new-fancy-project.xyz").click();

            commands
              .pickWithContent(
                ["project-screen", "header"],
                "new-fancy-project"
              )
              .should("exist");

            commands.pick("project-screen", "header", "radicleId").then(el => {
              const urn = el.attr("data");
              if (!urn) {
                throw new Error("Could not find URN");
              }

              // We’re giving the P2P node some space to avoid race
              // conditions.
              cy.wait(2000);
              nodeManager.asNode(contributorNode);

              commands.pick("sidebar", "search").click();
              commands.pasteInto(["search-input"], urn);
              commands.pick("follow-toggle").click();
            });

            cy.log("project moved out of the waiting area and is available");
            commands.pick("project-list-entry-new-fancy-project.xyz").click();

            cy.log("maintainer shows up in the peer selector");
            commands
              .pickWithContent(["peer-selector"], "rudolfs")
              .should("exist");
            commands
              .pickWithContent(["peer-selector"], "maintainer")
              .should("exist");
            commands.pick("peer-selector").click();

            cy.log("current user does not show up in the peer selector");
            commands
              .pickWithContent(["peer-dropdown-entry"], "abbey")
              .should("not.exist");

            cy.log("open maintainer's profile from the contributor's client");
            commands.pick("manage-remotes").click();
            cy.get(
              '[data-cy=followed-peers] [data-peer-handle="rudolfs"]'
            ).trigger("mouseenter");
            commands.pick("view-profile-button").click();
            commands.pick("user-profile-screen").should("exist");
            commands
              .pickWithContent(["entity-name"], "rudolfs")
              .should("exist");
            commands.pick("deviceId").then(el => {
              const peerId = el.attr("data");
              expect(peerId).equal(maintainerNode.peerId);
            });
            commands
              .pickWithContent(["project-list"], projectName)
              .should("exist");

            cy.log("add contributor remote on maintainer's node");

            // We’re giving the P2P node some space to avoid race
            // conditions.
            cy.wait(2000);
            nodeManager.asNode(maintainerNode);

            commands.pick("project-list-entry-new-fancy-project.xyz").click();

            commands.pick("peer-selector").click();
            commands.pick("manage-remotes").click();
            commands.pick("followed-peers", "peer-abbey").should("not.exist");

            commands.pasteInto(["peer-input"], contributorNode.peerId);
            commands.pick("follow-button").click();

            cy.log("remote shows up in the waiting area");
            commands
              .pickWithContent(
                ["followed-peers", "peer-abbey"],
                contributorNode.peerId.slice(-5)
              )
              .should("exist");
            commands
              .pick("followed-peers", "peer-abbey", "follow-toggle")
              .should("exist");

            cy.log("test commit replication from maintainer to contributor");

            cy.log("add a new commit to the maintainer's project working dir");
            const projectPath = path.join(maintainerProjectsDir, projectName);
            const maintainerCommitSubject =
              "Commit replication from maintainer to contributor";

            // We’re giving the P2P node some space to avoid race
            // conditions.
            cy.wait(2000);
            nodeManager.exec(
              `cd "${projectPath}"
            git commit --allow-empty -m "${maintainerCommitSubject}"
            git push rad`,
              maintainerNode
            );

            cy.log("refresh the UI for the new commit to show up");
            cy.get("body").type("{esc}");
            commands.pick("sidebar", "profile").click();

            cy.log("make sure new commit shows up in the maintainer's UI");
            commands.pick("project-list-entry-new-fancy-project.xyz").click();
            commands.pick("commits-tab").click();
            commands
              .pickWithContent(["commits-page"], maintainerCommitSubject)
              .should("exist");

            cy.log("check that the commit shows up in the contributor's UI");
            nodeManager.asNode(contributorNode);

            commands.pick("project-list-entry-new-fancy-project.xyz").click();
            commands.pick("commits-tab").click();
            commands
              .pickWithContent(["commits-page"], maintainerCommitSubject)
              .should("exist");

            cy.log("test commit replication from contributor to maintainer");

            const contributorProjectsPath = path.join(
              tempDirPath,
              "contributor-projects"
            );

            cy.exec(`mkdir -p "${contributorProjectsPath}"`);

            ipcStub.getStubs().then(stubs => {
              stubs.selectDirectory.returns(contributorProjectsPath);
            });
            commands.pick("checkout-modal-toggle").click();
            commands.pick("choose-path-button").click();
            commands.pick("checkout-button").click();

            cy.log("make sure checkout finishes writing to disk");
            commands
              .pickWithContent(
                ["notification"],
                `${projectName} checked out to`
              )
              .should("exist");

            cy.log("add a new commit to the project from contributor's node");
            const contributorCommitSubject =
              "Commit replication from contributor to maintainer";
            const forkedProjectPath = path.join(
              contributorProjectsPath,
              projectName
            );

            // We’re giving the P2P node some space to avoid race
            // conditions.
            cy.wait(2000);
            nodeManager.exec(
              `cd "${forkedProjectPath}"
            git commit --allow-empty -m "${contributorCommitSubject}"
            git push rad`,
              contributorNode
            );

            cy.log("refresh the UI for the new commit to show up");
            cy.get("body").type("{esc}");
            commands.pick("sidebar", "profile").click();

            cy.log('project moved to the "Projects" tab');
            commands.pick("project-list-entry-new-fancy-project.xyz").click();

            cy.log('"Fork" button now is called "Checkout"');
            commands
              .pickWithContent(["checkout-modal-toggle"], "Checkout")
              .should("exist");

            cy.log("contributor is pre-selected in the peer selector");
            commands
              .pickWithContent(["peer-selector"], "abbey")
              .should("exist");
            commands.pickWithContent(["peer-selector"], "you").should("exist");
            commands.pick("commits-tab").click();
            commands
              .pickWithContent(["commits-page"], contributorCommitSubject)
              .should("exist");

            cy.log("maintainer received the contributor's commit");
            nodeManager.asNode(maintainerNode);
            commands.pick("project-list-entry-new-fancy-project.xyz").click();
            commands.pick("peer-selector").click();
            commands.pickWithContent(["peer-dropdown-entry"], "abbey").click();
            commands.pick("commits-tab").click();
            commands
              .pickWithContent(["commits-page"], contributorCommitSubject)
              .should("exist");
          }
        );
      });
    }
  );

  context.skip("network", () => {
    beforeEach(() => {
      commands.resetProxyState();
      commands.onboardUser();
      cy.visit("public/index.html");
      commands.pick("sidebar", "network").click();
    });

    it("validates the seed input", () => {
      cy.log("checks the format");
      commands.pasteInto(["seed-input"], "invalid-seed@seed.my.org:123");
      commands.pick("add-seed").click();
      commands
        .pick("seed-entry-form")
        .should("contain", "This is not a valid seed address");
      cy.get(".seeds").find(".seed").should("have.length", 0);

      cy.log("checks for duplication");
      commands.pasteInto(["seed-input"], validSeedAddress);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);
      // add the same seed again
      commands.pasteInto(["seed-input"], validSeedAddress);
      commands.pick("add-seed").click();
      commands
        .pick("seed-entry-form")
        .should("contain", "This seed already exists");
      cy.get(".seeds").find(".seed").should("have.length", 1);
    });

    it("adds and removes seeds", () => {
      cy.get(".seeds").find(".seed").should("have.length", 0);
      commands.pasteInto(["seed-input"], validSeedAddress);

      cy.log("adds a seed via button click");
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      cy.log("the input is cleared after a seed is added");
      commands.pick("seed-input").should("have.value", "");

      cy.log("persists adding a seed across app start");
      commands.restartAndUnlock();
      commands.pick("sidebar", "network").click();
      cy.get(".seeds").find(".seed").should("have.length", 1);

      cy.log("adds a seed via button click");
      commands.pasteInto(["seed-input"], `${validSeedAddress}2`);
      commands.pick("seed-input").type("{enter}");
      cy.get(".seeds").find(".seed").should("have.length", 2);

      cy.log("adds new seeds to the end of the list");
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress.slice(-20)}2`);
      commands.pasteInto(["seed-input"], `${validSeedAddress}3`);
      commands.pick("add-seed").click();
      cy.get(".seeds").find(".seed").should("have.length", 3);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress.slice(-20)}3`);

      cy.log("can delete seeds and persist the lists order");
      cy.get(".seeds")
        .find(".seed")
        .eq(1)
        .within(() => {
          commands.pick("remove-seed").click();
        });
      cy.get(".seeds").find(".seed").should("have.length", 2);
      cy.get(".seeds")
        .find(".seed")
        .first()
        .should("contain", `${validSeedAddress.slice(-20)}`);
      cy.get(".seeds")
        .find(".seed")
        .last()
        .should("contain", `${validSeedAddress.slice(-20)}3`);

      cy.log("persists the removal across app start");
      commands.restartAndUnlock();
      commands.pick("sidebar", "network").click();
      cy.get(".seeds").find(".seed").should("have.length", 2);
    });
  });
});
