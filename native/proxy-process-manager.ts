import { spawn, ChildProcess } from "child_process";
import { CircularBuffer } from "mnemonist";

export interface ProcessResult {
  status: number | null;
  signal: NodeJS.Signals | null;
  // The last 200 lines of stdout and stderr combined.
  output: string;
}

// The `ProxyProcessManager` runs and kills the proxy process and allows us to
// obtain the process result when the process exits.
//
// Stdout and stderr of the child process are forwarded to stdout and
// stderr of this process. The lines of both these streams are also
// written to a circular buffer with a capacity of 200 lines.
export class ProxyProcessManager {
  private childProcess: ChildProcess | undefined;
  private proxyPath: string;
  private proxyArgs: string[];
  private enabled: boolean;

  // Create a new ProxyProcessManager.
  //
  // `proxyPath` is the path to the executable and `proxyArgs` are the command
  // line arguments.
  //
  // If `enabled` is set to false no proxy, is started and `run()` never
  // finishes.
  constructor(proxyPath: string, proxyArgs: string[], enabled: boolean) {
    this.proxyPath = proxyPath;
    this.proxyArgs = proxyArgs;
    this.enabled = enabled;
  }

  // Run the proxy process and return `ProcessResult` when it exits.
  //
  // Throws an error if a process is already running.
  async run(): Promise<ProcessResult> {
    if (!this.enabled) {
      return new Promise(() => undefined);
    }

    if (this.childProcess !== undefined) {
      throw new Error("Proxy process already started");
    }
    const outputBuffer = new CircularBuffer<string>(Array, 200);

    const childProcess = spawn(this.proxyPath, this.proxyArgs, {
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
