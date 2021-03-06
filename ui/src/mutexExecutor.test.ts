// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as mutexExecutor from "./mutexExecutor";
import { sleep } from "./sleep";
import * as sinon from "sinon";

describe("executor", () => {
  test("cancels running task", async () => {
    const e = mutexExecutor.create();

    const first = e.run(async () => {
      await sleep(10);
      return "first";
    });
    const second = e.run(async () => {
      return "second";
    });

    expect(await first).toBe(undefined);
    expect(await second).toBe("second");

    const third = e.run(async () => {
      await sleep(10);
      return "third";
    });
    const fourth = e.run(async () => {
      return "fourth";
    });

    expect(await third).toBe(undefined);
    expect(await fourth).toBe("fourth");
  });

  test("cancels multiple tasks", async () => {
    const e = mutexExecutor.create();

    const canceled1 = e.run(async () => {
      await sleep(10);
      return true;
    });
    const canceled2 = e.run(async () => {
      await sleep(10);
      return true;
    });
    const canceled3 = e.run(async () => {
      await sleep(10);
      return true;
    });
    const last = e.run(async () => {
      return true;
    });

    expect(await canceled1).toBe(undefined);
    expect(await canceled2).toBe(undefined);
    expect(await canceled3).toBe(undefined);
    expect(await last).toBe(true);
  });

  test("triggers abort signal event", async () => {
    const e = mutexExecutor.create();
    const abortListener = sinon.spy();

    e.run(async abort => {
      abort.addEventListener("abort", abortListener);
      await sleep(10);
      return "first";
    });
    expect(abortListener.called).toBe(false);
    e.run(async () => {});
    expect(abortListener.called).toBe(true);
  });

  test("don’t throw error on aborted task", async () => {
    const e = mutexExecutor.create();

    const first = e.run(async () => {
      await sleep(10);
      throw new Error();
    });
    const second = e.run(async () => {
      return "second";
    });

    expect(await first).toBe(undefined);
    expect(await second).toBe("second");
  });
});

describe("worker", () => {
  test("sequential work", async () => {
    const w = mutexExecutor.createWorker(async (value: number) => {
      await sleep(10);
      return value;
    });

    const outputs: number[] = [];
    w.output.onValue(value => outputs.push(value));

    await w.submit(1);
    await w.submit(2);
    await w.submit(3);

    expect(outputs).toEqual([1, 2, 3]);
  });

  test("overlapping work cancels", async () => {
    const w = mutexExecutor.createWorker(async (value: number) => {
      await sleep(10);
      return value;
    });

    const nextOutput = w.output.firstToPromise();

    w.submit(1);
    w.submit(2);
    w.submit(3);

    expect(await nextOutput).toEqual(3);
  });
});
