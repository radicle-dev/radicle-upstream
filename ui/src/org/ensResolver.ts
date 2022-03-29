// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { TransactionResponse } from "./contract";

import { ethers } from "ethers";
import LruCache from "lru-cache";
import { ENS__factory as EnsRegistryFactory } from "radicle-contracts/build/contract-bindings/ethers";

import * as error from "ui/src/error";
import * as svelteStore from "ui/src/svelteStore";
import * as Wallet from "ui/src/wallet";
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
  records: EnsRecord[]
): Promise<TransactionResponse> {
  const wallet = svelteStore.get(Wallet.store);

  const resolver = await wallet.provider.getResolver(domain);

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
    wallet.signer
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
  const wallet = svelteStore.get(Wallet.store);
  const resolver = await wallet.provider.getResolver(domain);

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
    (value: PromiseSettledResult<string | null>) =>
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
  const wallet = svelteStore.get(Wallet.store);
  const ensAddr = ethereum.ensAddress(wallet.environment);

  const registry = EnsRegistryFactory.connect(ensAddr, wallet.provider);
  const owner = await registry.owner(ethers.utils.namehash(domain));

  return owner;
}

interface RegistrationCacheEntry {
  value: Registration | undefined;
}

const registrationCache = new LruCache<string, RegistrationCacheEntry>({
  max: 1000,
  ttl: 10 * 60 * 1000, // 10 minutes
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
    const wallet = svelteStore.get(Wallet.store);
    const name = await wallet.provider.lookupAddress(normalisedAddress);

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
