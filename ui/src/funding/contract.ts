import type { ethers, BigNumber, ContractTransaction, Signer } from "ethers";

import Big from "big.js";

import {
  Erc20Pool,
  Erc20Pool__factory as PoolFactory,
  ERC20,
  ERC20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

import * as ethereum from "../ethereum";

const addresses = {
  pool: {
    local: "0x37c10d847bf9e708add451bf2f80c1297d7aa691",
    ropsten: "0xEc5bfca987C5FAA6d394044793f0aD6C9A85Da76",
  },
  dai: {
    local: "0xf34a89802590f944e3de71b1f74d66ed1bafc9cd",
    ropsten: "0xD069f9Cbe64979953357bCa3f21d902e775f1F48",
  },
};

// Get the address of the Pool Contract for the given environment
export function poolAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.pool.local;
    case ethereum.Environment.Ropsten:
      return addresses.pool.ropsten;
  }
}

export function pool(signer: Signer, address: string): PoolContract {
  return new PoolContract(signer, address);
}

// Get the address of the Pool Contract for the given environment
export function daiTokenAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.dai.local;
    case ethereum.Environment.Ropsten:
      return addresses.dai.ropsten;
  }
}

export function daiToken(signer: Signer, address: string): ERC20 {
  return Erc20Factory.connect(address, signer);
}

// PoolContract is a wrapper type around the actual contract, `Erc20Pool`,
// that offers a more ergonomic API around the contract.
export class PoolContract {
  contract: Erc20Pool;

  constructor(signer: Signer, address: string) {
    this.contract = PoolFactory.connect(address, signer);
  }

  async onboard(
    topUp: Big,
    weeklyBudget: Big,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(
      ethereum.fromBaseUnit(topUp),
      0,
      weeklyBudgetToAmountPerBlock(weeklyBudget),
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
      weeklyBudgetToAmountPerBlock(weeklyBudget),
      receivers,
      []
    );
  }

  async topUp(amount: Big): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    return this.contract.updateSender(
      ethereum.fromBaseUnit(amount),
      0,
      UNCHANGED,
      [],
      []
    );
  }

  async withdraw(amount: Big): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    return this.contract.updateSender(
      0,
      ethereum.fromBaseUnit(amount),
      UNCHANGED,
      [],
      []
    );
  }

  async withdrawAll(): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    const ALL = await this.withdrawAllFlag();
    return this.contract.updateSender(0, ALL, UNCHANGED, [], []);
  }

  async collect(): Promise<ContractTransaction> {
    return this.contract.collect();
  }

  async withdrawAllFlag(): Promise<ethers.BigNumber> {
    return this.contract.WITHDRAW_ALL();
  }

  async withdrawable(): Promise<Big> {
    return this.contract.withdrawable().then(ethereum.toBaseUnit);
  }

  async collectable(): Promise<Big> {
    return this.contract.collectable().then(ethereum.toBaseUnit);
  }

  async weeklyBudget(): Promise<Big> {
    return this.contract.getAmountPerBlock().then(amountPerBlockToWeeklyBudget);
  }

  async receivers(): Promise<PoolReceiver[]> {
    return this.contract.getAllReceivers();
  }
}

// The type used by the Radicle-Contracts library to express a Pool Receiver.
export interface PoolReceiver {
  // The address of the receiver.
  receiver: string;
  // The share the receiver gets within the pool they are a part of.
  weight: number;
}

// Convert the user-inputed `weeklyBudget` into how much it means per Ethereum block.
function weeklyBudgetToAmountPerBlock(weeklyBudget: Big): BigNumber {
  return ethereum.fromBaseUnit(weeklyBudget.div(ESTIMATED_BLOCKS_IN_WEEK));
}

// The inverse operation of `weeklyBudgetToAmountPerBlock`.
function amountPerBlockToWeeklyBudget(amountPerBlock: BigNumber): Big {
  return ethereum.toBaseUnit(
    Big(amountPerBlock.toString()).mul(ESTIMATED_BLOCKS_IN_WEEK)
  );
}

// The Ethereum network aims to mine a block at every 12.5 seconds.
const AVG_ETHEREUM_BLOCK_TIME_SECONDS = 12.5;

// The number of seconds in a week
const AVG_SECONDS_IN_WEEK = 604800;

// The estimated number of Ethereum blocks mined in a week
const ESTIMATED_BLOCKS_IN_WEEK =
  AVG_SECONDS_IN_WEEK / AVG_ETHEREUM_BLOCK_TIME_SECONDS;
