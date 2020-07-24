context("project discovery", () => {
  before(() => {
    cy.createIdentity();
    cy.visit("public/index.html");
  });

  it("loads", () => {
    cy.pick("discovery").click();
    cy.pick("project-card").should("have.length", 1);
  });
});
