// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type {} from "native/preload";
import qs from "qs";
import * as zod from "zod";

export const INFURA_API_KEY_RINKEBY = "de5e2a8780c04964950e73b696d1bfb1";
export const INFURA_API_KEY_MAINNET = "7a19a4bf0af84fcc86ffb693a257fad4";

// Configuration values.
export interface Config {
  // The address of the proxy in `host:port` format.
  proxyAddress: string;
  // If set, a test wallet is used to automatically sign Ethereum
  // transactions. The value is the mnemonic from which the private key
  // is derived.
  testWalletMnemonic?: string;
  isDev: boolean;
}

const partialConfigSchema: zod.Schema<Partial<Config>> = zod.object({
  proxyAddress: zod.string().optional(),
  testWalletMnemonic: zod.string().optional(),
  isDev: zod.boolean().optional(),
});

// `true` if we are running unit tests with Jest.
export const isNodeTestEnv = Boolean(
  globalThis.process && globalThis.process.env["NODE_ENV"] === "test"
);

// `true` if Upstream is tested with cypress.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const isCypressTestEnv = (globalThis as any).isCypressTestEnv === true;

// `true` if this code is run by the Cypress test driver.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const isCypressTestRunner = Boolean((globalThis as any).cy);

export const config = getConfig();

// Load the partial configuration from the query string parameters and
// populate it with the default values.
function getConfig(): Config {
  const config = loadPartialConfig();
  return {
    isDev: isNodeTestEnv || isCypressTestEnv,
    proxyAddress: isCypressTestEnv ? "127.0.0.1:30000" : "127.0.0.1:17246",
    ...config,
  };
}

function loadPartialConfig(): Partial<Config> {
  if (isNodeTestEnv) {
    return {};
  }

  const queryString = window.location.search.replace("?", "");
  const query = qs.parse(queryString);
  if (typeof query.config !== "string") {
    return {};
  }

  const configData = JSON.parse(query.config);
  const result = partialConfigSchema.safeParse(configData);
  if (result.success) {
    return result.data;
  } else {
    console.error("Failed to parse user configuration");
    console.error(result.error);
    return {};
  }
}
