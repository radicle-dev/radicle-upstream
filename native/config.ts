// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import path from "path";

type Environment = "development" | "production";

export interface Config {
  environment: Environment;
  httpAddr: string;
  radHome?: string;
  testWalletMnemonic?: string;
}

// Process environment variables that configure Upstream.
interface ProcessEnv {
  // Environment that Upstream runs in. Used to determine configuration
  // default values.
  NODE_ENV?: string;
  RAD_HOME?: string;
  // Enables test wallet for ethereum and uses the mnemonic to create
  // the private key.
  RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC?: string;
  // Port on 127.0.0.1 to bind HTTP API to
  RADICLE_UPSTREAM_HTTP_PORT?: string;
}

export const config = buildConfig(process.env as ProcessEnv);

function buildConfig(processEnv: ProcessEnv): Config {
  let environment: Environment;
  if (processEnv.NODE_ENV === "development") {
    environment = "development";
  } else {
    environment = "production";
  }

  let radHome: string | undefined;
  if (processEnv.RAD_HOME) {
    radHome = processEnv.RAD_HOME;
  } else if (environment === "development") {
    radHome = path.resolve(__dirname, "..", "sandbox", "rad_home");
  }

  let httpPort: number;
  if (processEnv.RADICLE_UPSTREAM_HTTP_PORT) {
    httpPort = parseInt(processEnv.RADICLE_UPSTREAM_HTTP_PORT);
  } else if (environment === "development") {
    httpPort = 40000;
  } else {
    httpPort = 17246;
  }

  return {
    environment,
    radHome,
    httpAddr: `127.0.0.1:${httpPort.toString()}`,
    testWalletMnemonic: processEnv.RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC,
  };
}
