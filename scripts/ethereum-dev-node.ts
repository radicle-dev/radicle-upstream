#!/usr/bin/env -S yarn node -r ts-node/register/transpile-only

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as fs from "fs/promises";
import execa from "execa";
import * as radicleContracts from "radicle-contracts";
import * as ethers from "ethers";
import waitOn from "wait-on";
import assert from "assert";

import * as contractAddresses from "ui/src/ethereum/contractAddresses";

const ethAccountFile = "sandbox/.local-eth-account";

main().catch(e => {
  console.error(e);
  process.exit(1);
});

async function main() {
  let devEthAccount;
  try {
    devEthAccount = (await fs.readFile(ethAccountFile, "utf-8")).trim();
  } catch (err: unknown) {
    throw new Error(
      `Failed to read address of development account from ${ethAccountFile}:\n  ${
        (err as Error).message
      }`
    );
  }

  const ganache = execa(
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

  const claimsContract = await radicleContracts.deployClaims(signer);
  assert.strictEqual(
    claimsContract.address.toLowerCase(),
    contractAddresses.claims.local.toLowerCase(),
    "Locally deployed contract address does not match configured contract address. Please update `ui/src/ethereum/contractAddresses`"
  );

  const radContract = await radicleContracts.deployRadicleToken(
    signer,
    address
  );
  assert.strictEqual(
    radContract.address.toLowerCase(),
    contractAddresses.radToken.local.toLowerCase(),
    "Locally deployed contract address does not match configured contract address. Please update `ui/src/ethereum/contractAddresses`"
  );

  // Set the initial balance of the used erc20 token for the development account.
  const tokenDecimals = await radContract.decimals();
  await (
    await radContract.transfer(
      devEthAccount,
      ethers.BigNumber.from(100).mul(
        ethers.BigNumber.from(10).pow(tokenDecimals)
      )
    )
  ).wait();

  console.log();
  console.log(`Rad token deployed at ${radContract.address}`);
  console.log(`Claims contract deployed at ${claimsContract.address}`);

  await ganache;
}
