export const resetProxyState = (): Cypress.Chainable<void> =>
  requestOk({ url: "http://localhost:17246/v1/control/reset" });

export const sealKeystore = (): Cypress.Chainable<void> =>
  requestOk({ url: "http://localhost:17246/v1/control/seal" });

export const pick = (...ids: string[]): Cypress.Chainable<JQuery> => {
  const selectorString = ids.map(id => `[data-cy="${id}"]`).join(" ");
  return cy.get(selectorString);
};

// Selects one or more elements with the given `data-cy` ID that
// contain the given content.
export const pickWithContent = (
  id: string,
  content: string
): Cypress.Chainable<JQuery> => {
  return cy.contains(`[data-cy="${id}"]`, content);
};

export const createProjectWithFixture = (
  name = "platinum",
  description = "Best project ever.",
  defaultBranch = "master",
  fakePeers = []
): Cypress.Chainable<void> =>
  requestOk({
    url: "http://localhost:17246/v1/control/create-project",
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
  });

export const onboardUser = (
  handle = "secretariat"
): Cypress.Chainable<void> => {
  requestOk({
    url: "http://localhost:17246/v1/keystore",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      passphrase: "radicle-upstream",
    }),
  });
  return requestOk({
    url: "http://localhost:17246/v1/identities",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      handle,
    }),
  });
};

/**
 * Invokes `cy.request` and assert that the response status code is 2xx.
 */
function requestOk(
  opts: Partial<Cypress.RequestOptions> & { url: string }
): Cypress.Chainable<void> {
  return cy.request(opts).then(response => {
    expect(response.status).to.be.within(200, 299, "Failed response");
    return undefined;
  });
}
