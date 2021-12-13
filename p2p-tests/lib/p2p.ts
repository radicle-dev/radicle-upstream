// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import fetch, { Headers, Request, Response } from "node-fetch";

if (!globalThis.fetch) {
  // This might be due to https://github.com/microsoft/TypeScript/issues/43990.
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  globalThis.fetch = fetch;
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  globalThis.Headers = Headers;
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  globalThis.Request = Request;
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  globalThis.Response = Response;
}

import * as ProxyClient from "proxy-client";
import * as fs from "fs-extra";
import * as path from "path";
import * as stream from "stream";
import execa from "execa";
import chalk, { Color } from "chalk";
import { StringDecoder } from "string_decoder";

import { retryOnError } from "ui/src/retryOnError";

const PADDING = 12;

const colors: typeof Color[] = ["blue", "cyan", "green", "magenta", "red"];
const assignedColors: Record<string, typeof Color> = {};

const ROOT_PATH = path.join(__dirname, "..", "..");
const P2P_TEST_PATH = path.join(ROOT_PATH, "p2p-tests");
const BIN_PATH = path.join(ROOT_PATH, "target", "debug");

const processes: execa.ExecaChildProcess[] = [];

export function stopProcesses(): void {
  processes.forEach(node => node.kill("SIGKILL"));
}

export function workspacePath(paths: string[] = []): string {
  return path.join(P2P_TEST_PATH, "workspace", ...paths);
}

export function p2pTestPath(filename: string): string {
  return path.join(P2P_TEST_PATH, filename);
}

export function keyPath(filename: string): string {
  return path.join(P2P_TEST_PATH, "keys", filename);
}

function prefix(pfx: string): string {
  if (assignedColors[pfx] === undefined) {
    const color = colors.pop();
    if (color) {
      assignedColors[pfx] = color;
    } else {
      throw new Error("we're out of colors 🤷");
    }
  }

  // We reset colors at the beginning of each line to avoid styles from previous
  // lines messing up prefix colors. This is noticable in rust stack traces
  // where the `in` and `with` keywords have a white background color.
  return chalk.reset[assignedColors[pfx]](`${pfx.padEnd(PADDING)} | `);
}

// A transform that prefixes each line from the source with the given
// string and pushes it to the sink.
export class LinePrefix extends stream.Transform {
  private buffer: string = "";
  private stringDecoder = new StringDecoder();

  public constructor(private prefix: string) {
    super();
  }

  public _transform(data: Buffer, _encoding: string, next: () => void) {
    const str = this.buffer + this.stringDecoder.write(data);
    const lines = str.split(/\r?\n/);
    this.buffer = lines.pop() || "";
    lines.forEach(line => {
      if (line === "") {
        this.push("\n");
      } else {
        this.push(`${this.prefix}${line}\n`);
      }
    });
    next();
  }

  public _flush(done: () => void) {
    this.push(`${this.prefix}${this.buffer}${this.stringDecoder.end()}\n`);
    done();
  }
}

export function initPeer(namespace: string): void {
  const radHome = workspacePath([namespace]);
  fs.removeSync(radHome);
  fs.mkdirsSync(radHome);

  execa.sync(path.join(BIN_PATH, "radicle-proxy-init"), [
    namespace,
    "--key-passphrase",
    namespace,
    "--rad-home",
    workspacePath([namespace]),
  ]);
}

export function startPeer(
  namespace: string,
  ip: string,
  args: string[]
): ProxyClient.ProxyClient {
  initPeer(namespace);

  const childProcess = spawnInNamespace(
    namespace,
    [
      path.join(BIN_PATH, "radicle-proxy"),
      "--peer-listen",
      `${ip}:8776`,
      "--http-listen",
      `${ip}:17246`,
      "--skip-remote-helper-install",
      "--unsafe-fast-keystore",
      "--insecure-http-api",
      "--dev-log",
      ...args,
    ],
    {
      RAD_HOME: workspacePath([namespace]),
    }
  );

  processes.push(childProcess);

  return new ProxyClient.ProxyClient(`http://${ip}:17246`);
}

export function startSeed(namespace: string, args: string[]): void {
  const radHome = workspacePath([namespace]);
  fs.removeSync(radHome);
  fs.mkdirsSync(radHome);

  const childProcess = spawnInNamespace(
    namespace,
    [path.join(BIN_PATH, "upstream-seed"), "--rad-home", radHome, ...args],
    {}
  );

  processes.push(childProcess);
}

export function spawnInNamespace(
  namespace: string,
  args: string[],
  env: NodeJS.ProcessEnv
): execa.ExecaChildProcess {
  const subprocess = execa(
    "ip",
    ["netns", "exec", `upstream-test-${namespace}`, ...args],
    {
      env,
    }
  );

  const stdoutPrefix = new LinePrefix(prefix(namespace));
  const stderrPrefix = new LinePrefix(prefix(namespace));

  if (subprocess.stdout) {
    subprocess.stdout.pipe(stdoutPrefix).pipe(process.stdout);
  }
  if (subprocess.stderr) {
    subprocess.stderr.pipe(stderrPrefix).pipe(process.stderr);
  }

  return subprocess;
}

export function commit(author: string, checkoutPath: string): void {
  execa.sync("git", ["commit", "--allow-empty", "-m", "commit-message"], {
    cwd: checkoutPath,
    env: {
      GIT_AUTHOR_NAME: author,
      GIT_AUTHOR_EMAIL: `${author}@${author}.com`,
      GIT_COMMITTER_NAME: author,
      GIT_COMMITTER_EMAIL: `${author}@${author}.com`,
    },
  });
}

export function pushRad(
  radHome: string,
  checkoutPath: string,
  keyPassphrase: string
): void {
  execa.sync("git", ["push", "rad"], {
    cwd: checkoutPath,
    env: {
      RADICLE_UNSAFE_FAST_KEYSTORE: "1",
      RAD_HOME: radHome,
      KEY_PASSPHRASE: keyPassphrase,
      GIT_EXEC_PATH: BIN_PATH,
    },
  });
}

export async function withRetry(action: () => Promise<unknown>) {
  return await retryOnError(action, () => true, 1000, 10);
}
