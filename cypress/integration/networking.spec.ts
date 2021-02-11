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
        nodeManager.withTwoOnboardedNodes(
          { node1User: {}, node2User: {} },
          (node1, node2) => {
            nodeManager.asNode(node1);
            commands.pick("connection-status-offline").should("exist");
            nodeManager.connectTwoNodes(node1, node2);
            commands.pick("connection-status-online").should("exist");
            nodeManager.asNode(node2);
            commands.pick("connection-status-online").should("exist");
          }
        );
      }
    );
  });

  it("replicates a project from one node to another", () => {
    const node1User = { handle: "rudolfs", passphrase: "1111" };
    const node2User = { handle: "abbey", passphrase: "2222" };

    commands.withTempDir(tempDirPath => {
      nodeManager.withTwoOnboardedNodes(
        {
          node1User,
          node2User,
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
          commands.pick("new-project-button").click();
          commands.pasteInto(["name"], projectName);

          commands.pick("new-project").click();
          commands.pick("new-project", "choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);

          commands.pick("create-project-button").click();

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

          cy.log("the maintainer shows up in the peer selector");
          commands
            .pickWithContent(["peer-selector"], "rudolfs")
            .should("exist");
          commands
            .pickWithContent(["peer-selector"], "maintainer")
            .should("exist");
          commands.pick("peer-selector").click();

          cy.log("the current user does not show up in the peer selector");
          commands
            .pickWithContent(["peer-dropdown-container"], "abbey")
            .should("not.exist");

          cy.log("add contributor remote on maintainer's node");
          nodeManager.asNode(maintainerNode);

          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          commands.pick("peer-selector").click();
          commands.pick("manage-remotes").click();

          commands.pick("followed-peers", "peer-abbey").should("not.exist");

          commands.pasteInto(["peer-input"], contributorNode.peerId);
          commands.pick("follow-button").click();

          cy.log("the remote shows up in the waiting area");
          const shortenedPeerId = contributorNode.peerId.slice(0, 7);
          commands
            .pickWithContent(["followed-peers", "peer-abbey"], shortenedPeerId)
            .should("exist");
          commands
            .pickWithContent(
              ["followed-peers", "peer-abbey", "follow-toggle"],
              "Following"
            )
            .should("exist");

          cy.log("test commit replication from maintainer to contributor");

          cy.log("add a new commit to the project from maintainer's node");
          const projctPath = path.join(maintainerNodeWorkingDir, projectName);
          const credentialHelper = `'!f() { test "$1" = get && echo "password=${node1User.passphrase}"; }; f'`;
          const maintainerCommitSubject =
            "Commit replication from maintainer to contributor";

          cy.exec(
            `set -euo pipefail
             export PATH=$PWD/proxy/target/release:$PATH
             cd ${projctPath}
             git commit --allow-empty -m "${maintainerCommitSubject}"
             git -c credential.helper=${credentialHelper} push rad`,
            {
              env: {
                RAD_HOME: maintainerNode.storagePath,
              },
            }
          );

          cy.log("refresh the UI, because new commits don't show up otherwise");
          cy.get("body").type("{esc}");
          commands.pick("sidebar", "profile").click();

          cy.log("make sure new commit shows up in the UI of the maintainer");
          commands.pick("project-list-entry-new-fancy-project.xyz").click();
          commands.pick("commits-tab").click();
          commands
            .pickWithContent(["commits-page"], maintainerCommitSubject)
            .should("exist");

          cy.log("check that the commit shows up in the contributor's node");
          nodeManager.asNode(contributorNode);
          commands.pick("following-tab").click();
          commands.pick("project-list-entry-new-fancy-project.xyz").click();
          commands.pick("commits-tab").click();
          commands
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
          commands.pick("checkout-modal-toggle").click();
          commands.pick("choose-path-button").click();
          commands.pick("checkout-button").click();

          // Wait for checkout to write the files to disk.
          commands
            .pickWithContent(["notification"], `${projectName} checked out to`)
            .should("exist");

          cy.log("add a new commit to the project from contributor's node");
          const contributorCredentialHelper = `'!f() { test "$1" = get && echo "password=${node2User.passphrase}"; }; f'`;
          const contributorCommitSubject =
            "Commit replication from contributor to maintainer";

          const forkedProjectPath = path.join(
            contributorNodeWorkingDir,
            projectName
          );

          cy.exec(
            `set -euo pipefail
             export PATH=$PWD/proxy/target/release:$PATH
             cd ${forkedProjectPath}
             git commit --allow-empty -m "${contributorCommitSubject}"
             git -c credential.helper=${contributorCredentialHelper} push rad`,
            {
              env: {
                RAD_HOME: contributorNode.storagePath,
              },
            }
          );

          cy.log("refresh the UI, because new commits don't show up otherwise");
          cy.get("body").type("{esc}");
          commands.pick("sidebar", "profile").click();

          cy.log('the project moved to the "Projects"');
          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          cy.log('the "Fork" button is now called "Checkout"');
          commands
            .pickWithContent(["checkout-modal-toggle"], "Checkout")
            .should("exist");

          cy.log("the contributor is pre-selected in the peer selector");
          commands.pickWithContent(["peer-selector"], "abbey").should("exist");
          commands.pickWithContent(["peer-selector"], "you").should("exist");
          commands.pick("commits-tab").click();
          commands
            .pickWithContent(["commits-page"], contributorCommitSubject)
            .should("exist");

          cy.log("test that the maintainer received the contributor's commit");
          nodeManager.asNode(maintainerNode);
          commands.pick("project-list-entry-new-fancy-project.xyz").click();
          commands.pick("peer-selector").click();
          commands
            .pickWithContent(["peer-dropdown-container"], "abbey")
            .click();
          commands.pick("commits-tab").click();
          commands
            .pickWithContent(["commits-page"], contributorCommitSubject)
            .should("exist");
        }
      );
    });
  });
});
