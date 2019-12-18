context("debugging", () => {
  beforeEach(() => {
    cy.visit("http://localhost:8080/");
  });

  it("shows latest commit", () => {
    cy.get(".CodeMirror-code")
      .first()
      .type("{ctrl}{a}{del}");
    cy.get(".CodeMirror-code")
      .first()
      .type(
        `{
  tree(id:{name: "bla", domain:"bla"}, revision:"master", prefix:""){
    info{
      lastCommit{
        summary
`,
        { parseSpecialCharSequences: false }
      );

    cy.wait(1);
    cy.get(".execute-button").click();
    cy.screenshot();
  });
});
