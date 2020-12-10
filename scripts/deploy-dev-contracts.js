#!/usr/bin/env node

const { deployAll } = require("radicle-contracts");
const ethers = require("ethers");

main().catch(e => {
  console.error(e);
  process.exit(1);
});

async function main() {
  const provider = new ethers.providers.JsonRpcProvider(
    "http://localhost:8545"
  );
  const signer = provider.getSigner(0);
  const txCount = await signer.getTransactionCount();
  if (txCount !== 0) {
    throw new Error(
      "Deployer account has non-zero transaction count. You need to reset your chain"
    );
  }

  console.log("\n### Deploying the Radicle Contracts...\n");
  const contracts = await deployAll(signer);
  console.log(`Rad token deployed at ${contracts.rad.address.toLowerCase()}`);
  console.log(`ENS deployed at ${contracts.ens.address.toLowerCase()}`);
  console.log(
    `Eth Pool deployed at ${contracts.ethPool.address.toLowerCase()}`
  );
  console.log(
    `Erc20 Pool deployed at ${contracts.erc20Pool.address.toLowerCase()}`
  );
  console.log("Done.\n");
}
