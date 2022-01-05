// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ethers from "ethers";
import EthersSafe, { EthersAdapter } from "@gnosis.pm/safe-core-sdk";
import type { OperationType } from "@gnosis.pm/safe-core-sdk-types";
import SafeServiceClient, {
  SafeMultisigTransactionResponse,
} from "@gnosis.pm/safe-service-client";

import * as Ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import type { Wallet } from "ui/src/wallet";
import { retryOnError } from "ui/src/retryOnError";

export async function waitUntilSafeIsReady(
  safeAddress: string,
  ethEnv: Ethereum.Environment
): Promise<void> {
  safeAddress = ethers.utils.getAddress(safeAddress);
  const safeServiceClient = createSafeServiceClient(ethEnv);
  try {
    // retry for 30 mins
    await retryOnError(
      () => safeServiceClient.getSafeInfo(safeAddress),
      () => true,
      5000,
      360
    );
  } catch (err: unknown) {
    throw new error.Error({
      message: "Cannot fetch Gnosis Safe info",
      details: { safeAddress },
      source: err,
    });
  }
}

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

  const ethAdapter = new EthersAdapter({
    ethers,
    signer: wallet.signer,
  });

  const safeSdk = await EthersSafe.create({
    ethAdapter,
    safeAddress,
  });

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
  view: "transactions/queue" | "settings/owners"
): string {
  let network: string;
  switch (ethEnv) {
    case Ethereum.Environment.Local:
      throw new error.Error({
        message: "appUrl() is not implemented for ethereum.Environment.Local",
      });
    case Ethereum.Environment.Rinkeby:
      network = "rin";
      break;
    case Ethereum.Environment.Mainnet:
      network = "eth";
      break;
  }
  return `https://gnosis-safe.io/app/${network}:${gnosisSafeAddress}/${view}`;
}

function createSafeServiceClient(
  ethEnv: Ethereum.Environment
): SafeServiceClient {
  let uri: string;
  switch (ethEnv) {
    case Ethereum.Environment.Local:
      throw new error.Error({
        message:
          "createSafeServiceClient() is not implemented for ethereum.Environment.Local",
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
