import { spawn, ChildProcess } from "child_process";
import { CircularBuffer } from "mnemonist";

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
  // If `false` then no proxy is started and `run()` never finishes.
  enabled: boolean;
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
  private childProcess: ChildProcess | undefined;
  private readonly options: Options;

  constructor(options: Options) {
    this.options = options;
  }

  // Run the proxy process and return `ProcessResult` when it exits.
  //
  // Throws an error if a process is already running.
  async run(): Promise<ProcessResult> {
    if (!this.options.enabled) {
      return new Promise(() => undefined);
    }

    if (this.childProcess !== undefined) {
      throw new Error("Proxy process already started");
    }
    const outputBuffer = new CircularBuffer<string>(
      Array,
      this.options.lineLimit
    );

    const childProcess = spawn(this.options.proxyPath, this.options.proxyArgs, {
      stdio: ["ignore", "pipe", "pipe"],
    });

    this.childProcess = childProcess;

    let stdoutBuf = "";
    // We know that `stdout` is set because of the `stdio` spawn options
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    childProcess.stdout!.on("data", (chunk: Buffer) => {
      process.stdout.write(chunk);
      const split = chunk.toString("utf8").split("\n");
      split[0] = stdoutBuf + split[0];
      stdoutBuf = split.pop() || "";
      for (const line of split) {
        outputBuffer.push(line);
      }
    });

    let stderrBuf = "";
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    childProcess.stderr!.on("data", (chunk: Buffer) => {
      process.stderr.write(chunk);
      const split = chunk.toString("utf8").split("\n");
      split[0] = stderrBuf + split[0];
      stderrBuf = split.pop() || "";
      for (const line of split) {
        outputBuffer.push(line);
      }
    });

    const [status, signal] = await new Promise(resolve => {
      childProcess.on("exit", (status, signal) => {
        this.childProcess = undefined;
        resolve([status, signal]);
      });
    });

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

  // Kill the process if it is running. Do nothing otherwise.
  kill(): void {
    if (this.childProcess) {
      this.childProcess.kill();
    }
  }
}
