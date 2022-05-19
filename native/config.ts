// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import path from "path";

type Environment = "development" | "production";

export interface Config {
  path?: string;
  environment: Environment;
  httpAddr: string;
  lnkHome?: string;
  testWalletMnemonic?: string;
  proxyGitSeeds: string;
}

// Process environment variables that configure Upstream.
interface ProcessEnv {
  // Environment that Upstream runs in. Used to determine configuration
  // default values.
  NODE_ENV?: string;
  LNK_HOME?: string;
  // Enables test wallet for ethereum and uses the mnemonic to create
  // the private key.
  RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC?: string;
  // Port on 127.0.0.1 to bind HTTP API to
  RADICLE_UPSTREAM_HTTP_PORT?: string;
  RADICLE_PROXY_GIT_SEEDS?: string;
  PATH?: string;
}

export const config = buildConfig(process.env as ProcessEnv);

function buildConfig(processEnv: ProcessEnv): Config {
  let environment: Environment;
  if (processEnv.NODE_ENV === "development") {
    environment = "development";
  } else {
    environment = "production";
  }

  let lnkHome: string | undefined;
  if (processEnv.LNK_HOME) {
    lnkHome = processEnv.LNK_HOME;
  } else if (environment === "development") {
    lnkHome = path.resolve(__dirname, "..", "sandbox", "lnk_home");
  }

  let httpPort: number;
  if (processEnv.RADICLE_UPSTREAM_HTTP_PORT) {
    httpPort = parseInt(processEnv.RADICLE_UPSTREAM_HTTP_PORT);
  } else if (environment === "development") {
    httpPort = 40000;
  } else {
    httpPort = 17246;
  }

  let proxyGitSeeds: string;
  if (processEnv.RADICLE_PROXY_GIT_SEEDS === undefined) {
    if (environment === "development") {
      proxyGitSeeds = "https://seed.upstream.radicle.xyz";
    } else {
      proxyGitSeeds = [
        "https://maple.radicle.garden",
        "https://pine.radicle.garden",
        "https://willow.radicle.garden",
      ].join(",");
    }
  } else {
    proxyGitSeeds = processEnv.RADICLE_PROXY_GIT_SEEDS;
  }

  const arm64HomebrewPath = "/opt/homebrew/bin";
  const x86_64HomebrewPath = "/usr/local/bin";
  const macPortsPath = "/opt/local/bin";

  let globalPath: string | undefined = undefined;

  if (processEnv.PATH !== undefined) {
    if (process.platform === "darwin") {
      globalPath = `${arm64HomebrewPath}:${x86_64HomebrewPath}:${macPortsPath}:${processEnv.PATH}`;
    } else {
      globalPath = processEnv.PATH;
    }
  }

  return {
    path: globalPath,
    environment,
    lnkHome,
    httpAddr: `127.0.0.1:${httpPort.toString()}`,
    testWalletMnemonic: processEnv.RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC,
    proxyGitSeeds,
  };
}
