import { get } from "svelte/store";
import type { Identity } from "./identity";
import { ethereumAddress } from "./identity";
import type { ClaimsContract } from "./funding/contract";

export enum Status {
  Incomplete,
  Verified,
  Refuted,
};

export async function status(connectedAddress: string, localIdentity: Identity, claims: ClaimsContract): Promise<Status> {
  let claimedEthAddress = get(ethereumAddress);
  let claimedIdentity = await claims.claimed();

  if (claimedEthAddress === null || claimedIdentity === null) {
    return Status.Incomplete;
  }
  else if (claimedEthAddress === connectedAddress && claimedIdentity === localIdentity.peerId) {
    return Status.Verified;
  }
  else {
    return Status.Refuted;
  }
}