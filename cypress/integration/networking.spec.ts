// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as commands from "../support/commands";
import * as path from "path";
import * as ipcStub from "../support/ipc-stub";
import * as nodeManager from "../support/nodeManager";

context("p2p networking", () => {
  context("network status indicator", () => {
    it(
      "reacts to network state changes",
      { defaultCommandTimeout: 8000 },
      () => {
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
      }
    );
  });

  it("replicates a project from one node to another", () => {
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
            projectName,
            maintainerProjectsDir,
            maintainerNode.httpPort
          );

          cy.log("refresh the UI for the project to show up");
          commands.pick("sidebar", "settings").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          commands
            .pickWithContent(["project-screen", "header"], "new-fancy-project")
            .should("exist");

          commands.pick("project-screen", "header", "urn").then(el => {
            const urn = el.attr("title");
            if (!urn) {
              throw new Error("Could not find URN");
            }

            nodeManager.asNode(contributorNode);

            cy.log("navigate to the 'Following' tab");
            commands.pick("following-tab").click();
            commands.pick("sidebar", "search").click();
            commands.pasteInto(["search-input"], urn);
            commands.pick("follow-toggle").click();
          });

          cy.log("project moved out of the waiting area and is available");
          commands
            .pick(
              "following-tab-contents",
              "project-list-entry-new-fancy-project.xyz"
            )
            .click();

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

          cy.log("add contributor remote on maintainer's node");
          nodeManager.asNode(maintainerNode);

          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          commands.pick("peer-selector").click();
          commands.pick("manage-remotes").click();
          commands.pick("followed-peers", "peer-abbey").should("not.exist");

          commands.pasteInto(["peer-input"], contributorNode.peerId);
          commands.pick("follow-button").click();

          cy.log("remote shows up in the waiting area");
          const shortenedPeerId = contributorNode.peerId.slice(0, 7);
          commands
            .pickWithContent(["followed-peers", "peer-abbey"], shortenedPeerId)
            .should("exist");
          commands
            .pick("followed-peers", "peer-abbey", "follow-toggle")
            .should("exist");

          cy.log("test commit replication from maintainer to contributor");

          cy.log("add a new commit to the maintainer's project working dir");
          const projectPath = path.join(maintainerProjectsDir, projectName);
          const maintainerCommitSubject =
            "Commit replication from maintainer to contributor";

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

          // Commit replication can take a while which leads this test to often
          // fail on CI.
          cy.wait(2000);

          cy.log("check that the commit shows up in the contributor's UI");
          nodeManager.asNode(contributorNode);

          commands.pick("following-tab").click();
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
            .pickWithContent(["notification"], `${projectName} checked out to`)
            .should("exist");

          cy.log("add a new commit to the project from contributor's node");
          const contributorCommitSubject =
            "Commit replication from contributor to maintainer";
          const forkedProjectPath = path.join(
            contributorProjectsPath,
            projectName
          );

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
          commands.pickWithContent(["peer-selector"], "abbey").should("exist");
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
  });
});
