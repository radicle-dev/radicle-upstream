import * as commands from "../support/commands";
import * as ipcStub from "../support/ipc-stub";
const metaKey = commands.metaKey();

context("break stuff", () => {
  it("tries forever", () => {
    cy.visit("./public/index.html");
    commands.pick("passphrase-input").type("1234");
    commands.pick("unlock-button").click();

    ipcStub.getStubs().then(stubs => {
      stubs.selectDirectory.returns("/tmp/repos");
    });

    cy.wait(1000);

    let i;
    for (i = 0; i < 15; i++) {
      cy.get("body").type(`{${metaKey}+n}`);
      commands.pick("create-project", "new-project").click();
      commands
        .pick("create-project", "choose-path-button")
        .click({ force: true });
      commands.pick("create-project", "name").type(`test${i}`);
      commands
        .pick("create-project", "create-project-button")
        .click({ force: true });
      cy.wait(1000);
    }
  });
});
