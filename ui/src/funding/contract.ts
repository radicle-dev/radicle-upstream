import type { BigNumber, ContractTransaction, Signer } from "ethers";
import * as svelteStore from "svelte/store";

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

// Address of the Funding Pool contract
export const POOL_ADDRESS: svelteStore.Readable<string> = svelteStore.derived(
  ethereum.selectedEnvironment,
  $environment => {
    switch ($environment) {
      case ethereum.Environment.Local:
        return addresses.pool.local;
      case ethereum.Environment.Ropsten:
        return addresses.pool.ropsten;
    }
  }
);

export function pool(signer: Signer): PoolContract {
  return new PoolContract(signer, svelteStore.get(POOL_ADDRESS));
}

// Address of the DAI ERC20 token contract
export const DAI_TOKEN_ADDRESS: svelteStore.Readable<string> = svelteStore.derived(
  ethereum.selectedEnvironment,
  $environment => {
    switch ($environment) {
      case ethereum.Environment.Local:
        return addresses.dai.local;
      case ethereum.Environment.Ropsten:
        return addresses.dai.ropsten;
    }
  }
);

export function daiToken(signer: Signer): ERC20 {
  return Erc20Factory.connect(svelteStore.get(DAI_TOKEN_ADDRESS), signer);
}

// PoolContract is a wrapper type around the actual contract, `Erc20Pool`,
// that offers a more ergonomic API around the contract.
export class PoolContract {
  contract: Erc20Pool;

  constructor(signer: Signer, address: string) {
    this.contract = PoolFactory.connect(address, signer);
  }

  async onboard(
    topUp: BigNumber,
    amountPerBlock: BigNumber,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(
      ethereum.toDecimals(topUp),
      0,
      ethereum.toDecimals(amountPerBlock),
      receivers,
      []
    );
  }

  async updatePlan(
    amountPerBlock: BigNumber,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(
      0,
      0,
      ethereum.toDecimals(amountPerBlock),
      receivers,
      []
    );
  }

  async topUp(amount: BigNumber): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    return this.contract.updateSender(
      ethereum.toDecimals(amount),
      0,
      UNCHANGED,
      [],
      []
    );
  }

  async withdraw(amount: BigNumber): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    const ALL = await this.withdrawAllFlag();
    const finalAmount = amount.eq(ALL) ? ALL : ethereum.toDecimals(amount);
    return this.contract.updateSender(0, finalAmount, UNCHANGED, [], []);
  }

  async collect(): Promise<ContractTransaction> {
    return this.contract.collect().then((x: BigNumber) => ethereum.toHumans(x));
  }

  async withdrawAllFlag(): Promise<BigNumber> {
    return this.contract.WITHDRAW_ALL();
  }

  async withdrawable(): Promise<BigNumber> {
    return this.contract
      .withdrawable()
      .then((x: BigNumber) => ethereum.toHumans(x));
  }

  async collectable(): Promise<BigNumber> {
    return this.contract
      .collectable()
      .then((x: BigNumber) => ethereum.toHumans(x));
  }

  async amountPerBlock(): Promise<BigNumber> {
    return this.contract
      .getAmountPerBlock()
      .then((x: BigNumber) => ethereum.toHumans(x));
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
