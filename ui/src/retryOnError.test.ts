// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { retryOnError } from "./retryOnError";

const TRY_COUNT = 10;

test("retries exhausted", async () => {
  const error = new Error("ERROR");
  let callCount = 0;
  const action = async (): Promise<void> => {
    callCount += 1;
    throw error;
  };

  const result = await retryOnError(action, () => true, 0, TRY_COUNT).catch(
    e => e
  );
  expect(result).toBe(error);
  expect(callCount).toBe(TRY_COUNT);
});

test("rethrows unmatched errors", async () => {
  const unmatchedError = new Error("UNMATCHED");
  const retryError = new Error("RETRY");

  let callCount = 0;
  const action = async (): Promise<void> => {
    callCount += 1;
    if (callCount < TRY_COUNT / 2) {
      throw retryError;
    } else {
      throw unmatchedError;
    }
  };

  const result = await retryOnError(
    action,
    e => e === retryError,
    0,
    TRY_COUNT
  ).catch(e => e);
  expect(result).toBe(unmatchedError);
  expect(callCount).toBe(TRY_COUNT / 2);
});

test("passes valid results", async () => {
  const error = new Error("ERROR");

  let callCount = 0;
  const action = async (): Promise<string> => {
    callCount += 1;
    if (callCount < TRY_COUNT / 2) {
      throw error;
    } else {
      return "RESULT";
    }
  };

  const result = await retryOnError(
    action,
    e => e === error,
    0,
    TRY_COUNT
  ).catch(e => e);
  expect(result).toBe("RESULT");
  expect(callCount).toBe(TRY_COUNT / 2);
});
