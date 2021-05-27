import * as validation from "./validation";
import * as multibase from "multibase";

// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type Urn = string;

// URN validation.
const VALID_URN_MATCH = /^rad:git:[1-9A-HJ-NP-Za-km-z]{37}/;
const GET_URN_ID = /^rad:git:([1-9A-HJ-NP-Za-km-z]+)(?:\/.*)?/;

const urnConstraints = {
  format: {
    pattern: VALID_URN_MATCH,
    message: `Not a valid Radicle ID`,
  },
};

export const urnValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(urnConstraints);

// Parses the identity URN and returns its SHA-1 root hash
export function parseIdentitySha1(urn: string): Uint8Array {
  const matches = urn.match(GET_URN_ID) || [];
  const id = matches[1];
  if (!id) {
    throw new Error("URN malformed");
  }
  let hash;
  try {
    hash = multibase.decode(id);
  } catch {
    throw new Error("URN has a malformed multibase payload");
  }
  // A multihash-encoded SHA-1 hash is always 22 bytes
  if (hash.length !== 22) {
    throw new Error(`URN has an invalid payload size: ${hash.length}`);
  }
  // The first byte of a multihash is a hash algorithm identifier, for SHA-1 it's 17
  if (hash[0] !== 17) {
    throw new Error(
      `URN has an invalid multihash algorithm identifier: ${hash[0]}`
    );
  }
  // The second byte of a multihash is a hash payload size in bytes, for SHA-1 it's 20
  if (hash[1] !== 20) {
    throw new Error(`URN has an invalid multihash payload size: ${hash[1]}`);
  }
  // Drop multihash header, keep only the payload
  return hash.slice(2);
}

export function identitySha1Urn(hash: Uint8Array): string {
  // a SHA-1 digest is always 20 bytes
  if (hash.length != 20) {
    throw new Error(`SHA1 hash has invalid size: ${hash.length}`);
  }
  // Create a multihash by adding prefix 17 for SHA-1 and 20 for the hash length
  const multihash = new Uint8Array([17, 20, ...hash]);
  const payload = multibase.encode("base32z", multihash);
  return `rad:git:${new TextDecoder().decode(payload)}`;
}
