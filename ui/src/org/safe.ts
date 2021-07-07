// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ethers from "ethers";
import EthersSafe from "@gnosis.pm/safe-core-sdk";
import SafeServiceClient, {
  SafeMultisigTransactionResponse,
} from "@gnosis.pm/safe-service-client";

import * as Ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import type { Wallet } from "ui/src/wallet";
import type { OperationType } from "@gnosis.pm/safe-core-sdk-types";

export async function getPendingTransactions(
  ethEnv: Ethereum.Environment,
  safeAddress: string
): Promise<SafeMultisigTransactionResponse[]> {
  safeAddress = ethers.utils.getAddress(safeAddress);
  const safeServiceClient = createSafeServiceClient(ethEnv);
  const response = await safeServiceClient.getPendingTransactions(safeAddress);
  // Despite the return type the `results` field may be not set because
  // of a bug in the safe client.
  // https://github.com/gnosis/safe-core-sdk/pull/31#issuecomment-863245875
  return response.results || [];
}

export interface TransactionData {
  readonly to: string;
  readonly value: string;
  readonly data: string;
  readonly operation: OperationType;
}

export async function signAndProposeTransaction(
  wallet: Wallet,
  safeAddress: string,
  tx: TransactionData
): Promise<void> {
  // Gnosis APIs only accept checksummed addresses.
  safeAddress = ethers.utils.getAddress(safeAddress);
  tx = { ...tx, to: ethers.utils.getAddress(tx.to) };

  const safeServiceClient = createSafeServiceClient(wallet.environment);
  const estimation = await safeServiceClient.estimateSafeTransaction(
    ethers.utils.getAddress(safeAddress),
    tx
  );

  const safeSdk = await EthersSafe.create(ethers, safeAddress, wallet.signer);
  const transaction = await safeSdk.createTransaction({
    ...tx,
    safeTxGas: Number(estimation.safeTxGas),
  });
  const safeTxHash = await safeSdk.getTransactionHash(transaction);

  const signature = await safeSdk.signTransactionHash(safeTxHash);

  await safeServiceClient.proposeTransaction(
    safeAddress,
    transaction.data,
    safeTxHash,
    signature
  );
}

export function appUrl(
  ethEnv: Ethereum.Environment,
  gnosisSafeAddress: string,
  view: "transactions" | "settings"
): string {
  let domain: string;
  switch (ethEnv) {
    case Ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Gnosis Safe links are not supported on the Local testnet",
      });
    case Ethereum.Environment.Rinkeby:
      domain = `rinkeby.gnosis-safe.io`;
      break;
    case Ethereum.Environment.Mainnet:
      domain = `gnosis-safe.io`;
      break;
  }
  return `https://${domain}/app/#/safes/${gnosisSafeAddress}/${view}`;
}

function createSafeServiceClient(
  ethEnv: Ethereum.Environment
): SafeServiceClient {
  let uri: string;
  switch (ethEnv) {
    case Ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Orgs are not available in the Local environment.",
      });
    case Ethereum.Environment.Rinkeby:
      uri = "https://safe-transaction.rinkeby.gnosis.io";
      break;
    case Ethereum.Environment.Mainnet:
      uri = "https://safe-transaction.gnosis.io";
      break;
  }

  return new SafeServiceClient(uri);
}
