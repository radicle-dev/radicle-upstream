import type { Signer } from "ethers";
import {
  Erc20Pool as PoolContract,
  Erc20Pool__factory as PoolFactory,
  Erc20,
  Erc20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

// TODO(nuno): make the contract addresses configurable/env-dependant

export const POOL_ADDRESS: string =
  "0x8bc07c0de95a0c1a08f6736d07a233fb8609ee95";

export function pool(signer: Signer): PoolContract {
  return PoolFactory.connect(POOL_ADDRESS, signer);
}

export const ERC20_TOKEN_ADDRESS = "0xff1d4d289bf0aaaf918964c57ac30481a67728ef";

export function erc20Token(signer: Signer): Erc20 {
  return Erc20Factory.connect(ERC20_TOKEN_ADDRESS, signer);
}
