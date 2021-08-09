// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { WalletConnectSigner } from "ui/src/ethereum/walletConnectSigner";

import * as ethers from "ethers";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as svelteStore from "ui/src/svelteStore";
import * as wallet from "ui/src/wallet";

import {
  Registrar__factory as RegistrarFactory,
  RadicleToken__factory as RadicleTokenFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

const walletStore = svelteStore.get(wallet.store);

function registrarAddress(network: ethereum.Environment): string {
  switch (network) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "ENS Registrar not available on the Local testnet",
      });
    case ethereum.Environment.Rinkeby:
      return "0x80b68878442b6510D768Be1bd88712710B86eAcD";
    case ethereum.Environment.Mainnet:
      return "0x37723287Ae6F34866d82EE623401f92Ec9013154";
  }
}

function registrar(environment: ethereum.Environment) {
  return RegistrarFactory.connect(
    registrarAddress(environment),
    walletStore.signer
  );
}

// TODO: Move RAD-related logic to its own file
function radTokenAddress(network: ethereum.Environment): string {
  switch (network) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "ENS Registrar not available on the Local testnet",
      });
    case ethereum.Environment.Rinkeby:
      return "0x7b6CbebC5646D996d258dcD4ca1d334B282e9948";
    case ethereum.Environment.Mainnet:
      return "0x31c8EAcBFFdD875c74b94b077895Bd78CF1E64A3";
  }
}

function radToken(environment: ethereum.Environment) {
  return RadicleTokenFactory.connect(
    radTokenAddress(environment),
    walletStore.signer
  );
}

export async function checkAvailability(
  environment: ethereum.Environment,
  name: string
): Promise<{
  available: boolean;
  fee: ethers.BigNumber;
}> {
  const r = registrar(environment);

  const [available, fee] = await Promise.all([
    r.available(name),
    r.registrationFeeRad(),
  ]);

  return {
    available,
    fee,
  };
}

export async function commit(
  environment: ethereum.Environment,
  name: string,
  salt: Uint8Array,
  fee: ethers.BigNumber
): Promise<{
  receipt: ethers.providers.TransactionReceipt;
  minAge: number;
}> {
  const signer = walletStore.signer;
  const minAge = (await registrar(environment).minCommitmentAge()).toNumber();
  const ownerAddr = walletStore.getAddress();
  const spender = registrarAddress(environment);
  const deadline = ethers.BigNumber.from(Math.floor(Date.now() / 1000)).add(
    3600
  ); // Expire one hour from now.
  const token = radToken(environment);
  const signature = await permitSignature(
    walletStore.signer,
    token,
    spender,
    fee,
    deadline
  );

  if (!ownerAddr) {
    throw new error.Error({
      message: "Wallet not initialized",
    });
  }

  const commitment = createCommitment(name, ownerAddr, salt);

  // TODO: Once upstream wallet is aware of RAD balance, check if the user has
  // enough rads before committing.
  const tx = await registrar(environment)
    .connect(signer)
    .commitWithPermit(
      commitment,
      ownerAddr.toLowerCase(),
      fee,
      deadline,
      signature.v,
      signature.r,
      signature.s
    );

  await tx.wait(1);

  return {
    receipt: await walletStore.provider.getTransactionReceipt(tx.hash),
    minAge,
  };
}

export async function register(
  environment: ethereum.Environment,
  name: string,
  salt: Uint8Array
): Promise<ethers.providers.TransactionReceipt> {
  const signer = walletStore.signer;

  const address = walletStore.getAddress();

  if (!address) {
    throw new error.Error({
      message: "Wallet not initialized",
    });
  }

  const tx = await registrar(environment)
    .connect(signer)
    .register(name, address, ethers.BigNumber.from(salt));

  await tx.wait();

  return walletStore.provider.getTransactionReceipt(tx.hash);
}

async function permitSignature(
  owner: WalletConnectSigner,
  token: ethers.Contract,
  spenderAddr: string,
  value: ethers.BigNumberish,
  deadline: ethers.BigNumberish
): Promise<ethers.Signature> {
  const ownerAddr = (await owner.getAddress()).toLowerCase();
  const nonce = await token.nonces(ownerAddr);
  const chainId = (await walletStore.provider.getNetwork()).chainId;

  const data = {
    domain: {
      name: await token.name(),
      chainId,
      verifyingContract: token.address,
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
  salt: Uint8Array
): string {
  const bytes = ethers.utils.concat([
    ethers.utils.toUtf8Bytes(name),
    ownerAddress,
    ethers.BigNumber.from(salt).toHexString(),
  ]);

  return ethers.utils.keccak256(bytes);
}
