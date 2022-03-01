// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import fetch from "node-fetch";
import EventSource from "eventsource";
import waitOn from "wait-on";
import * as ProxyClient from "proxy-client";
import * as fs from "fs-extra";
import * as path from "path";
import execa from "execa";

import { retryOnError } from "ui/src/retryOnError";

import * as Process from "./process";

export { killAllProcesses } from "./process";

const ROOT_PATH = path.join(__dirname, "..", "..");
const P2P_TEST_PATH = path.join(ROOT_PATH, "p2p-tests");
const CARGO_TARGET_DIR =
  process.env.CARGO_TARGET_DIR ?? path.join(ROOT_PATH, "target");
const BIN_PATH = path.join(CARGO_TARGET_DIR, "debug");

interface RadicleProxyParams {
  dataPath: string;
  // IP address to bind to. Defaults to 127.0.0.1
  ipAddress?: string;
  name: string;
  // Address of a seed node to connect to
  seed?: string;
  // If true, run the proxy in a network namespace. Defaults to `true`.
  networkNamespace?: boolean;
}

export class RadicleProxy {
  public checkoutPath: string;
  public identityUrn: string;
  public name: string;
  public passphrase: string;
  public peerId: string;
  public proxyClient: ProxyClient.ProxyClient;
  public lnkHome: string;

  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #ipAddress: string;
  #seed: string | undefined;
  #networkNamespace: boolean;

  public constructor({
    dataPath,
    ipAddress,
    name,
    seed,
    networkNamespace,
  }: RadicleProxyParams) {
    this.#ipAddress = ipAddress ?? "127.0.0.1";
    this.#seed = seed;
    this.#networkNamespace = networkNamespace ?? true;
    this.name = name;
    this.passphrase = name;

    this.checkoutPath = path.join(dataPath, `${name}-checkouts`);
    this.lnkHome = path.join(dataPath, `${name}-lnk-home`);

    fs.mkdirsSync(this.lnkHome);

    const initResult = JSON.parse(
      execa.sync(path.join(BIN_PATH, "upstream-proxy-dev"), [
        "--lnk-home",
        this.lnkHome,
        "init",
        this.name,
        "--key-passphrase",
        this.passphrase,
      ]).stdout
    );

    this.identityUrn = initResult.identityUrn;
    this.peerId = initResult.peerId;

    this.proxyClient = new ProxyClient.ProxyClient(
      `http://${this.#ipAddress}:30000`,
      fetch,
      EventSource
    );
  }

  public async start(): Promise<void> {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    const bin = path.join(BIN_PATH, "upstream-proxy");
    const httpSocketAddr = `${this.#ipAddress}:30000`;
    const args = [
      "--peer-listen",
      `${this.#ipAddress}:8776`,
      "--http-listen",
      httpSocketAddr,
      "--key-passphrase",
      this.passphrase,
      "--unsafe-fast-keystore",
      "--dev-log",
    ];

    if (this.#seed) {
      args.push("--seed", this.#seed);
    }

    const env = {
      LNK_HOME: this.lnkHome,
    };
    if (this.#networkNamespace) {
      this.#childProcess = Process.spawnInNamespace(this.name, [bin, ...args], {
        env,
      });
    } else {
      this.#childProcess = Process.spawn(bin, args, { env });
    }

    Process.prefixOutput(this.#childProcess, this.name);

    await waitOn({ resources: [`tcp:${httpSocketAddr}`], timeout: 7000 });
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
  public lnkHome: string;
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
    this.lnkHome = path.join(dataPath, `${name}-lnk-home`);
    this.seedAddress = `${this.peerId}@${this.listen}`;

    fs.mkdirsSync(this.lnkHome);
  }

  public start(): void {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    this.#childProcess = Process.spawnInNamespace(this.name, [
      path.join(BIN_PATH, "upstream-seed"),
      "--lnk-home",
      this.lnkHome,
      "--listen",
      this.listen,
      "--identity-key",
      path.join(P2P_TEST_PATH, "keys", `seed-${this.peerId}.key`),
      "--project",
      this.#project,
    ]);

    Process.prefixOutput(this.#childProcess, this.name);
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
  lnkHome: string;
  checkoutPath: string;
  keyPassphrase: string;
}

export function pushRad({
  lnkHome,
  checkoutPath,
  keyPassphrase,
}: PushRadParams): void {
  execa.sync("git", ["push", "rad"], {
    cwd: checkoutPath,
    env: {
      RADICLE_UNSAFE_FAST_KEYSTORE: "1",
      LNK_HOME: lnkHome,
      KEY_PASSPHRASE: keyPassphrase,
      GIT_EXEC_PATH: BIN_PATH,
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

interface LnkCliParams {
  lnkHome: string;
  args: string[];
}

export function lnkCli({ lnkHome, args }: LnkCliParams): unknown {
  const radBinaryPath = path.join(BIN_PATH, "lnk");
  const result = execa.sync(radBinaryPath, args, {
    env: {
      LNK_HOME: lnkHome,
    },
  });

  try {
    return JSON.parse(result.stdout);
  } catch {
    throw new Error(`Couldn't parse rad cli output: ${result.stdout}`);
  }
}
