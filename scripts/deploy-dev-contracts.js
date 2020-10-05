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
  const contracts = await deployAll(provider, signer);
  console.log(`Rad token deployed at ${contracts.rad.address.toLowerCase()}`);
  console.log(`ENS deployed at ${contracts.ens.address.toLowerCase()}`);
}
