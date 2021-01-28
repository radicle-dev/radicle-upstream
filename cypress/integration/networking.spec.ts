import * as commands from "../support/commands";
import * as path from "path";
import * as ipcStub from "../support/ipc-stub";
import * as nodeManager from "../support/nodeManager";

context("networking", () => {
  it("replicates a project from one node to another", () => {
    commands.withTempDir(tempDirPath => {
      nodeManager.withTwoConnectedNodes(
        (node1, node2) => {
          nodeManager.asNode(node1);
          const newProjectPath = path.join(tempDirPath, "node1/new-project");

          cy.exec(`mkdir -p ${newProjectPath}`);

          ipcStub.getStubs().then(stubs => {
            stubs.IPC_DIALOG_SHOWOPENDIALOG.returns(newProjectPath);
          });
          commands.pick("new-project-button").click();
          commands.pick("name").type("new-fancy-project.xyz");

          commands.pick("new-project").click();
          commands.pick("new-project", "choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // this prevents this spec from failing on CI.
          cy.wait(500);

          commands.pick("create-project-button").click();

          commands
            .pick("project-screen", "header")
            .contains("new-fancy-project")
            .should("exist");

          commands.pick("project-screen", "header", "urn").then(el => {
            const urn = el.attr("title");
            if (!urn) {
              throw "Could not find URN";
            }

            nodeManager.asNode(node2);
            cy.log("navigate to the 'Following' tab");
            commands.pick("Following").click();
            commands
              .pick("following-tab")
              .contains("Still looking...")
              .should("not.exist");

            commands.pick("sidebar", "search").click();
            commands.pick("search-input").type(urn || "");
            commands.pick("follow-toggle").click();

            cy.log("project is briefly shown in the waitingroom");
            commands
              .pick("following-tab")
              .contains(urn.replace("rad:git:", ""))
              .should("exist");

            cy.log("project moved out of the waiting area and is available");
            commands
              .pick("following-tab", "project-list-entry-new-fancy-project.xyz")
              .click();

            cy.log("the maintainer shows up in the peer selector");
            commands.pick("peer-selector").contains("rudolfs").should("exist");
            commands
              .pick("peer-selector")
              .contains("maintainer")
              .should("exist");
            commands.pick("peer-selector").click();
            cy.log("the current user does not show up in the peer selector");
            commands
              .pick("peer-dropdown-container")
              .contains("abbey")
              .should("not.exist");
          });
        },
        { node1Handle: "rudolfs", node2Handle: "abbey" }
      );
    });
  });
});
