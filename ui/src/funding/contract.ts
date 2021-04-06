import * as ethers from "ethers";
import type { BigNumber, ContractTransaction, Signer } from "ethers";

import Big from "big.js";

import {
  Claims,
  Claims__factory as ClaimsFactory,
  Erc20Pool,
  Erc20Pool__factory as PoolFactory,
  ERC20,
  ERC20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

import * as ethereum from "../ethereum";
import * as transaction from "../transaction";

const addresses = {
  pool: {
    local: "0x56a32c0c857f1ae733562078a693ea845d9bb423",
    ropsten: "0x336C7fE92c08A9e48738a48f846860C1fD35647C",
  },
  dai: {
    local: "0xff1d4d289bf0aaaf918964c57ac30481a67728ef",
    ropsten: "0x6e80bf4Fd0b102E6385C545375C8fF3B30D554eA",
  },
  claims: {
    local: "0x785e8de68df899d77ce689f863e4166849c8bfd5",
    ropsten: "0xF8F22AA794DDA79aC0C634a381De0226f369bCCe",
  },
};

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

// Get the address of the Claims Contract for the given environment
export function claimsAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.claims.local;
    case ethereum.Environment.Ropsten:
      return addresses.claims.ropsten;
  }
}

export function claims(signer: Signer, address: string): ClaimsContract {
  return new ClaimsContract(signer, address);
}

export class ClaimsContract {
  contract: Claims;

  constructor(signer: Signer, address: string) {
    this.contract = ClaimsFactory.connect(address, signer);
  }

  async claim(root: string): Promise<void> {
    const payload = ethers.utils.toUtf8Bytes(root);
    return this.contract
      .claim(0, payload)
      .then((ctx: ContractTransaction) =>
        transaction.add(transaction.claimRadicleIdentity(ctx, root))
      );
  }

  // Fetch the latest Radicle Identity root claim by the given ethereum address.
  // Return undefined if no claim is found.
  async claimed(address: string): Promise<string | undefined> {
    const filter = this.contract.filters.Claimed(address);
    const events = await this.contract.queryFilter(filter, 0, "latest");
    const last = events[events.length - 1];

    if (last === undefined) {
      return undefined;
    }

    const tx = await last.getTransaction();
    const claimsFactory = new ClaimsFactory();
    const inputs = claimsFactory.interface.decodeFunctionData("claim", tx.data);
    const payload = ethers.utils.arrayify(inputs.payload);
    const root = Buffer.from(payload).toString("utf-8");

    return root;
  }
}

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
    const UNCHANGED = await this.contract.AMT_PER_SEC_UNCHANGED();
    return this.contract.updateSender(
      ethereum.fromBaseUnit(amount),
      0,
      UNCHANGED,
      [],
      []
    );
  }

  async withdraw(amount: Big): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMT_PER_SEC_UNCHANGED();
    return this.contract.updateSender(
      0,
      ethereum.fromBaseUnit(amount),
      UNCHANGED,
      [],
      []
    );
  }

  async withdrawAll(): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMT_PER_SEC_UNCHANGED();
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
    return this.contract.getAmtPerSec().then(amountPerSecToWeeklyBudget);
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
