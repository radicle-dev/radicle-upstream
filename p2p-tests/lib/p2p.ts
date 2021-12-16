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
import onExit from "exit-hook";

import { retryOnError } from "ui/src/retryOnError";

const PADDING = 12;

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

const ROOT_PATH = path.join(__dirname, "..", "..");
const P2P_TEST_PATH = path.join(ROOT_PATH, "p2p-tests");

// Processes that should be SIGKILLed when the Node process shutsdown.
// We add all proxy and seed instances that we spawn to this list.
const processes: execa.ExecaChildProcess[] = [];

onExit(() => {
  for (const process of processes) {
    process.kill("SIGKILL");
  }
});

function binPath(): string {
  if (process.env.CARGO_TARGET_DIR === undefined) {
    return path.join(ROOT_PATH, "target", "debug");
  } else {
    return path.join(process.env.CARGO_TARGET_DIR, "debug");
  }
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
    lines.forEach(line => {
      if (line === "") {
        this.push("\n");
      } else {
        this.push(`${this.prefix}${line}\n`);
      }
    });
    next();
  }

  public _flush(done: () => void): void {
    this.push(`${this.prefix}${this.buffer}${this.stringDecoder.end()}\n`);
    done();
  }
}

interface RadicleProxyParams {
  dataPath: string;
  ipAddress: string;
  name: string;
  seed: string;
}

export class RadicleProxy {
  public checkoutPath: string;
  public identityUrn: string;
  public name: string;
  public passphrase: string;
  public peerId: string;
  public proxyClient: ProxyClient.ProxyClient;
  public radHome: string;

  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #ipAddress: string;
  #seed: string;

  public constructor({ dataPath, ipAddress, name, seed }: RadicleProxyParams) {
    this.#ipAddress = ipAddress;
    this.#seed = seed;
    this.name = name;
    this.passphrase = name;

    this.checkoutPath = path.join(dataPath, `${name}-checkouts`);
    this.radHome = path.join(dataPath, `${name}-rad-home`);

    fs.mkdirsSync(this.radHome);

    const initResult = JSON.parse(
      execa.sync(path.join(binPath(), "radicle-proxy-init"), [
        this.name,
        "--key-passphrase",
        this.passphrase,
        "--rad-home",
        this.radHome,
      ]).stdout
    );

    this.identityUrn = initResult.identityUrn;
    this.peerId = initResult.peerId;

    this.proxyClient = new ProxyClient.ProxyClient(
      `http://${this.#ipAddress}:17246`
    );
  }

  public start(): void {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    this.#childProcess = spawnInNamespace(
      this.name,
      [
        path.join(binPath(), "radicle-proxy"),
        "--peer-listen",
        `${this.#ipAddress}:8776`,
        "--http-listen",
        `${this.#ipAddress}:17246`,
        "--key-passphrase",
        this.passphrase,
        "--skip-remote-helper-install",
        "--unsafe-fast-keystore",
        "--insecure-http-api",
        "--dev-log",
        "--seed",
        this.#seed,
      ],
      {
        RAD_HOME: this.radHome,
      }
    );

    processes.push(this.#childProcess);
  }

  public async stop(): Promise<void> {
    if (!this.#childProcess) {
      throw new Error("Tried to stop() process that wasn't started.");
    }

    this.#childProcess.kill("SIGTERM");
    await this.#childProcess;
    this.#childProcess = undefined;
  }
}

interface UpstreamSeedParams {
  dataPath: string;
  ipAddress: string;
  name: string;
  project: string;
}

export class UpstreamSeed {
  public listen: string;
  public name: string;
  public peerId: string;
  public radHome: string;
  public seedAddress: string;

  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #project: string;

  public constructor({
    name,
    ipAddress,
    dataPath,
    project,
  }: UpstreamSeedParams) {
    this.#project = project;
    this.listen = `${ipAddress}:8776`;
    this.name = name;
    this.peerId = "hybfoqx9wrdjhnr9jyb74zpduph57z99f67bjgfnsf83p1rk7z1diy";
    this.radHome = path.join(dataPath, `${name}-rad-home`);
    this.seedAddress = `${this.peerId}@${this.listen}`;

    fs.mkdirsSync(this.radHome);
  }

  public start(): void {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    this.#childProcess = spawnInNamespace(
      this.name,
      [
        path.join(binPath(), "upstream-seed"),
        "--rad-home",
        this.radHome,
        "--listen",
        this.listen,
        "--identity-key",
        path.join(P2P_TEST_PATH, "keys", `seed-${this.peerId}.key`),
        "--project",
        this.#project,
      ],
      {}
    );

    processes.push(this.#childProcess);
  }

  public async stop(): Promise<void> {
    if (!this.#childProcess) {
      throw new Error("Tried to stop() process that wasn't started.");
    }

    this.#childProcess.kill("SIGTERM");
    await this.#childProcess;
    this.#childProcess = undefined;
  }
}

function spawnInNamespace(
  name: string,
  args: string[],
  env: NodeJS.ProcessEnv
): execa.ExecaChildProcess {
  const subprocess = execa(
    "ip",
    ["netns", "exec", `upstream-test-${name}`, ...args],
    {
      env,
    }
  );

  const stdoutPrefix = new LinePrefix(prefix(name));
  const stderrPrefix = new LinePrefix(prefix(name));

  if (subprocess.stdout) {
    subprocess.stdout.pipe(stdoutPrefix).pipe(process.stdout);
  }
  if (subprocess.stderr) {
    subprocess.stderr.pipe(stderrPrefix).pipe(process.stderr);
  }

  return subprocess;
}

interface CommitParams {
  author: string;
  checkoutPath: string;
}

export function commit({ author, checkoutPath }: CommitParams): void {
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

export function getLatestCommitSha(checkoutPath: string): string {
  return execa
    .sync("git", ["rev-parse", "HEAD"], {
      cwd: checkoutPath,
    })
    .stdout.trim();
}

interface PushRadParams {
  radHome: string;
  checkoutPath: string;
  keyPassphrase: string;
}

export function pushRad({
  radHome,
  checkoutPath,
  keyPassphrase,
}: PushRadParams): void {
  execa.sync("git", ["push", "rad"], {
    cwd: checkoutPath,
    env: {
      RADICLE_UNSAFE_FAST_KEYSTORE: "1",
      RAD_HOME: radHome,
      KEY_PASSPHRASE: keyPassphrase,
      GIT_EXEC_PATH: binPath(),
    },
  });
}

export async function withRetry(
  action: () => Promise<unknown>
): Promise<unknown> {
  return await retryOnError(action, () => true, 1000, 10);
}

interface RunTestcaseParams {
  testcase: (dataDirPath: string) => Promise<void>;
  networkScript: string;
  dataDirName: string;
}

export async function runTestcase({
  testcase,
  networkScript,
  dataDirName,
}: RunTestcaseParams): Promise<void> {
  const scriptPath = path.join(P2P_TEST_PATH, networkScript);

  execa.commandSync(`${scriptPath} start`, { stdio: "inherit" });

  const testDataDir = path.join(P2P_TEST_PATH, "workspace", dataDirName);
  fs.removeSync(testDataDir);

  const maybeError: Error | void = await testcase(testDataDir).catch(
    err => err
  );
  if (maybeError) {
    console.log("\nTEST FAILED ❌\n");
    console.log(maybeError);
  } else {
    console.log("\nTEST PASSED ✅\n");
    fs.removeSync(testDataDir);
  }

  execa.commandSync(`${scriptPath} stop`, { stdio: "inherit" });

  if (maybeError) {
    process.exit(1);
  }

  process.exit(0);
}

interface RadCliParams {
  radHome: string;
  args: string[];
}

export function radCli({ radHome, args }: RadCliParams): unknown {
  const radBinaryPath = path.join(binPath(), "rad");
  const result = execa.sync(radBinaryPath, args, {
    env: {
      RAD_HOME: radHome,
    },
  });

  try {
    return JSON.parse(result.stdout);
  } catch {
    throw new Error(`Couldn't parse rad cli output: ${result.stdout}`);
  }
}
