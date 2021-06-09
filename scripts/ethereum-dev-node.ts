#!/usr/bin/env -S yarn node -r ts-node/register/transpile-only

import * as fs from "fs/promises";
import execa from "execa";
import { deployAll } from "radicle-contracts";
import * as ethers from "ethers";
import waitOn from "wait-on";

const ethAccountFile = "sandbox/.local-eth-account";

main().catch(e => {
  console.error(e);
  process.exit(1);
});

async function main() {
  let devEthAccount;
  try {
    devEthAccount = (await fs.readFile(ethAccountFile, "utf-8")).trim();
  } catch (err) {
    throw new Error(
      `Failed to read address of development account from ${ethAccountFile}:\n  ${err.message}`
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

  const contracts = await deployAll(signer);

  const tokenDecimals = await contracts.rad.decimals();

  // Set the initial balance of the used erc20 token for the development account.
  await (
    await contracts.rad.transfer(
      devEthAccount,
      ethers.BigNumber.from(100).mul(
        ethers.BigNumber.from(10).pow(tokenDecimals)
      )
    )
  ).wait();

  console.log(`\nRad token deployed at ${contracts.rad.address.toLowerCase()}`);
  console.log(`ENS deployed at ${contracts.ens.address.toLowerCase()}`);
  console.log(
    `Erc20 Pool deployed at ${contracts.erc20Pool.address.toLowerCase()}`
  );
  console.log(
    `Claims contract deployed at ${contracts.claims.address.toLowerCase()}`
  );

  await ganache;
}
