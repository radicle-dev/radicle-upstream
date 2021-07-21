// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";
import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as ethers from "ethers";
import * as multihash from "multihashes";
import * as urn from "ui/src/urn";

export type { TransactionResponse };

const orgFactoryAbi = [
  "function createOrg(address) returns (address)",
  "function createOrg(address[], uint256) returns (address)",
  "event OrgCreated(address, address)",
];

const orgAbi = [
  "function owner() view returns (address)",
  "function anchor(bytes32, uint32, bytes)",
  "function setName(string, address) returns (bytes32)",
];

function orgFactoryAddress(network: ethereum.Environment): string {
  switch (network) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Orgs not available on the Local testnet",
      });
    case ethereum.Environment.Rinkeby:
      return "0xF3D04e874D07d680e8b26332eEae5b9B1c263121";
    case ethereum.Environment.Mainnet:
      return "0xa15bEb4876F20018b6b4A4116B7560c5fcC9336e";
  }
}

export function submitCreateOrgTx(
  environment: ethereum.Environment,
  owner: string,
  signer: ethers.Signer,
  isMultiSig: boolean
): Promise<TransactionResponse> {
  const orgFactory = new ethers.Contract(
    orgFactoryAddress(environment),
    orgFactoryAbi,
    signer
  );
  if (isMultiSig) {
    return orgFactory["createOrg(address[],uint256)"]([owner], 1);
  } else {
    return orgFactory["createOrg(address)"](owner);
  }
}

export function parseOrgCreatedReceipt(receipt: TransactionReceipt): string {
  const iface = new ethers.utils.Interface(orgFactoryAbi);

  let orgAddress: string | undefined;

  receipt.logs.forEach(log => {
    try {
      const parsed = iface.parseLog(log);

      if (parsed.name === "OrgCreated") {
        orgAddress = parsed.args[0].toLowerCase();
      }
    } catch {
      // Ignore parsing errors.
    }
  });

  if (!orgAddress) {
    throw new error.Error({
      code: error.Code.OrgCreateNotFoundInInterfaceLogs,
      message: "Org not found in interface logs",
    });
  }

  return orgAddress;
}

export async function getOwner(
  orgAddress: string,
  provider: ethers.providers.Provider
): Promise<string> {
  const org = new ethers.Contract(orgAddress, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr;
}

// Returns the hex encoded data for a transaction that will anchor the project
// at the commit hash.
export async function generateAnchorProjectTxData(
  projectUrn: string,
  commitHash: string
): Promise<string> {
  const orgContract = new ethers.Contract(ethers.constants.AddressZero, orgAbi);

  const { encodedProjectUrn, encodedCommitHash } = encodeAnchorData(
    projectUrn,
    commitHash
  );

  const orgContractInstance = await orgContract.populateTransaction.anchor(
    encodedProjectUrn,
    ethers.constants.Zero,
    encodedCommitHash
  );

  const txData = orgContractInstance.data;
  if (!txData) {
    throw new error.Error({
      code: error.Code.OrgCreateCouldNotGenerateTx,
      message: "Could not generate transaction",
    });
  }
  return txData;
}

// Submits a anchoring transaction for orgs controlled directly by `signer`.
export function submitSingleSigAnchor(
  projectUrn: string,
  commitHash: string,
  orgAddress: string,
  signer: ethers.Signer
): Promise<TransactionResponse> {
  const org = new ethers.Contract(orgAddress, orgAbi, signer);
  const { encodedProjectUrn, encodedCommitHash } = encodeAnchorData(
    projectUrn,
    commitHash
  );

  return org.anchor(
    encodedProjectUrn,
    ethers.constants.Zero,
    encodedCommitHash
  );
}

function encodeAnchorData(
  projectUrn: string,
  commitHash: string
): { encodedProjectUrn: Uint8Array; encodedCommitHash: Uint8Array } {
  const encodedProjectUrn = ethers.utils.zeroPad(
    urn.parseIdentitySha1(projectUrn),
    32
  );
  const encodedCommitHash = multihash.encode(
    ethers.utils.arrayify(`0x${commitHash}`),
    "sha1"
  );

  return { encodedProjectUrn, encodedCommitHash };
}

interface AnchorData {
  projectId: string;
  commitHash: string;
}

export function parseAnchorTx(data: string): AnchorData | undefined {
  const iface = new ethers.utils.Interface(orgAbi);
  const parsedTx = iface.parseTransaction({ data });

  if (parsedTx.name === "anchor") {
    const encodedProjectUrn = parsedTx.args[0];
    const encodedCommitHash = parsedTx.args[2];

    const projectId = urn.identitySha1Urn(
      ethers.utils.arrayify(`0x${encodedProjectUrn.slice(26)}`)
    );
    const byteArray = ethers.utils.arrayify(encodedCommitHash);
    const decodedMultihash = multihash.decode(byteArray);
    const decodedCommitHash = ethers.utils
      .hexlify(decodedMultihash.digest)
      .replace(/^0x/, "");

    return { projectId: projectId, commitHash: decodedCommitHash };
  }
}

export async function updateName(
  name: string,
  orgAddress: string,
  provider: ethers.providers.Provider,
  signer: ethers.Signer,
): Promise<TransactionResponse> {
  const org = new ethers.Contract(orgAddress, orgAbi, signer);

  return org.setName(
    name,
    (await provider.getNetwork()).ensAddress,
  );
}
