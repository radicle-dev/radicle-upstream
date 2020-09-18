Cypress.Commands.add("resetCocoState", async () => {
  console.log("Reset CoCo state");
  await fetch("http://localhost:8080/v1/control/reset/coco");
});

Cypress.Commands.add("resetSessionState", async () => {
  console.log("Reset Session state");
  await fetch("http://localhost:8080/v1/session", { method: "DELETE" });
});

Cypress.Commands.add("resteAllState", async () => {
  console.log("Reset CoCo, Registry and session state");
  try {
    await fetch("http://localhost:8080/v1/session", { method: "DELETE" });
    await fetch("http://localhost:8080/v1/control/reset/coco");
  } catch (error) {
    console.error(error);
  }
});

Cypress.Commands.add("pick", (...ids) => {
  const selectorString = ids.map(id => `[data-cy="${id}"]`).join(" ");
  cy.get(selectorString);
});

Cypress.Commands.add(
  "createProjectWithFixture",
  async (
    name = "platinum",
    description = "Best project ever.",
    defaultBranch = "master",
    fakePeers = []
  ) =>
    await fetch("http://localhost:8080/v1/control/create-project", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name,
        description,
        defaultBranch,
        fakePeers,
      }),
    })
);

Cypress.Commands.add(
  "onboardUser",
  async (handle = "secretariat") =>
    await fetch("http://localhost:8080/v1/identities", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        handle,
      }),
    })
);
