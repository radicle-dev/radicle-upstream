// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { BigNumber } from "ethers";
import type { Registration } from "ui/src/org/ensResolver";

export interface EnsConfiguration {
  owner: string;
  address: string;
  name: string;
  fee: BigNumber;
  minAge: number;
  commitmentBlock: number;
  commitmentSalt: Uint8Array;
  registered: boolean;
}

export interface SubmitPayload {
  ensNameConfiguration?: Partial<EnsConfiguration>;
  ensMetadata?: Partial<EnsMetadataPayload>;
}

export type EnsMetadataPayload = Registration;
