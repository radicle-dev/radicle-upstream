// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";

import * as commands from "cypress/support/commands";
import * as nodeManager from "cypress/support/nodeManager";

const commitMessage = "Adding something new";
const patchName = "my-patch/fix";
const patchTitle = "Title";
const patchDescription = "Description.";

context("patches", () => {
  it("shows the empty screen if there are no patches", () => {
    commands.resetProxyState();
    commands.onboardUser();
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");
    commands.pick("project-list-entry-platinum").click();
    commands.pick("patches-tab").click();
    commands.pick("patches-tab", "counter").should("not.exist");
    commands
      .pickWithContent(
        ["empty-state"],
        "There are no patches to show at the moment. If you’re looking for someone’s patch, be sure to add their Device ID as a remote using the dropdown above."
      )
      .should("exist");
    commands
      .pickWithContent(["patch-modal-toggle"], "New Patch")
      .should("exist");
  });

  it("shows annotated patches", () => {
    commands.resetProxyState();
    commands.withTempDir(tempDirPath => {
      nodeManager.withOneOnboardedNode(
        { dataDir: tempDirPath, handle: "rudolfs" },
        node => {
          nodeManager.asNode(node);
          cy.then(() =>
            node.client.project.create({
              repo: {
                type: "new",
                path: tempDirPath,
                name: "new-project",
              },
              description: "",
              defaultBranch: "main",
            })
          );
          commands.pick("sidebar", "settings").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-project").should("exist");
          nodeManager.exec(
            `cd "${tempDirPath}/new-project"
            git checkout -b "${patchName}"
            git commit --allow-empty -m "${commitMessage}"
            upstream patch create -m "${patchTitle}\n\n${patchDescription}"`,
            node
          );
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-project").click();
          commands.pick("patches-tab").click();
          commands
            .pickWithContent(["patches-tab", "counter"], "1")
            .should("exist");

          cy.log("verifying the contents of the patch list page");
          commands
            .pick(`patch-card-${patchName}`)
            .should("contain", patchName)
            .should("contain", "Opened")
            .should("contain", "rudolfs");
          commands
            .pick(`patch-card-${patchName}`, "compare-branches")
            .should("contain", "main")
            .should("contain", patchName);

          cy.log("checking the navigation");
          commands.pick(`patch-card-title-${patchName}`).click();
          commands.pick("patch-page").should("exist");
          commands
            .pickWithContent(
              ["patch-page", "history", "commit-group", "commit"],
              commitMessage
            )
            .click();
          commands.pick("commit-page").should("exist");
          commands.pick("commit-page", "back-button").click();
          commands.pick("patch-page").should("exist");
          commands.pick("patch-page", "back-button").click();
          commands.pick("patch-list").should("exist");

          cy.log("verifying the contents of the patch page");
          commands.pick(`patch-card-title-${patchName}`).click();
          commands
            .pickWithContent(["checkout-patch-modal-toggle"], "Checkout")
            .should("exist");
          commands
            .pickWithContent(["merge-patch-modal-toggle"], "Merge")
            .should("exist");
          commands.pickWithContent(["patch-page"], patchTitle).should("exist");
          commands
            .pickWithContent(["patch-page"], patchDescription)
            .should("exist");
          commands.pickWithContent(["patch-page"], "Opened").should("exist");
          commands.pickWithContent(["patch-page"], "rudolfs").should("exist");
          commands
            .pickWithContent(["patch-page", "compare-branches"], "main")
            .should("exist");
          commands
            .pickWithContent(["patch-page", "compare-branches"], patchName)
            .should("exist");

          cy.log("verify that only the single patch commit is displayed");
          commands
            .pick("patch-page", "history", "commit-group")
            .should("have.length", 1);
          commands
            .pick("patch-page", "history", "commit-group", "commit")
            .should("have.length", 1);
          commands
            .pick("patch-page", "history", "commit-group", "commit")
            .should("contain", commitMessage);
        }
      );
    });
  });

  it("shows patches without a message", () => {
    commands.resetProxyState();
    commands.withTempDir(tempDirPath => {
      nodeManager.withOneOnboardedNode(
        { dataDir: tempDirPath, handle: "rudolfs" },
        node => {
          nodeManager.asNode(node);
          commands.createEmptyProject(node.client, "new-project", tempDirPath);
          commands.pick("sidebar", "settings").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-project").should("exist");
          nodeManager.exec(
            `cd "${tempDirPath}/new-project"
            git checkout -b "${patchName}"
            git commit --allow-empty -m "${commitMessage}"
            upstream patch create -m ""`,
            node
          );
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-project").click();
          commands.pickWithContent(["patches-tab", "counter"], "1").click();

          cy.log("verifying the contents of the patch list page");
          commands
            .pick(`patch-card-${patchName}`)
            .should("contain", patchName)
            .should("contain", "Opened")
            .should("contain", "rudolfs");
          commands
            .pick(`patch-card-${patchName}`, "compare-branches")
            .should("contain", "main")
            .should("contain", patchName);

          commands.pick(`patch-card-title-${patchName}`).click();
          commands
            .pickWithContent(["patch-page", "patch-title"], patchName)
            .should("exist");
        }
      );
    });
  });

  it("replicates a patch from contributor to maintainer", () => {
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
            .pickWithContent(["project-screen", "header"], "new-fancy-project")
            .should("exist");

          const contributorProjectsDir = path.join(
            tempDirPath,
            "contributor-projects"
          );

          commands.pick("project-screen", "header", "radicleId").then(el => {
            const urn = el.attr("data");
            if (!urn) {
              throw new Error("Could not find URN");
            }

            nodeManager.asNode(contributorNode);

            cy.log("contributor follows the project");
            cy.then(() => contributorNode.client.project.requestSubmit(urn));
            commands
              .pick("project-list-entry-new-fancy-project.xyz")
              .should("exist");

            cy.log("contributor checks out the project");
            cy.exec(`mkdir -p "${contributorProjectsDir}"`);
            cy.then(() =>
              contributorNode.client.project.checkout(urn, {
                path: contributorProjectsDir,
                peerId: maintainerNode.peerId,
              })
            );
          });

          cy.log("the project is now under the project tab");
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-fancy-project.xyz").click();
          commands.pick("patches-tab").click();

          cy.log("test patch replication from contributor to maintainer");
          cy.log("add a patch to the project from contributor's node");
          const patchCommitSubject =
            "Merge request replication from contributor to maintainer";
          const forkedProjectPath = path.join(
            contributorProjectsDir,
            projectName
          );
          const patchMessage = "This is an awesome feature";

          nodeManager.exec(
            `cd "${forkedProjectPath}"
            git checkout -b "${patchName}"
            git commit --allow-empty -m "${patchCommitSubject}"
            upstream patch create -m "${patchMessage}"`,
            contributorNode
          );

          cy.log("contributor sees the patch");
          commands
            .pickWithContent(["patch-list"], patchMessage)
            .should("exist");

          cy.log("add contributor remote on maintainer's node");
          nodeManager.asNode(maintainerNode);

          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          commands.pick("project-screen", "header", "radicleId").then(el => {
            const urn = el.attr("data");
            if (!urn) {
              throw new Error("Could not find URN");
            }

            cy.then(() =>
              maintainerNode.client.project.peerTrack(
                urn,
                contributorNode.peerId
              )
            );
          });

          cy.log("maintainer received the contributor's patch");
          commands
            .pickWithContent(["patches-tab", "counter"], "1")
            .should("exist");
          commands.pick("patches-tab").click();
          commands.pick(`patch-card-title-${patchName}`).click();

          cy.log("maintainer can see the patch details");
          commands
            .pickWithContent(["patch-page"], patchMessage)
            .should("exist");

          cy.log("maintainer receives patch updates and shows them in the UI");

          nodeManager.exec(
            `cd "${forkedProjectPath}"
            upstream patch update --message "updated patch message"`,
            contributorNode
          );

          commands
            .pickWithContent(["patch-page"], "updated patch message")
            .should("exist");

          cy.log("maintainer can navigate to the commit");
          commands
            .pickWithContent(
              ["patch-page", "history", "commit-group", "commit"],
              patchCommitSubject
            )
            .click();
          commands
            .pickWithContent(["commit-page"], patchCommitSubject)
            .should("exist");
        }
      );
    });
  });

  it("is possible to merge a patch", () => {
    const maintainer = {
      handle: "maintainer",
      passphrase: "1111",
    };
    const contributor = {
      handle: "contributor",
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
          commands
            .createEmptyProject(
              maintainerNode.client,
              projectName,
              maintainerProjectsDir
            )
            .as("projectUrn");

          cy.log("refresh the UI for the project to show up");
          commands.pick("sidebar", "settings").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-new-fancy-project.xyz").click();

          commands
            .pickWithContent(["project-screen", "header"], "new-fancy-project")
            .should("exist");

          const contributorProjectsDir = path.join(
            tempDirPath,
            "contributor-projects"
          );

          nodeManager.asNode(contributorNode);
          cy.get<string>("@projectUrn").then(urn => {
            cy.log("contributor checks out the project");
            cy.then(() => contributorNode.client.project.requestSubmit(urn));
            commands.pick(`project-list-entry-${projectName}`).should("exist");

            cy.exec(`mkdir -p "${contributorProjectsDir}"`);
            cy.then(() =>
              contributorNode.client.project.checkout(urn, {
                path: contributorProjectsDir,
                peerId: maintainerNode.peerId,
              })
            );
          });

          cy.log("maintainer tracks peer");
          nodeManager.asNode(maintainerNode);
          cy.get<string>("@projectUrn").then(urn => {
            cy.then(() =>
              maintainerNode.client.project.peerTrack(
                urn,
                contributorNode.peerId
              )
            );
          });

          commands.pick(`project-list-entry-${projectName}`).click();
          commands.pick("patches-tab").click();

          cy.log("add a patch to the project from contributor's node");
          const forkedProjectPath = path.join(
            contributorProjectsDir,
            projectName
          );

          nodeManager.exec(
            `cd "${forkedProjectPath}"
            git checkout -b "${patchName}"
            git commit --allow-empty -m "commit message"
            upstream patch create --message "patch message"`,
            contributorNode
          );

          commands.pick("patches-tab", "counter").should("contain", "1");
          commands.pickWithContent(["patch-list"], patchName).should("exist");

          cy.log("maintainer merges patch in background");
          nodeManager.asNode(contributorNode);
          commands.pick(`project-list-entry-${projectName}`).click();
          commands.pick("patches-tab").click();

          nodeManager.exec(
            `cd "${contributorProjectsDir}/${projectName}"
              upstream patch fetch "${contributorNode.peerId}/${patchName}"
              git checkout main
              git merge --ff-only "radicle-patch/${contributorNode.peerId}/${patchName}"
              git push rad`,
            maintainerNode
          );

          commands
            .pickWithContent(
              ["patch-filter-tabs", "segmented-control-option"],
              "Closed"
            )
            .click();
          commands.pick(`patch-card-${patchName}`).should("exist");
        }
      );
    });
  });

  it("refreshes the UI on local project update events", () => {
    commands.withTempDir(tempDirPath => {
      nodeManager.withOneOnboardedNode(
        {
          dataDir: tempDirPath,
          handle: "maintainer",
        },
        node => {
          nodeManager.asNode(node);

          const projectsDir = path.join(tempDirPath, "maintainer-projects");
          cy.exec(`mkdir -p "${projectsDir}"`);

          const projectName = "test-project";
          cy.log("Create a project via API");
          commands
            .createEmptyProject(node.client, projectName, projectsDir)
            .as("projectUrn");

          cy.log("refresh the UI for the project to show up");
          commands.pick("sidebar", "settings").click();
          commands.pick("sidebar", "profile").click();
          commands.pick("project-list-entry-test-project").click();

          cy.log("the patches tab counter updates when a patch is created");
          commands.pick("patches-tab", "counter").should("not.exist");

          const projectPath = path.join(projectsDir, projectName);

          nodeManager.exec(
            `cd "${projectPath}"
            git checkout -b "${patchName}"
            git commit --allow-empty -m "commit message"
            upstream patch create --message "patch message"`,
            node
          );
          commands.pick("patches-tab", "counter").should("contain", "1");

          commands.pick("patches-tab").click();
          commands
            .pick(`patch-card-title-${patchName}`)
            .should("contain", "patch message");

          cy.log(
            "updating a patch reloads the UI and shows the updated message"
          );
          nodeManager.exec(
            `cd "${projectPath}"
            upstream patch update --message "updated patch message"`,
            node
          );

          commands
            .pick(`patch-card-title-${patchName}`)
            .should("contain", "updated patch message");

          cy.log("the patches tab counter updates when a patch is merged");
          nodeManager.exec(
            `cd "${projectPath}"
              git checkout main
              git merge --ff-only "${patchName}"
              git push rad`,
            node
          );
          commands.pick("patches-tab", "counter").should("not.exist");
        }
      );
    });
  });
});
