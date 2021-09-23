// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as ethers from "ethers";
import * as svelteStore from "ui/src/svelteStore";
import * as Wallet from "ui/src/wallet";
import * as browserStore from "ui/src/browserStore";

import { Registrar__factory as RegistrarFactory } from "radicle-contracts/build/contract-bindings/ethers";

function registrarAddress(network: ethereum.Environment): string {
  switch (network) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "ENS Registrar not available on the Local testnet",
      });
    case ethereum.Environment.Rinkeby:
      return ethereum.contractAddresses.radicleEnsRegistrar.rinkeby;
    case ethereum.Environment.Mainnet:
      return ethereum.contractAddresses.radicleEnsRegistrar.mainnet;
  }
}

function registrar() {
  const wallet = svelteStore.get(Wallet.store);
  return RegistrarFactory.connect(
    registrarAddress(wallet.environment),
    wallet.signer
  );
}

export async function isAvailable(name: string): Promise<boolean> {
  const r = registrar();
  return r.available(name);
}

export async function getFee(): Promise<ethers.BigNumber> {
  const r = registrar();
  return await r.registrationFeeRad();
}

export function deadline(): ethers.BigNumber {
  // Expire one hour from now.
  return ethers.BigNumber.from(Math.floor(Date.now() / 1000)).add(3600);
}

// Return salt as a hex string.
export function generateSalt(): string {
  return ethers.BigNumber.from(ethers.utils.randomBytes(32)).toHexString();
}

export interface Commitment {
  name: string;
  ownerAddress: string;
  salt: string;
  // Commitment tx id.
  txHash: string;
  // The number of the block the commitment transaction is included.
  block?: number;
  // The minimum number of blocks that must have passed between a commitment
  // and name registration.
  minimumCommitmentAge: number;
}

export const commitmentSchema: zod.Schema<Commitment> = zod.object({
  name: zod.string(),
  ownerAddress: zod.string(),
  salt: zod.string(),
  txHash: zod.string(),
  minimumCommitmentAge: zod.number(),
});

export const commitmentStore = browserStore.create<Commitment | null>(
  "radicle.ens.commitment",
  null,
  commitmentSchema.nullable()
);

export async function commit(
  name: string,
  salt: string,
  fee: ethers.BigNumber,
  signature: ethers.Signature,
  deadline: ethers.BigNumber
): Promise<{ tx: ethers.ContractTransaction; commitment: Commitment }> {
  const wallet = svelteStore.get(Wallet.store);
  const ownerAddr = wallet.getAddress()?.toLowerCase();
  if (!ownerAddr) {
    throw new error.Error({
      message: "Wallet not connected",
    });
  }

  const minimumCommitmentAge = (
    await registrar().minCommitmentAge()
  ).toNumber();

  const commitment = createCommitment(name, ownerAddr, salt);

  const tx = await registrar().commitWithPermit(
    commitment,
    ownerAddr,
    fee,
    deadline,
    signature.v,
    signature.r,
    signature.s
  );

  return {
    tx,
    commitment: {
      name,
      txHash: tx.hash,
      ownerAddress: ownerAddr,
      salt,
      minimumCommitmentAge,
    },
  };
}

export async function register(
  name: string,
  salt: string
): Promise<ethers.ContractTransaction> {
  const wallet = svelteStore.get(Wallet.store);

  const address = wallet.getAddress();

  if (!address) {
    throw new error.Error({
      message: "Wallet not initialized",
    });
  }

  return await registrar().register(name, address, ethers.BigNumber.from(salt));
}

export async function permitSignature(
  value: ethers.BigNumberish,
  deadline: ethers.BigNumberish
): Promise<ethers.Signature> {
  const wallet = svelteStore.get(Wallet.store);
  const spenderAddr = registrarAddress(wallet.environment);
  const owner = wallet.signer;

  const rad = Wallet.radToken.connect(wallet.signer, wallet.environment);

  const ownerAddr = (await owner.getAddress()).toLowerCase();
  const nonce = await rad.nonces(ownerAddr);
  const chainId = (await wallet.provider.getNetwork()).chainId;

  const data = {
    domain: {
      name: await rad.name(),
      chainId,
      verifyingContract: rad.address,
    },
    primaryType: "Permit",
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      Permit: [
        { name: "owner", type: "address" },
        { name: "spender", type: "address" },
        { name: "value", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ],
    },
    message: {
      owner: ownerAddr.toLowerCase(),
      spender: spenderAddr.toLowerCase(),
      value: ethers.BigNumber.from(value).toString(),
      nonce: ethers.BigNumber.from(nonce).toString(),
      deadline: ethers.BigNumber.from(deadline).toString(),
    },
  };

  const sig = await owner.signTypedData(ownerAddr, data);

  return ethers.utils.splitSignature(sig);
}

function createCommitment(
  name: string,
  ownerAddress: string,
  salt: string
): string {
  const bytes = ethers.utils.concat([
    ethers.utils.toUtf8Bytes(name),
    ownerAddress,
    salt,
  ]);

  return ethers.utils.keccak256(bytes);
}
