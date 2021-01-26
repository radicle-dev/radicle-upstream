import * as commands from "../support/commands";
import * as cmd from "../plugins/nodeManager/commands";
import * as path from "path";
import { ipcStub } from "../support";

context("networking", () => {
  it("replicates a project from one node to another", () => {
    commands.withTempDir(tempDirPath => {
      cmd.withTwoConnectedNodes((node1, node2) => {
        cmd.asNode(node1, () => {
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

            cmd.asNode(node2, () => {
              commands.pick("sidebar", "search").click();
              commands.pick("search-input").type(urn || "");
              commands.pick("follow-toggle").click();
              commands
                .pick(
                  "following-tab",
                  "project-list-entry-new-fancy-project.xyz"
                )
                .should("exist");
            });
          });
        });
      });
    });
  });
});
