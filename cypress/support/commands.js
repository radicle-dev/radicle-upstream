Cypress.Commands.add("nukeCocoState", () => {
  console.log("Nuking CoCo state");
  fetch("http://localhost:8080/v1/control/nuke/coco");
});

Cypress.Commands.add("nukeRegistryState", () => {
  console.log("Nuking Registry state");
  fetch("http://localhost:8080/v1/control/nuke/registry");
});

Cypress.Commands.add("nukeSessionState", () => {
  console.log("Nuking Session state");
  fetch("http://localhost:8080/v1/session", { method: "DELETE" });
});

Cypress.Commands.add("nukeAllState", () => {
  console.log("Nuking CoCo and Registry state");
  fetch("http://localhost:8080/v1/control/nuke/coco");
  fetch("http://localhost:8080/v1/control/nuke/registry");
  fetch("http://localhost:8080/v1/control/nuke/session");
});

Cypress.Commands.add("select", (...ids) => {
  const selectorString = ids.map((id) => `[data-cy="${id}"]`).join(" ");
  cy.get(selectorString);
});

Cypress.Commands.add(
  "createProjectWithFixture",
  (
    name = "Monadic",
    description = "Monadic is currently supporting radicle.",
    defaultBranch = "master"
  ) =>
    fetch("http://localhost:8080/v1/control/create-project", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name,
        description,
        defaultBranch,
      }),
    })
);

Cypress.Commands.add("registerOrg", (id = "monadic") =>
  fetch("http://localhost:8080/v1/orgs", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id,
    }),
  })
);

Cypress.Commands.add("registerUser", (handle = "nope", id = "123abcd.git") =>
  fetch("http://localhost:8080/v1/users", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      handle,
      id,
    }),
  })
);

Cypress.Commands.add(
  "createIdentity",
  (
    handle = "secretariat",
    displayName = "Christopher Chenery",
    avatarUrl = null
  ) =>
    fetch("http://localhost:8080/v1/identities", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        handle,
        displayName,
        avatarUrl,
      }),
    })
);
