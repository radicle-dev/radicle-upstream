// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { CircularBuffer } from "mnemonist";
import stripAnsi from "strip-ansi";
import execa from "execa";

export interface ProcessResult {
  status: number | null;
  signal: NodeJS.Signals | null;
  // The last lines of stdout and stderr combined.
  output: string;
}

interface Options {
  // Path to the executables
  proxyPath: string;
  // Arguments passed to the executable
  proxyArgs: string[];
  // Maximum number of log lines we store
  lineLimit: number;
}

// The `ProxyProcessManager` runs and kills the proxy process and allows us to
// obtain the process result when the process exits.
//
// Stdout and stderr of the child process are forwarded to stdout and
// stderr of this process. The lines of both these streams are also
// written to a circular buffer with a configurable line capacity.
export class ProxyProcessManager {
  private childProcess: execa.ExecaChildProcess | undefined;
  private shutdownInProgress: boolean = false;
  private readonly options: Options;

  public constructor(options: Options) {
    this.options = options;
  }

  // Run the proxy process and return `ProcessResult` when it exits.
  //
  // Throws an error if a process is already running.
  public async run(): Promise<ProcessResult> {
    if (this.childProcess !== undefined) {
      throw new Error("Proxy process already started");
    }
    const outputBuffer = new CircularBuffer<string>(
      Array,
      this.options.lineLimit
    );

    const childProcess = execa(this.options.proxyPath, this.options.proxyArgs, {
      stdio: ["ignore", "pipe", "pipe"],
      buffer: false,
    });

    this.childProcess = childProcess;

    // We know that `stdout` is set because of the `stdio` spawn options
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const stdout = childProcess.stdout!;

    const stdoutDone = streamDone(stdout);

    let stdoutBuf = "";
    stdout.on("data", (chunk: Buffer) => {
      process.stdout.write(chunk);
      const split = chunk.toString("utf8").split("\n");
      split[0] = stdoutBuf + split[0];
      stdoutBuf = split.pop() || "";
      for (const line of split) {
        outputBuffer.push(stripAnsi(line));
      }
    });

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const stderr = childProcess.stderr!;

    const stderrDone = streamDone(stderr);

    let stderrBuf = "";
    stderr.on("data", (chunk: Buffer) => {
      process.stderr.write(chunk);
      const split = chunk.toString("utf8").split("\n");
      split[0] = stderrBuf + split[0];
      stderrBuf = split.pop() || "";
      for (const line of split) {
        outputBuffer.push(stripAnsi(line));
      }
    });

    const [status, signal] = await new Promise(resolve => {
      childProcess.on("exit", (status, signal) => {
        this.childProcess = undefined;
        resolve([status, signal]);
      });
    });

    await Promise.all([stdoutDone, stderrDone]);

    let output = Array.from(outputBuffer).join("\n");
    if (stderrBuf) {
      output = `${output}\n${stderrBuf}`;
    }
    if (stdoutBuf) {
      output = `${output}\n${stdoutBuf}`;
    }

    return {
      status,
      signal,
      output,
    };
  }

  // Shutdown the process with SIGTERM and wait for it to exit if it is running. Do nothing otherwise.
  public async shutdown(): Promise<void> {
    if (this.childProcess) {
      if (!this.shutdownInProgress) {
        this.childProcess.kill("SIGTERM");
        this.shutdownInProgress = true;
      }
      await this.childProcess;
    }
  }
}

// Returns a promise that resolves when the stream ends or errors.
function streamDone(stream: NodeJS.ReadableStream): Promise<void> {
  return new Promise<void>(resolve => {
    stream.on("end", () => resolve());
    stream.on("error", () => resolve());
  });
}
