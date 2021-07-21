// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { ethers } from "ethers";
import type { EnsResolver } from '@ethersproject/providers';
import * as svelteStore from "ui/src/svelteStore";
import * as wallet from "ui/src/wallet";
import type { TransactionResponse } from "./contract";

import {
  ENS__factory
} from "radicle-contracts/build/contract-bindings/ethers";

const walletStore = svelteStore.get(wallet.store);

const resolverAbi = [
  "function multicall(bytes[] calldata data) returns(bytes[] memory results)",
  "function setAddr(bytes32 node, address addr)",
  "function setText(bytes32 node, string calldata key, string calldata value)"
];

export type EnsRecord = { name: string; value: string };

async function setRecords(name: string, resolver: EnsResolver, records: EnsRecord[]): Promise<TransactionResponse> {
  const resolverContract = new ethers.Contract(resolver.address, resolverAbi, walletStore.signer);
  const node = ethers.utils.namehash(`${name}.radicle.eth`);

  const calls = [];
  const iface = new ethers.utils.Interface(resolverAbi);

  for (const r of records) {
    switch (r.name) {
      case "address":
        calls.push(
          iface.encodeFunctionData("setAddr", [node, r.value])
        );
        break;
      case "url":
        calls.push(
          iface.encodeFunctionData("setText", [node, r.name, r.value])
        );
        break;
      case "avatar":
        calls.push(
          iface.encodeFunctionData("setText", [node, r.name, r.value])
        );
        break;
      case "github":
        calls.push(
          iface.encodeFunctionData("setText", [node, r.name, r.value])
        );
        break;
      case "twitter":
        calls.push(
          iface.encodeFunctionData("setText", [node, `vnd.${r.name}`, r.value])
        );
        break;
      default:
        console.error(`unknown field "${r.name}"`);
    }
  }
  return resolverContract.multicall(calls);
}

export interface Registration {
  name: string;
  owner: string;
  address: string | null;
  url: string | null;
  avatar: string | null;
  twitter: string | null;
  github: string | null;
  resolver: EnsResolver;
}

async function getRegistration(name: string): Promise<Registration | null> {
  const resolver = await walletStore.provider.getResolver(name);

  if (!resolver) {
    return null;
  }

  const owner = await getOwner(name);

  const meta = await Promise.allSettled([
    resolver.getAddress(),
    resolver.getText('avatar'),
    resolver.getText('url'),
    resolver.getText('vnd.twitter'),
    resolver.getText('vnd.github'),
  ]);

  const [address, avatar, url, twitter, github] =
    meta.map((
      value: PromiseSettledResult<string>
    ) => value.status === "fulfilled" ? value.value : null);

  return {
    name,
    url,
    avatar,
    owner,
    address,
    twitter,
    github,
    resolver,
  };
}

async function getOwner(name: string): Promise<string> {
  const ensAddr = (await walletStore.provider.getNetwork()).ensAddress;

  if (!ensAddr) {
    throw new Error("Missing ENS address for network");
  }

  const registry = ENS__factory.connect(ensAddr, walletStore.signer);
  const owner = await registry.owner(ethers.utils.namehash(name));

  return owner;
}

export {
  getRegistration,
  setRecords
}
