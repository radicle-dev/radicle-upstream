// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { TransactionResponse } from "./contract";
import type * as wallet from "ui/src/wallet";

import { ethers } from "ethers";
import LruCache from "lru-cache";
import { ENS__factory as EnsRegistryFactory } from "radicle-contracts/build/contract-bindings/ethers";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";

const resolverAbi = [
  "function multicall(bytes[] calldata data) returns(bytes[] memory results)",
  "function setAddr(bytes32 node, address addr)",
  "function setText(bytes32 node, string calldata key, string calldata value)",
];

export const DOMAIN = "radicle.eth";
export type EnsRecord = { name: string; value: string };

export interface Registration {
  // The fully qualified domain name for the registration.
  domain: string;
  // Address that owns this registration
  owner: string;
  // Address record
  address: string | null;
  url: string | null;
  avatar: string | null;
  twitter: string | null;
  github: string | null;
  seedId: string | null;
  seedHost: string | null;
}

export async function setRecords(
  domain: string,
  records: EnsRecord[],
  signer: wallet.WalletConnectSigner
): Promise<TransactionResponse> {
  const resolver = await ethereum.getProvider().getResolver(domain);

  // The type definitions of `ethers` are not correct. `getResolver()`
  // can return `null`.
  //
  // See https://github.com/ethers-io/ethers.js/issues/1850
  if (!resolver) {
    throw new error.Error({
      message: "Domain is not registered",
      details: { domain },
    });
  }

  const resolverContract = new ethers.Contract(
    resolver.address,
    resolverAbi,
    signer
  );
  const node = ethers.utils.namehash(domain);

  const calls = [];
  const iface = new ethers.utils.Interface(resolverAbi);

  for (const record of records) {
    switch (record.name) {
      case "address":
        calls.push(iface.encodeFunctionData("setAddr", [node, record.value]));
        break;
      case "url":
      case "avatar":
        calls.push(
          iface.encodeFunctionData("setText", [node, record.name, record.value])
        );
        break;
      case "github":
      case "twitter":
        calls.push(
          iface.encodeFunctionData("setText", [
            node,
            `com.${record.name}`,
            record.value,
          ])
        );
        break;
      case "seedId":
        calls.push(
          iface.encodeFunctionData("setText", [
            node,
            "eth.radicle.seed.id",
            record.value,
          ])
        );
        break;
      case "seedHost":
        calls.push(
          iface.encodeFunctionData("setText", [
            node,
            "eth.radicle.seed.host",
            record.value,
          ])
        );
        break;
      default:
        throw new error.Error({
          message: `Unknown field ${record.name}`,
          details: { record },
        });
    }
  }
  return resolverContract.multicall(calls);
}

export async function getRegistration(
  domain: string
): Promise<Registration | undefined> {
  const resolver = await ethereum.getProvider().getResolver(domain);

  // The type definitions of `ethers` are not correct. `getResolver()`
  // can return `null`.
  //
  // See https://github.com/ethers-io/ethers.js/issues/1850
  if (!resolver) {
    return;
  }

  const owner = await getOwner(domain);

  const meta = await Promise.allSettled([
    resolver.getAddress(),
    resolver.getText("avatar"),
    resolver.getText("url"),
    resolver.getText("com.twitter"),
    resolver.getText("com.github"),
    resolver.getText("eth.radicle.seed.id"),
    resolver.getText("eth.radicle.seed.host"),
  ]);

  const [address, avatar, url, twitter, github, seedId, seedHost] = meta.map(
    (value: PromiseSettledResult<string>) =>
      value.status === "fulfilled" ? value.value : null
  );

  return {
    domain,
    url,
    avatar,
    owner,
    address,
    twitter,
    github,
    seedId,
    seedHost,
  };
}

async function getOwner(domain: string): Promise<string> {
  const ensAddr = ethereum.ensAddress(ethereum.getEnvironment());

  const registry = EnsRegistryFactory.connect(ensAddr, ethereum.getProvider());
  const owner = await registry.owner(ethers.utils.namehash(domain));

  return owner;
}

interface RegistrationCacheEntry {
  value: Registration | undefined;
}

const registrationCache = new LruCache<string, RegistrationCacheEntry>({
  max: 1000,
  maxAge: 10 * 60 * 1000, // TTL 10 minutes
});

export async function getCachedRegistrationByAddress(
  address: string,
  invalidateCache: boolean = false
): Promise<Registration | undefined> {
  const normalisedAddress = address.toLowerCase();
  const cached: RegistrationCacheEntry | undefined =
    registrationCache.get(normalisedAddress);

  if (!invalidateCache && cached) {
    return cached.value;
  } else {
    const name = await ethereum.getProvider().lookupAddress(normalisedAddress);

    // The type definitions of `ethers` are not correct. `lookupAddress()`
    // can return `null`.
    if (!name) {
      registrationCache.set(normalisedAddress, { value: undefined });
      return;
    }

    const registration = await getRegistration(name);
    registrationCache.set(normalisedAddress, { value: registration });

    return registration;
  }
}
