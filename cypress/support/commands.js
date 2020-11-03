Cypress.Commands.add("resetProxyState", async () => {
  console.log("Reset Proxy state");
  await fetchOk("http://localhost:8080/v1/control/reset");
  await fetchOk("http://localhost:8080/v1/keystore", {
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
  await fetchOk("http://localhost:8080/v1/control/seal");
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
    await fetchOk("http://localhost:8080/v1/control/create-project", {
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
  await fetchOk("http://localhost:8080/v1/keystore", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({
      passphrase: "radicle-upstream",
    }),
  });
  await fetchOk("http://localhost:8080/v1/identities", {
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

/**
 * Invokes `fetch` and assert that the response status code is 2xx.
 * Throws an error otherwise.
 */
async function fetchOk(url, opts) {
  const response = await fetch(url, opts);
  if (response.ok) {
    return response;
  } else {
    throw new Error(`Invalid response code ${response.status}`);
  }
}
