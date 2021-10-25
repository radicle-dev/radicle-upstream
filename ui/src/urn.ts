// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import multibase from "multibase";
import * as error from "ui/src/error";

// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type Urn = string;

const VALID_URN_MATCH = /^rad:git:([1-9A-HJ-NP-Za-km-z]+)(?:\/.*)?/;

export function extractSha1FromUrn(
  urn: string
):
  | { isUrnValid: true; sha1: Uint8Array }
  | { isUrnValid: false; error: string } {
  const match = urn.match(VALID_URN_MATCH) || [];
  const id = match[1];

  if (!id) {
    return { isUrnValid: false, error: "URN malformed" };
  }

  let hash;
  try {
    hash = multibase.decode(id);
  } catch {
    return {
      isUrnValid: false,
      error: "URN has a malformed multibase payload",
    };
  }

  // A multihash-encoded SHA-1 hash is always 22 bytes.
  if (hash.length !== 22) {
    return {
      isUrnValid: false,
      error: `URN has an invalid payload size: ${hash.length}`,
    };
  }

  // The first byte of a multihash is a hash algorithm identifier,
  // for SHA-1 it's 17.
  if (hash[0] !== 17) {
    return {
      isUrnValid: false,
      error: `URN has an invalid multihash algorithm identifier: ${hash[0]}`,
    };
  }

  // The second byte of a multihash is a hash payload size in bytes,
  // for SHA-1 it's 20.
  if (hash[1] !== 20) {
    return {
      isUrnValid: false,
      error: `URN has an invalid multihash payload size: ${hash[1]}`,
    };
  }

  // Drop multihash header, keep only the payload.
  return { isUrnValid: true, sha1: hash.slice(2) };
}

// Takes a Radicle URN and returns its payload as a binary encoded SHA1
export function urnToSha1(urn: string): Uint8Array {
  const result = extractSha1FromUrn(urn);
  if (result.isUrnValid) {
    return result.sha1;
  } else {
    throw new Error(result.error);
  }
}

// Takes a binary encoded SHA1 and encodes it into a Radicle URN --
// it's the inverse of urnToSha1.
export function sha1ToUrn(hash: Uint8Array): string {
  // a SHA-1 digest is always 20 bytes
  if (hash.length !== 20) {
    throw new error.Error({
      code: error.Code.OrgIdentitySha1UrnError,
      message: "SHA1 hash has invalid size",
      details: { hash, hashLength: hash.length },
    });
  }
  // Create a multihash by adding prefix 17 for SHA-1 and 20 for the hash
  // length.
  const multihash = new Uint8Array([17, 20, ...hash]);
  const payload = multibase.encode("base32z", multihash);
  return `rad:git:${new TextDecoder().decode(payload)}`;
}
