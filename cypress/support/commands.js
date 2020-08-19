Cypress.Commands.add("resetCache", async () => {
  console.log("Resetting Cache");
  await fetch("http://localhost:8080/v1/session/cache", { method: "DELETE" });
});

Cypress.Commands.add("resetSession", async () => {
  console.log("Resetting Session state");
  await fetch("http://localhost:8080/v1/session", { method: "DELETE" });
});

Cypress.Commands.add("resetAllState", async () => {
  console.log("Resetting Cache, CoCo, Registry and Session state");
  try {
    await fetch("http://localhost:8080/v1/session/cache", { method: "DELETE" });
    await fetch("http://localhost:8080/v1/session", { method: "DELETE" });
    await fetch("http://localhost:8080/v1/control/reset");
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
  "registerOrg",
  async (id = "monadic", transactionFee = 111) =>
    await fetch("http://localhost:8080/v1/orgs", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id,
        transactionFee,
      }),
    })
);

Cypress.Commands.add(
  "registerUser",
  async (handle = "nope", id = "123abcd.git", transactionFee = 222) =>
    await fetch("http://localhost:8080/v1/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        handle,
        id,
        transactionFee,
      }),
    })
);

Cypress.Commands.add(
  "registerAlternativeUser",
  async (handle = "anotherUser", transactionFee = 333) =>
    await fetch("http://localhost:8080/v1/control/register-user", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        handle,
        transactionFee,
      }),
    })
);

Cypress.Commands.add(
  "createIdentity",
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
