import type { BigNumberish, ContractTransaction, Signer } from "ethers";
import {
  Erc20Pool,
  Erc20Pool__factory as PoolFactory,
  Erc20,
  Erc20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

// TODO(nuno): make the contract addresses configurable/env-dependant

export const POOL_ADDRESS: string =
  "0x8bc07c0de95a0c1a08f6736d07a233fb8609ee95";

export function pool(
  signer: Signer,
  address: string = POOL_ADDRESS
): PoolContract {
  return new PoolContract(signer, address);
}

export const ERC20_TOKEN_ADDRESS = "0xff1d4d289bf0aaaf918964c57ac30481a67728ef";

export function erc20Token(signer: Signer): Erc20 {
  return Erc20Factory.connect(ERC20_TOKEN_ADDRESS, signer);
}

// PoolContract is a wrapper type around the actual contract, `Erc20Pool`,
// that offers a more ergonomic API around the contract.
export class PoolContract {
  contract: Erc20Pool;

  constructor(signer: Signer, address: string = POOL_ADDRESS) {
    this.contract = PoolFactory.connect(address, signer);
  }

  async onboard(
    topUp: BigNumberish,
    amountPerBlock: BigNumberish,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(topUp, 0, amountPerBlock, receivers, []);
  }

  async updatePlan(
    amountPerBlock: BigNumberish,
    receivers: PoolReceiver[]
  ): Promise<ContractTransaction> {
    return this.contract.updateSender(0, 0, amountPerBlock, receivers, []);
  }

  async topUp(amount: BigNumberish): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    return this.contract.updateSender(amount, 0, UNCHANGED, [], []);
  }

  async withdraw(amount: BigNumberish): Promise<ContractTransaction> {
    const UNCHANGED = await this.contract.AMOUNT_PER_BLOCK_UNCHANGED();
    return this.contract.updateSender(0, amount, UNCHANGED, [], []);
  }

  async collect(): Promise<ContractTransaction> {
    return this.contract.collect();
  }

  async withdrawAllFlag(): Promise<BigNumberish> {
    return this.contract.WITHDRAW_ALL();
  }

  async withdrawable(): Promise<BigNumberish> {
    return this.contract.withdrawable();
  }

  async collectable(): Promise<BigNumberish> {
    return this.contract.collectable();
  }

  async amountPerBlock(): Promise<BigNumberish> {
    return this.contract.getAmountPerBlock();
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
