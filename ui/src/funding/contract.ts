// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { BigNumber, ContractTransaction, Signer } from "ethers";

import Big from "big.js";
import * as error from "ui/src/error";

import {
  Erc20Pool,
  Erc20Pool__factory as PoolFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

import type { TypedEvent } from "radicle-contracts/build/contract-bindings/ethers/commons";

import * as ethereum from "../ethereum";

const addresses = {
  local: "0x801Db725a6B32DC3C9917CDB4CD0138F0C5907E7",
  rinkeby: "0x8c6E1E293346cc4cD31A1972D94DaDcecEd98997",
};

interface SenderUpdatedArgs {
  sender: string;
  balance: BigNumber;
  amtPerSec: BigNumber;
}
type SenderUpdatedEvent = TypedEvent<SenderUpdatedArgs & Array<unknown>>;

interface CollectedArgs {
  receiver: string;
  amt: BigNumber;
}
type CollectedEvent = TypedEvent<CollectedArgs & Array<unknown>>;

interface SenderToReceiverUpdatedArgs {
  sender: string;
  receiver: string;
  amtPerSec: BigNumber;
  endTime: BigNumber;
}
type SenderToReceiverUpdatedEvent = TypedEvent<
  SenderToReceiverUpdatedArgs & Array<unknown>
>;

// Get the address of the Pool Contract for the given environment
export function poolAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.local;
    case ethereum.Environment.Rinkeby:
      return addresses.rinkeby;
    case ethereum.Environment.Mainnet:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Token streaming contracts are not yet deployed on mainnet",
      });
  }
}

export function pool(signer: Signer, address: string): PoolContract {
  return new PoolContract(signer, address);
}

// PoolContract is a wrapper type around the actual contract, `Erc20Pool`,
// that offers a more ergonomic API around the contract.
export class PoolContract {
  contract: Erc20Pool;
  WITHDRAW_ALL = BigNumber.from(1).shl(128).sub(1);
  AMT_PER_SEC_UNCHANGED = BigNumber.from(1).shl(128).sub(1);

  constructor(signer: Signer, address: string) {
    this.contract = PoolFactory.connect(address, signer);
  }

  contractAddr(): string {
    return this.contract.address;
  }

  async signerAddr(): Promise<string> {
    return this.contract.signer.getAddress();
  }

  async onboard(
    topUp: Big,
    weeklyBudget: Big,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(
      ethereum.fromBaseUnit(topUp),
      0,
      weeklyBudgetToAmountPerSec(weeklyBudget),
      receivers,
      []
    );
  }

  async updatePlan(
    weeklyBudget: Big,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(
      0,
      0,
      weeklyBudgetToAmountPerSec(weeklyBudget),
      receivers,
      []
    );
  }

  async topUp(amount: Big): Promise<ContractTransaction> {
    return this.contract.updateSender(
      ethereum.fromBaseUnit(amount),
      0,
      this.AMT_PER_SEC_UNCHANGED,
      [],
      []
    );
  }

  async withdraw(amount: Big): Promise<ContractTransaction> {
    return this.contract.updateSender(
      0,
      ethereum.fromBaseUnit(amount),
      this.AMT_PER_SEC_UNCHANGED,
      [],
      []
    );
  }

  async withdrawAll(): Promise<ContractTransaction> {
    return this.contract.updateSender(
      0,
      this.WITHDRAW_ALL,
      this.AMT_PER_SEC_UNCHANGED,
      [],
      []
    );
  }

  async collect(): Promise<ContractTransaction> {
    return this.contract.collect();
  }

  // Start watching the state of a given pool sender.
  // `onUpdated` is called immediately with the latest state.
  // Returns a function, which unwatches the state when called.
  async watchPoolSender(
    sender: string,
    onUpdated: (data: PoolSenderData) => void
  ): Promise<() => void> {
    const filter = this.contract.filters.SenderUpdated(sender);

    const listener = async (
      _sender: unknown,
      _balance: unknown,
      _amtPerSec: unknown,
      event: SenderUpdatedEvent
    ) => onUpdated(await getSenderData(event));
    this.contract.on(filter, listener);

    const lastEvents = await this.contract.queryFilter(filter);
    const lastEvent = lastEvents.pop();
    if (lastEvent) {
      onUpdated(await getSenderData(lastEvent));
    } else {
      // Sender never configured
      onUpdated({
        getBalance: () => Big(0),
        weeklyBudget: Big(0),
        receivers: [],
      });
    }

    return () => this.contract.off(filter, listener);
  }

  // Start watching the state of a given pool receiver.
  // `onUpdated` is called immediately with the latest state.
  // Returns a function, which unwatches the state when called.
  async watchPoolReceiver(
    receiver: string,
    onUpdated: (getCollectable: (now: Date) => Big) => void
  ): Promise<() => void> {
    const cycleSecs = (await this.contract.cycleSecs()).toNumber();
    const state = new ReceiverState(cycleSecs);
    const getCollectable = (now: Date) => state.getCollectable(now);

    const filterStream = this.contract.filters.SenderToReceiverUpdated(
      null,
      receiver
    );

    const streamListener = async (
      _sender: unknown,
      _receiver: unknown,
      _amtPerSec: unknown,
      _endTime: unknown,
      event: SenderToReceiverUpdatedEvent
    ) => {
      await state.onStreamUpdated(event);
      // For now it doesn't make sense to call `onUpdated` here,
      // because `getCollectable` doesn't change.
      // This is to keep the behavior of the `watchPoolReceiver`
      // align with its documentation and leave room for addition of
      // more watched parameters, e.g. the list of senders
      onUpdated(getCollectable);
    };
    this.contract.on(filterStream, streamListener);

    const streamEvents = await this.contract.queryFilter(filterStream);
    for (const event of streamEvents) {
      await state.onStreamUpdated(event);
    }

    const filterCollected = this.contract.filters.Collected(receiver);

    const collectedListener = async (
      _receiver: unknown,
      _amt: unknown,
      event: CollectedEvent
    ) => {
      await state.onCollected(event);
      // For now it doesn't make sense to call `onUpdated` here,
      // because `getCollectable` doesn't change.
      // This is to keep the behavior of the `watchPoolReceiver`
      // align with its documentation and leave room for addition of
      // more watched parameters, e.g. the last collection time
      onUpdated(getCollectable);
    };
    this.contract.on(filterCollected, collectedListener);

    const collectedEvents = await this.contract.queryFilter(filterCollected);
    const lastCollectedEvent = collectedEvents.pop();
    if (lastCollectedEvent) {
      await state.onCollected(lastCollectedEvent);
    }

    onUpdated(getCollectable);

    return () => {
      this.contract.off(filterStream, streamListener);
      this.contract.off(filterCollected, collectedListener);
    };
  }
}

// The type used by the Radicle-Contracts library to express a Pool Receiver.
export interface PoolSenderData {
  getBalance: (now: Date) => Big;
  weeklyBudget: Big;
  receivers: string[];
}

async function getSenderData(
  event: SenderUpdatedEvent
): Promise<PoolSenderData> {
  const int = new PoolFactory().interface;

  const receipt = await event.getTransactionReceipt();
  const eventFragment = int.getEvent("SenderToReceiverUpdated");
  // The actual amount sent on every second,
  // may be lower than the sender's configured amtPerSec or even 0
  let totalAmtPerSec = BigNumber.from(0);
  const receivers = [];
  for (const log of receipt.logs) {
    let eventLog;
    try {
      eventLog = int.decodeEventLog(eventFragment, log.data, log.topics);
    } catch {
      // Ignore unmatching events
      continue;
    }
    const amtPerSec = eventLog.amtPerSec;
    if (amtPerSec.isZero()) {
      // Ignore events stopping sending
      continue;
    }
    totalAmtPerSec = totalAmtPerSec.add(amtPerSec);
    receivers.push(eventLog.receiver);
  }

  const balance = Big(event.args.balance.toString());
  const timestamp = (await event.getBlock()).timestamp;
  const amtPerSec = Big(totalAmtPerSec.toString());
  const getBalance = (now: Date) => {
    // The Ethereum timestamps are in seconds
    const timestampNow = Math.floor(now.getTime() / 1000);
    // Block timestamp in the future
    if (timestampNow < timestamp) {
      return balance;
    }
    const spent = amtPerSec.mul(timestampNow - timestamp);
    // Spent everything, return the unspendable reminder
    if (spent > balance) {
      return balance.mod(amtPerSec);
    }
    return balance.minus(spent);
  };

  const weeklyBudget = amountPerSecToWeeklyBudget(event.args.amtPerSec);

  return { getBalance, weeklyBudget, receivers };
}

export interface PoolReceiver {
  // The address of the receiver.
  receiver: string;
  // The share the receiver gets within the pool they are a part of.
  weight: number;
}

interface StreamFragment {
  // Timestamp since which a stream sends funds (inclusive)
  since: number;
  // Timestamp to which a stream sends funds (exclusively)
  to: number;
  amtPerSec: BigNumber;
}

class ReceiverState {
  // For each sender stores a list of non-overlapping stream fragments
  // in chronological order
  streams = new Map<string, StreamFragment[]>();
  cycleSecs: number;
  // Timestamp in seconds from which funds are collectable (inclusively).
  collectableSince = 0;

  constructor(cycleSecs: number) {
    this.cycleSecs = cycleSecs;
  }

  getCollectable(now: Date): Big {
    // The Ethereum timestamps are in seconds
    const nowTimestamp = Math.floor(now.getTime() / 1000);
    // timestamp to which a funds are collectable (exclusively)
    const collectableTo = nowTimestamp - (nowTimestamp % this.cycleSecs);
    let collectable = BigNumber.from(0);
    for (const streamFragments of this.streams.values()) {
      for (const fragment of streamFragments) {
        if (
          fragment.to <= this.collectableSince ||
          fragment.since >= collectableTo
        ) {
          continue;
        }
        const since =
          fragment.since >= this.collectableSince
            ? fragment.since
            : this.collectableSince;
        const to = fragment.to <= collectableTo ? fragment.to : collectableTo;
        const amt = fragment.amtPerSec.mul(to - since);
        collectable = collectable.add(amt);
      }
    }
    return Big(collectable.toString());
  }

  async onStreamUpdated(event: SenderToReceiverUpdatedEvent): Promise<void> {
    const sender = event.args.sender;
    const timestamp = (await event.getBlock()).timestamp;
    let fragments = this.streams.get(sender);
    if (!fragments) {
      fragments = [];
      this.streams.set(sender, fragments);
    }

    // Trim the last fragment if it overlaps with the new one
    const lastFragment = fragments[fragments.length - 1];
    if (lastFragment && lastFragment.to > timestamp) {
      lastFragment.to = timestamp;
    }

    fragments.push({
      since: timestamp,
      to: event.args.endTime.toNumber(),
      amtPerSec: event.args.amtPerSec,
    });
  }

  async onCollected(event: CollectedEvent): Promise<void> {
    const timestamp = (await event.getBlock()).timestamp;
    const cycleStart = timestamp - (timestamp % this.cycleSecs);
    this.collectableSince = cycleStart;
  }
}

// Convert the user-inputed `weeklyBudget` into how much it means per Ethereum block.
function weeklyBudgetToAmountPerSec(weeklyBudget: Big): BigNumber {
  return ethereum.fromBaseUnit(weeklyBudget.div(SECONDS_IN_A_WEEK));
}

// The inverse operation of `weeklyBudgetToAmountPerSec`.
function amountPerSecToWeeklyBudget(amountPerBlock: BigNumber): Big {
  return ethereum.toBaseUnit(
    Big(amountPerBlock.toString()).mul(SECONDS_IN_A_WEEK)
  );
}

const SECONDS_IN_A_WEEK = 604800;
