import * as uuid from "uuid";
import * as path from "path";

import * as proxy from "../../ui/src/proxy";

const proxyClient = new proxy.Client("http://localhost:17246");

export const resetProxyState = (): void => {
  cy.then(() => proxyClient.control.reset());
};

export const sealKeystore = (): void => {
  cy.then(() => proxyClient.control.seal());
};

export const restartAndUnlock = (): void => {
  sealKeystore();
  cy.visit("./public/index.html");
  pick("passphrase-input").type("radicle-upstream");
  pick("unlock-button").click();
};

export const pick = (...ids: string[]): Cypress.Chainable<JQuery> => {
  const selectorString = ids.map(id => `[data-cy="${id}"]`).join(" ");
  return cy.get(selectorString);
};

// A directory that can be used for temporary test data.
//
// It is located within this repository so that there is no extra setup
// necessary when using it locally or on CI. To avoid committing any left-over
// temp data this directory ignored via .gitignore.
const CYPRESS_WORKSPACE_PATH = path.join(__dirname, "../workspace");

export const withTempDir = (callback: (tempDirPath: string) => void): void => {
  const tempDirPath = path.join(CYPRESS_WORKSPACE_PATH, uuid.v4());
  cy.exec(`mkdir -p ${tempDirPath}`);

  callback(tempDirPath);

  cy.exec(`rm -rf ${tempDirPath}`);
};

// Selects one or more elements with the given `data-cy` ID that
// contain the given content.
export const pickWithContent = (
  ids: string[],
  content: string
): Cypress.Chainable<JQuery> => {
  const selectorString = ids.map(id => `[data-cy="${id}"]`).join(" ");
  return cy.contains(selectorString, content);
};

// Selects the input element with the given `data-cy` ID and pastes
// the value inside
export const pasteInto = (ids: string[], value: string): void => {
  pick(...ids)
    .invoke("val", value)
    .trigger("input");
};

export const createProjectWithFixture = (
  name = "platinum",
  description = "Best project ever.",
  defaultBranch = "master",
  fakePeers: string[] = []
): void => {
  cy.then(() => {
    proxyClient.control.projectCreate({
      name,
      description,
      defaultBranch,
      fakePeers,
    });
  });
};

export const onboardUser = (
  handle = "secretariat"
): Cypress.Chainable<void> => {
  return cy.then(async () => {
    await proxyClient.keyStoreCreate({ passphrase: "radicle-upstream" });
    await proxyClient.identityCreate({ handle });
  });
};

export const metaKey = (): string => {
  return navigator.platform.includes("Mac") ? "meta" : "ctrl";
};
