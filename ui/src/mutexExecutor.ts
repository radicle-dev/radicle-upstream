// A task executor that runs only one task concurrently. If a new task
// is run, any previously running task is aborted and the promise
// returned from `run()` will return undefined.
//
//     import * as mutexExecutor from "ui/src/mutexExecutor"
//     const executor = mutexExecutor.create()
//     const first = executor.spwan(async () => {
//       await sleep(1000)
//       return "first"
//     })
//     const second = executor.spwan(async () => "second")
//
// In the example above the promise `first` will resolve to `undefined`
// while the promise `second` will resolve to "second".
//
// If the first tasks throws after the second task has run the
// behavior is the same.
//
//     const first = executor.spwan(async () => {
//       await sleep(1000)
//       throw new Error()
//     })
//
// The task call back receives an AbortSignal as a parameter. The abort
// event is emitted when another task is run.

export function create(): MutexExecutor {
  return new MutexExecutor();
}

class MutexExecutor {
  private runningTaskId = 0;
  private abortController: AbortController | null = null;

  async run<T>(
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
