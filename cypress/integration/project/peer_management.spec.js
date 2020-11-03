context("project peer management", () => {
  beforeEach(() => {
    cy.resetProxyState();
    cy.onboardUser();
    cy.createProjectWithFixture();
    cy.visit("./public/index.html");

    cy.pick("project-list-entry-platinum").click();
    cy.pick("peer-selector").click();
    cy.pick("manage-remotes").click();
  });

  it("shows our own peer", () => {
    cy.pick("followed-peers")
      .contains("li", "secretariat / platinum")
      .within(() => {
        cy.contains("maintainer").should("exist");
      });
  });

  it("allows adding a new peer follow request", () => {
    // The follow button is disabled when the input field is empty.
    cy.pick("follow-button").should("have.class", "disabled");

    cy.pick("peer-input")
      .invoke("val", "hynsejpdsftse6f9bczzf69c1im9ewanb5ajnqruq3cq19keiuzk4c")
      .trigger("input");

    cy.pick("follow-button").should("not.have.class", "disabled");

    cy.pick("follow-button").click();

    cy.pick("pending-peers")
      .contains("li", "hynsejpd…keiuzk4c")
      .within(() => {
        cy.pick("follow-toggle").contains("Following").should("exist");
        cy.pick("follow-toggle").trigger("mouseenter");
        cy.pick("follow-toggle").contains("Unfollow").should("exist");
        cy.pick("follow-toggle").trigger("mouseleave");
      });

    // Disallows adding the same peer again
    cy.pick("peer-input")
      .invoke("val", "hynsejpdsftse6f9bczzf69c1im9ewanb5ajnqruq3cq19keiuzk4c")
      .trigger("input");

    cy.pick("follow-button").click();
    cy.contains("This remote is already being followed").should("exist");

    // Clears the validation message when the input is cleared.
    cy.pick("peer-input").type("{selectall}{backspace}");
    cy.contains("This remote is already being followed").should("not.exist");

    // Disallows adding an invalid peer.
    cy.pick("peer-input").type("123");
    cy.pick("follow-button").click();
    cy.contains("This is not a valid remote").should("exist");

    // Allows deleting a peer follow request.
    cy.pick("pending-peers")
      .contains("li", "hynsejpd…keiuzk4c")
      .within(() => {
        cy.pick("follow-toggle").click();
      });

    cy.contains("li", "hynsejpd…keiuzk4c").should("not.exist");
  });
});
