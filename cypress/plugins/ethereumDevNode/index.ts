// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import execa from "execa";
import * as radicleContracts from "radicle-contracts";
import * as ethers from "ethers";
import waitOn from "wait-on";

import type { Plugin } from "./api";

let ganacheProcess: execa.ExecaChildProcess;

export const ethereumDevNodePlugin: Plugin = {
  async start(): Promise<null> {
    if (ganacheProcess) {
      ganacheProcess.kill();
    }

    ganacheProcess = execa(
      // We’re not using `yarn run` because it does not forward signals
      // properly.
      // https://github.com/yarnpkg/berry/issues/991
      "./node_modules/.bin/ganache-cli",
      [
        "--mnemonic",
        "image napkin cruise dentist name plunge crisp muscle nest floor vessel blush",
        "--defaultBalanceEther",
        "1000",
      ],
      { stdio: "inherit" }
    );
    await waitOn({
      resources: ["tcp:127.0.0.1:8545"],
      timeout: 10000,
    });

    const provider = new ethers.providers.JsonRpcProvider(
      "http://localhost:8545"
    );
    const signer = provider.getSigner(0);

    const address = await signer.getAddress();
    await radicleContracts.deployClaims(signer);
    await radicleContracts.deployRadicleToken(signer, address);

    return null;
  },

  async stop(): Promise<null> {
    if (ganacheProcess) {
      ganacheProcess.kill();
    }
    return null;
  },
};
