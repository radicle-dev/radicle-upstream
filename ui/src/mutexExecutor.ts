// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Bacon from "ui/src/bacon";

// A task executor that runs only one task concurrently. If a new task
// is run, any previously running task is aborted and the promise
// returned from `run()` will return undefined.
//
//     import * as mutexExecutor from "ui/src/mutexExecutor"
//     const executor = mutexExecutor.create()
//     const first = await executor.run(async () => {
//       await sleep(1000)
//       return "first"
//     })
//     const second = await executor.run(async () => "second")
//
// In the example above the promise `first` will resolve to `undefined`
// while the promise `second` will resolve to "second".
//
// If the first tasks throws after the second task has run the
// behavior is the same.
//
//     const first = await executor.run(async () => {
//       await sleep(1000)
//       throw new Error()
//     })
//
// The task call back receives an AbortSignal as a parameter. The abort
// event is emitted when another task is run.
export function create(): MutexExecutor {
  return new MutexExecutor();
}

// A worker that asynchronously process one item at a time and provides
// the result as an event stream.
//
//     import * as mutexExecutor from "ui/src/mutexExecutor"
//     const worker = mutexExecutor.createWorker(async (value) => {
//       await sleep(1000)
//       return value
//     })
//
//     const firstPromise = worker.output.firstToPromise()
//     worker.push("first)
//     assert.equal(await firstPromise, "first")
//
// When an item is submitted to the worker while the previous items is
// still being processed, the result of the first item will not be
// emitted to `worker.output`. Instead, only the last item will be
// emitted.
export function createWorker<In, Out>(
  fn: (x: In, abortSignal: AbortSignal) => Promise<Out>
): MutexWorker<In, Out> {
  return new MutexWorker(fn);
}

class MutexExecutor {
  private runningTaskId = 0;
  private abortController: AbortController | null = null;

  public async run<T>(
    f: (abortSignal: AbortSignal) => Promise<T>
  ): Promise<T | undefined> {
    this.runningTaskId += 1;
    const taskId = this.runningTaskId;

    if (this.abortController) {
      this.abortController.abort();
    }
    this.abortController = new AbortController();
    return f(this.abortController.signal).then(
      data => {
        if (this.runningTaskId === taskId) {
          return data;
        } else {
          return undefined;
        }
      },
      err => {
        if (this.runningTaskId === taskId) {
          throw err;
        } else {
          return undefined;
        }
      }
    );
  }
}

class MutexWorker<In, Out> {
  private outputBus = new Bacon.Bus<Out>();
  private executor = new MutexExecutor();

  public output: Bacon.EventStream<Out>;

  public constructor(
    private fn: (x: In, abortSignal: AbortSignal) => Promise<Out>
  ) {
    this.output = this.outputBus.toEventStream();
  }

  public async submit(x: In): Promise<void> {
    const output = await this.executor.run(abort => this.fn(x, abort));
    if (output !== undefined) {
      this.outputBus.push(output);
    }
  }
}
