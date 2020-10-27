Cypress.Commands.add("resetProxyState", async () => {
  console.log("Reset Proxy state");
  await fetch("http://localhost:8080/v1/control/reset");
  await fetch("http://localhost:8080/v1/session/unseal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({
      passphrase: "radicle-upstream",
    }),
  });
});

Cypress.Commands.add("sealKeystore", async () => {
  await fetch("http://localhost:8080/v1/control/seal");
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
      credentials: "include",
      body: JSON.stringify({
        name,
        description,
        defaultBranch,
        fakePeers,
      }),
    })
);

Cypress.Commands.add("onboardUser", async (handle = "secretariat") => {
  await fetch("http://localhost:8080/v1/session/unseal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({
      passphrase: "radicle-upstream",
    }),
  });
  await fetch("http://localhost:8080/v1/identities", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({
      handle,
    }),
  });
});
