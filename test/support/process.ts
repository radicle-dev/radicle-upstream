// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import execa from "execa";
import chalk, { Color } from "chalk";
import { StringDecoder } from "string_decoder";
import onExit from "exit-hook";
import * as stream from "stream";

const colors: typeof Color[] = [
  "blue",
  "cyan",
  "green",
  "magenta",
  "red",
  "yellow",
  "white",
];
const assignedColors: Record<string, typeof Color> = {};

// Processes that should be SIGKILLed when the Node process shutsdown.
// We add all proxy and seed instances that we spawn to this list.
const processes: execa.ExecaChildProcess[] = [];

onExit(killAllProcesses);

// Kill all processes with SIGKILL
export function killAllProcesses(): void {
  for (const process of processes) {
    if (process.exitCode === null) {
      process.kill("SIGKILL");
    }
  }
}

// Spawn a process with `execa` and register it.
//
// The process will be killed by `killAllProcesses`.
export function spawn(
  bin: string,
  args: string[],
  options?: execa.Options
): execa.ExecaChildProcess {
  const child = execa(bin, args, options);
  processes.push(child);
  return child;
}

// Forwards piped `stdout` and `stderr` of a child process to this
// process’s `stdout` and prefixes it with the given label. The prefix
// is colored.
export function prefixOutput(
  childProcess: execa.ExecaChildProcess,
  label: string
): execa.ExecaChildProcess {
  const pref = makePrefix(label);
  if (childProcess.stdout) {
    const stdoutPrefix = new LinePrefix(pref);
    childProcess.stdout.pipe(stdoutPrefix).pipe(process.stdout);
  }
  if (childProcess.stderr) {
    const stderrPrefix = new LinePrefix(pref);
    childProcess.stderr.pipe(stderrPrefix).pipe(process.stderr);
  }

  return childProcess;
}

function makePrefix(label: string): string {
  if (assignedColors[label] === undefined) {
    const color = colors.pop();
    if (!color) {
      throw new Error("we're out of colors 🤷");
    }

    assignedColors[label] = color;
  }

  // We reset colors at the beginning of each line to avoid styles from previous
  // lines messing up prefix colors. This is noticable in rust stack traces
  // where the `in` and `with` keywords have a white background color.
  return chalk.reset[assignedColors[label]](`${label.padEnd(12)} | `);
}

// A transform that prefixes each line from the source with the given
// string and pushes it to the sink.
class LinePrefix extends stream.Transform {
  private buffer: string = "";
  private stringDecoder = new StringDecoder();

  public constructor(private prefix: string) {
    super();
  }

  public _transform(data: Buffer, _encoding: string, next: () => void): void {
    const str = this.buffer + this.stringDecoder.write(data);
    const lines = str.split(/\r?\n/);
    this.buffer = lines.pop() || "";
    lines.forEach(line => this.push(`${this.prefix}${line}\n`));
    next();
  }

  public _flush(done: () => void): void {
    const rest = `${this.buffer}${this.stringDecoder.end()}`;
    if (rest) {
      this.push(`${this.prefix}${rest}\n`);
    }
    done();
  }
}
