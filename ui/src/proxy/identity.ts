import * as zod from "zod";

export interface Avatar {
  background: {
    r: number;
    g: number;
    b: number;
  };
  emoji: string;
}

const avatarSchema: zod.ZodSchema<Avatar> = zod.object({
  background: zod.object({
    r: zod.number(),
    g: zod.number(),
    b: zod.number(),
  }),
  emoji: zod.string(),
});

export interface Identity {
  avatarFallback: Avatar;
  metadata: Metadata;
  peerId: string;
  shareableEntityIdentifier: string;
  urn: string;
}

export interface Metadata {
  handle: string;
  ethereum: Ethereum | null;
}

// A claim over an Ethereum Address
export interface Ethereum {
  // TODO(nuno): make type-safe?
  address: string;
  expiration: string;
}

export const identitySchema: zod.ZodSchema<Identity> = zod.object({
  avatarFallback: avatarSchema,
  metadata: zod.object({
    handle: zod.string(),
    ethereum: zod
      .object({
        address: zod.string(),
        expiration: zod.string(),
      })
      .nullable(),
  }),
  peerId: zod.string(),
  shareableEntityIdentifier: zod.string(),
  urn: zod.string(),
});
