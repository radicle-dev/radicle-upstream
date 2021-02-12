import * as cmd from "../support/commands";
import * as path from "path";
import * as ipcStub from "../support/ipc-stub";
import * as nodeManager from "../support/nodeManager";

context("p2p networking", () => {
  context("network status indicator", () => {
    it(
      "reacts to network state changes",
      { defaultCommandTimeout: 8000 },
      () => {
        nodeManager.withTwoOnboardedNodes(
          { node1User: {}, node2User: {} },
          (node1, node2) => {
            nodeManager.asNode(node1);
            cmd.pick("connection-status-offline").should("exist");
            nodeManager.connectTwoNodes(node1, node2);
            cmd.pick("connection-status-online").should("exist");
            nodeManager.asNode(node2);
            cmd.pick("connection-status-online").should("exist");
          }
        );
      }
    );
  });

  it("replicates a project from one node to another", () => {
    const maintainer = {
      handle: "rudolfs",
      fullName: "Rūdolfs Ošiņš",
      passphrase: "1111",
    };
    const contributor = {
      handle: "abbey",
      fullName: "Abbey Titcomb",
      passphrase: "2222",
    };

    cmd.withTempDir(tempDirPath => {
      nodeManager.withTwoOnboardedNodes(
        {
          maintainer,
          contributor,
        },
        (maintainerNode, contributorNode) => {
          nodeManager.connectTwoNodes(maintainerNode, contributorNode);
          nodeManager.asNode(maintainerNode);

          const maintainerNodeWorkingDir = path.join(
            tempDirPath,
            "maintainerNode"
          );
          cy.exec(`mkdir -p ${maintainerNodeWorkingDir}`);

          ipcStub.getStubs().then(stubs => {
            stubs.IPC_DIALOG_SHOWOPENDIALOG.returns(maintainerNodeWorkingDir);
          });
          const projectName = "new-fancy-project.xyz";

          cmd.pick("new-project-button").click();
          cmd.pasteInto(["name"], projectName);

          cmd.pick("new-project").click();
          cmd.pick("new-project", "choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);

          cmd.pick("create-project-button").click();

          cmd
            .pickWithContent(["project-screen", "header"], "new-fancy-project")
            .should("exist");

          cmd.pick("project-screen", "header", "urn").then(el => {
            const urn = el.attr("title");
            if (!urn) {
              throw new Error("Could not find URN");
            }

            nodeManager.asNode(contributorNode);

            cy.log("navigate to the 'Following' tab");
            cmd.pick("following-tab").click();
            cmd.pick("sidebar", "search").click();
            cmd.pasteInto(["search-input"], urn);
            cmd.pick("follow-toggle").click();
          });

          cy.log("project moved out of the waiting area and is available");
          cmd
            .pick(
              "following-tab-contents",
              "project-list-entry-new-fancy-project.xyz"
            )
            .click();

          cy.log("maintainer shows up in the peer selector");
          cmd.pickWithContent(["peer-selector"], "rudolfs").should("exist");
          cmd.pickWithContent(["peer-selector"], "maintainer").should("exist");
          cmd.pick("peer-selector").click();

          cy.log("current user does not show up in the peer selector");
          cmd
            .pickWithContent(["peer-dropdown-container"], "abbey")
            .should("not.exist");

          cy.log("add contributor remote on maintainer's node");
          nodeManager.asNode(maintainerNode);

          cmd.pick("project-list-entry-new-fancy-project.xyz").click();

          cmd.pick("peer-selector").click();
          cmd.pick("manage-remotes").click();
          cmd.pick("followed-peers", "peer-abbey").should("not.exist");

          cmd.pasteInto(["peer-input"], contributorNode.peerId);
          cmd.pick("follow-button").click();

          cy.log("remote shows up in the waiting area");
          const shortenedPeerId = contributorNode.peerId.slice(0, 7);
          cmd
            .pickWithContent(["followed-peers", "peer-abbey"], shortenedPeerId)
            .should("exist");
          cmd
            .pickWithContent(
              ["followed-peers", "peer-abbey", "follow-toggle"],
              "Following"
            )
            .should("exist");

          cy.log("test commit replication from maintainer to contributor");

          cy.log("add a new commit to the maintainer's project working dir");
          const projctPath = path.join(maintainerNodeWorkingDir, projectName);
          const maintainerCommitSubject =
            "Commit replication from maintainer to contributor";

          nodeManager.createCommit({
            repositoryPath: projctPath,
            monorepoPath: maintainerNode.storagePath,
            subject: maintainerCommitSubject,
            passphrase: maintainer.passphrase,
            name: maintainer.fullName,
          });

          cy.log("refresh the UI for the new commit to show up");
          cy.get("body").type("{esc}");
          cmd.pick("sidebar", "profile").click();

          cy.log("make sure new commit shows up in the maintainer's UI");
          cmd.pick("project-list-entry-new-fancy-project.xyz").click();
          cmd.pick("commits-tab").click();
          cmd
            .pickWithContent(["commits-page"], maintainerCommitSubject)
            .should("exist");

          cy.log("check that the commit shows up in the contributor's UI");
          nodeManager.asNode(contributorNode);

          cmd.pick("following-tab").click();
          cmd.pick("project-list-entry-new-fancy-project.xyz").click();
          cmd.pick("commits-tab").click();
          cmd
            .pickWithContent(["commits-page"], maintainerCommitSubject)
            .should("exist");

          cy.log("test commit replication from contributor to maintainer");

          const contributorNodeWorkingDir = path.join(
            tempDirPath,
            "contributorNode"
          );

          cy.exec(`mkdir -p ${contributorNodeWorkingDir}`);

          ipcStub.getStubs().then(stubs => {
            stubs.IPC_DIALOG_SHOWOPENDIALOG.returns(contributorNodeWorkingDir);
          });
          cmd.pick("checkout-modal-toggle").click();
          cmd.pick("choose-path-button").click();
          cmd.pick("checkout-button").click();

          cy.log("make sure checkout finishes writing to disk");
          cmd
            .pickWithContent(["notification"], `${projectName} checked out to`)
            .should("exist");

          cy.log("add a new commit to the project from contributor's node");
          const contributorCommitSubject =
            "Commit replication from contributor to maintainer";
          const forkedProjectPath = path.join(
            contributorNodeWorkingDir,
            projectName
          );

          nodeManager.createCommit({
            repositoryPath: forkedProjectPath,
            monorepoPath: contributorNode.storagePath,
            subject: contributorCommitSubject,
            passphrase: contributor.passphrase,
            name: contributor.fullName,
          });

          cy.log("refresh the UI for the new commit to show up");
          cy.get("body").type("{esc}");
          cmd.pick("sidebar", "profile").click();

          cy.log('project moved to the "Projects" tab');
          cmd.pick("project-list-entry-new-fancy-project.xyz").click();

          cy.log('"Fork" button now is called "Checkout"');
          cmd
            .pickWithContent(["checkout-modal-toggle"], "Checkout")
            .should("exist");

          cy.log("contributor is pre-selected in the peer selector");
          cmd.pickWithContent(["peer-selector"], "abbey").should("exist");
          cmd.pickWithContent(["peer-selector"], "you").should("exist");
          cmd.pick("commits-tab").click();
          cmd
            .pickWithContent(["commits-page"], contributorCommitSubject)
            .should("exist");

          cy.log("maintainer received the contributor's commit");
          nodeManager.asNode(maintainerNode);
          cmd.pick("project-list-entry-new-fancy-project.xyz").click();
          cmd.pick("peer-selector").click();
          cmd.pickWithContent(["peer-dropdown-container"], "abbey").click();
          cmd.pick("commits-tab").click();
          cmd
            .pickWithContent(["commits-page"], contributorCommitSubject)
            .should("exist");
        }
      );
    });
  });
});
