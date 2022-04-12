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

const ROOT_PATH = path.join(__dirname, "..", "..", "..");
const CARGO_TARGET_DIR =
  process.env.CARGO_TARGET_DIR ?? path.join(ROOT_PATH, "target");
const BIN_PATH = path.join(CARGO_TARGET_DIR, "debug");

interface RadicleProxyParams {
  dataPath: string;
  // IP address to bind to. Defaults to 127.0.0.1
  ipAddress?: string;
  name: string;
  gitSeeds?: string[];
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
  #gitSeeds: string[] | undefined;

  public constructor({
    dataPath,
    ipAddress,
    name,
    gitSeeds,
  }: RadicleProxyParams) {
    this.#ipAddress = ipAddress ?? "127.0.0.1";
    this.#gitSeeds = gitSeeds;
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
      "--git-fetch-interval=1",
    ];

    for (const gitSeed of this.#gitSeeds || []) {
      args.push("--git-seed", gitSeed);
    }

    const env = {
      LNK_HOME: this.lnkHome,
    };

    this.#childProcess = Process.spawn(bin, args, { env });

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
