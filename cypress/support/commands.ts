// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import * as ProxyClient from "proxy-client";
import { sleep } from "ui/src/sleep";
import { createPlugin } from "cypress/support/plugin";
import * as ethereumDevNodeApi from "cypress/plugins/ethereumDevNode/api";
import { retryFetch } from "ui/src/retryOnError";

const proxyClient = new ProxyClient.ProxyClient("http://127.0.0.1:30000");

export const resetProxyState = (): void => {
  cy.then(async () => {
    await proxyClient.control.reset();
    await waitSealed();
  });
};

export const sealKeystore = (): void => {
  cy.then(async () => {
    await proxyClient.control.seal();
    await waitSealed();
  });
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

const TMP_DIR_ROOT = "./cypress/workspace/test-tmp";

// Create a temporary directory in TMP_DIR_ROOT and pass it to the
// callback. The name of the temporary directory is the name of the current
// test. The directory is removed if the test passes succesfully.
export function withTempDir(callback: (tempDirPath: string) => void): void {
  const testName = getCurrentTestName();
  const tempDir = path.join(TMP_DIR_ROOT, testName);
  cy.exec(
    `set -euo pipefail
    rm -rf "${tempDir}"
    mkdir -p "${tempDir}"
    chmod a+rx "${tempDir}"`,
    { log: false }
  );
  Cypress.log({
    name: "tmp",
    message: "using temporary directory",
    consoleProps: () => ({
      tempDir,
    }),
  });
  callback(tempDir);
  cy.exec(`rm -r "${tempDir}"`);
}

// Selects one or more elements with the given `data-cy` ID that
// contain the given content.
export const pickWithContent = (
  ids: string[],
  content: string,
  options: Partial<Cypress.Timeoutable> = {}
): Cypress.Chainable<JQuery> => {
  const selectorString = ids.map(id => `[data-cy="${id}"]`).join(" ");
  return cy.contains(selectorString, content, options);
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
  defaultBranch = "master"
): void => {
  cy.then(async () => {
    await proxyClient.control.projectCreate({
      name,
      description,
      defaultBranch,
    });
  });
};

export function createEmptyProject(
  client: ProxyClient.ProxyClient,
  name: string,
  path: string
): Cypress.Chainable<string> {
  return cy.then(async () => {
    const project = await client.project.create({
      repo: {
        type: "new",
        path,
        name,
      },
      description: "This is the description.",
      defaultBranch: "main",
    });
    return project.urn;
  });
}

export const onboardUser = (
  handle = "secretariat"
): Cypress.Chainable<void> => {
  return cy.then(async () => {
    await proxyClient.keyStoreCreate({ passphrase: "radicle-upstream" });
    await retryFetch(() => proxyClient.identity.create({ handle }), 10, 300);
  });
};

export const metaKey = (): string => {
  return navigator.platform.includes("Mac") ? "meta" : "ctrl";
};

export const ethereumDevNode = createPlugin<ethereumDevNodeApi.Plugin>(
  "ethereumDevNode",
  ethereumDevNodeApi.methods
);

function getCurrentTestName() {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let test = (Cypress as any).mocha.getRunner().suite.ctx.test;
  let testTitles = [];
  while (test) {
    if (test.title) {
      testTitles.push(test.title);
    }
    test = test.parent;
  }

  testTitles = testTitles.reverse();
  return testTitles.join(" -- ");
}

// Wait until the proxy has been re-sealed or reset.
async function waitSealed() {
  let remainingTries = 500;
  for (;;) {
    remainingTries -= 1;
    if (remainingTries < 0) {
      throw new Error("Waiting for unsealed timed out");
    }

    try {
      await proxyClient.sessionGet();
      await sleep(10);
    } catch (err: unknown) {
      if (
        err instanceof ProxyClient.ResponseError &&
        [404, 403].includes(err.response.status)
      ) {
        return;
      }
    }
  }
}
