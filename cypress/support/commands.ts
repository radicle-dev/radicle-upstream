// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as proxy from "ui/src/proxy";
import { sleep } from "ui/src/sleep";
import { createPlugin } from "cypress/support/plugin";
import * as ethereumDevNodeApi from "cypress/plugins/ethereumDevNode/api";

const proxyClient = new proxy.Client("http://127.0.0.1:17246");

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
// callback. The name of the temporary directory is based on the
// current test name
export const withTempDir = (callback: (tempDirPath: string) => void): void => {
  const testName = getCurrentTestName();
  cy.exec(
    `set -euo pipefail
    mkdir -p "${TMP_DIR_ROOT}"
    temp_dir=$(mktemp -d "${TMP_DIR_ROOT}/${testName}.XXXX")
    chmod a+rx "$temp_dir"
    echo "$temp_dir"`,
    { log: false }
  ).then(({ stdout }) => {
    const path = stdout.trim();
    Cypress.log({
      name: "tmp",
      message: "using temporary directory",
      consoleProps: () => ({
        path,
      }),
    });
    callback(path);
    cy.exec(`rm -r "${path}"`);
  });
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

export const createEmptyProject = (
  name: string = "new-project",
  path: string,
  port: number = 17246
): Cypress.Chainable<string> =>
  requestOk({
    url: `http://localhost:${port}/v1/projects`,
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      repo: {
        type: "new",
        path,
        name,
      },
      description: "This is the description.",
      defaultBranch: "main",
    }),
  }).then(response => response.urn as string);

export const followProject = (
  urn: string,
  port: number = 17246
): Cypress.Chainable<void> =>
  requestOk({
    url: `http://localhost:${port}/v1/projects/requests/${urn}`,
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
  });

export const checkoutProject = (
  urn: string,
  path: string,
  peerId: string,
  localhost: number = 17246
): Cypress.Chainable<void> =>
  requestOk({
    url: `http://localhost:${localhost}/v1/projects/${urn}/checkout`,
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      path,
      peerId,
    }),
  });

export const trackPeer = (
  urn: string,
  peerId: string,
  localhost: number = 17246
): Cypress.Chainable<void> =>
  requestOk({
    url: `http://localhost:${localhost}/v1/projects/${urn}/track/${peerId}`,
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
  });

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

export const ethereumDevNode = createPlugin<ethereumDevNodeApi.Plugin>(
  "ethereumDevNode",
  ethereumDevNodeApi.methods
);

/**
 * Invokes `cy.request` and assert that the response status code is 2xx.
 */
function requestOk(
  opts: Partial<Cypress.RequestOptions> & { url: string }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): Cypress.Chainable<any> {
  return cy.request(opts).then(response => {
    expect(response.status).to.be.within(200, 299, "Failed response");
    return response.body;
  });
}

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
  let remainingTries = 100;
  for (;;) {
    remainingTries -= 1;
    if (remainingTries < 0) {
      throw new Error("Waiting for unsealed timed out");
    }

    try {
      await proxyClient.sessionGet();
      await sleep(10);
    } catch (err) {
      if (
        err instanceof proxy.ResponseError &&
        [404, 403].includes(err.response.status)
      ) {
        return;
      }
    }
  }
}
