// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Crypto from "node:crypto";
import fetch from "node-fetch";
import EventSource from "eventsource";
import waitOn from "wait-on";
import * as Jest from "@jest/globals";
import * as ProxyClient from "proxy-client";
import * as fs from "node:fs/promises";
import * as path from "path";
import execa from "execa";
import getPort from "get-port";

import * as Process from "./process";

export { killAllProcesses } from "./process";

const ROOT_PATH = path.join(__dirname, "..", "..", "..");
const CARGO_TARGET_DIR =
  process.env.CARGO_TARGET_DIR ?? path.join(ROOT_PATH, "target");
const BIN_PATH = path.join(CARGO_TARGET_DIR, "debug");
const PATH = [BIN_PATH, process.env.PATH].join(path.delimiter);

export const SEED_URL = "http://127.0.0.1:8778";

interface RadicleProxyParams {
  dataPath: string;
  // Name to quickly identify this peer.
  //
  // Used as a prefix for directories, as a prefix for the user
  // handle, and as a prefix for the console logs.
  name: string;
  sshAuthSock?: string;
}

// Registers a test hook that ensures that `upstream-proxy` is built.
// We skip this hook if the `CI` environment variable is "true".
export function buildBeforeAll(): void {
  if (process.env.CI === "true") {
    return;
  }

  Jest.beforeAll(async () => {
    // Because we’re using `--quiet` to build the proxy we want to show
    // some progress to the user. But only after some initial delay so
    // that we don’t show it when we don’t need to rebuild or the rebuild
    // is quick.
    const notifyTimeout = setTimeout(() => {
      // We’re not using `console.log()` because it is patched by Jest
      process.stdout.write("Building upstream-proxy...");
    }, 3000);
    await execa("cargo", ["build", "--bin", "upstream-proxy", "--quiet"], {
      stdio: "inherit",
    });
    clearTimeout(notifyTimeout);
  }, 10 * 60 * 1000);
}

export class RadicleProxy {
  public checkoutPath: string;
  public peerId: string;
  public proxyClient: ProxyClient.ProxyClient;

  #name: string;
  #lnkHome: string;
  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #httpSocketAddr: string;
  #sshAuthSock: string;

  public static async create({
    dataPath,
    name,
    sshAuthSock = "/dev/null",
  }: RadicleProxyParams): Promise<RadicleProxy> {
    const httpPort = await getPort();
    const httpSocketAddr = `127.0.0.1:${httpPort}`;

    const checkoutPath = path.join(dataPath, `${name}-checkouts`);
    await fs.mkdir(checkoutPath, { recursive: true });

    const lnkHome = path.join(dataPath, `${name}-lnk-home`);
    await fs.mkdir(lnkHome, { recursive: true });

    // We need a random user handle so that the Radicle identity IDs
    // are different between runs
    const userHandle = `${name}-${randomTag()}`;

    const initResult = await execa(
      path.join(BIN_PATH, "upstream-proxy-dev"),
      ["--lnk-home", lnkHome, "init", userHandle, "--key-passphrase", "asdf"],
      { env: { SSH_AUTH_SOCK: sshAuthSock } }
    );
    const peerId = JSON.parse(initResult.stdout).peerId;

    const proxyClient = new ProxyClient.ProxyClient(
      `http://${httpSocketAddr}`,
      fetch,
      EventSource
    );

    return new RadicleProxy({
      checkoutPath,
      peerId,
      proxyClient,
      name,
      lnkHome,
      httpSocketAddr,
      sshAuthSock,
    });
  }

  private constructor(props: {
    checkoutPath: string;
    peerId: string;
    proxyClient: ProxyClient.ProxyClient;

    name: string;
    lnkHome: string;
    httpSocketAddr: string;
    sshAuthSock: string;
  }) {
    this.checkoutPath = props.checkoutPath;
    this.peerId = props.peerId;
    this.proxyClient = props.proxyClient;

    this.#name = props.name;
    this.#lnkHome = props.lnkHome;
    this.#httpSocketAddr = props.httpSocketAddr;
    this.#sshAuthSock = props.sshAuthSock;
  }

  public async start(): Promise<void> {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    const bin = path.join(BIN_PATH, "upstream-proxy");
    const args = [
      "--http-listen",
      this.#httpSocketAddr,
      "--unsafe-fast-keystore",
      "--dev-log",
      "--git-fetch-interval=1",
      "--git-seed",
      SEED_URL,
    ];

    const env = {
      LNK_HOME: this.#lnkHome,
      SSH_AUTH_SOCK: this.#sshAuthSock,
    };

    this.#childProcess = Process.spawn(bin, args, { env });

    Process.prefixOutput(this.#childProcess, this.#name);

    await waitOn({ resources: [`tcp:${this.#httpSocketAddr}`], timeout: 7000 });
  }

  public async stop(): Promise<void> {
    if (!this.#childProcess) {
      throw new Error("Tried to stop() process that wasn't started.");
    }

    this.#childProcess.kill("SIGTERM");
    await this.#childProcess;
    this.#childProcess = undefined;
  }

  // Spawn a process with an environment configured for this proxy
  // instance.
  //
  // In particular, the `LNK_HOME`, `SSH_AUTH_SOCK` and `GIT_*`
  // environment variables are set appropriately.
  public spawn(
    cmd: string,
    args: string[] = [],
    opts: execa.Options = {}
  ): execa.ExecaChildProcess {
    const defaultOpts = {
      LNK_HOME: this.#lnkHome,
      SSH_AUTH_SOCK: this.#sshAuthSock,
      GIT_CONFIG_GLOBAL: "/dev/null",
      GIT_CONFIG_NOSYSTEM: "1",
      GIT_AUTHOR_NAME: "John Doe",
      GIT_AUTHOR_EMAIL: "john@example.com",
      GIT_COMMITTER_NAME: "John Doe",
      GIT_COMMITTER_EMAIL: "john@example.com",
      PATH,
    } as Record<string, string>;
    opts = {
      ...opts,
      env: {
        ...defaultOpts,
        ...opts.env,
      },
    };
    return Process.prefixOutput(
      Process.spawn(cmd, args, opts),
      `${this.#name}-shell`
    );
  }
}

// Generate string of 12 random characters with 8 bits of entropy.
function randomTag(): string {
  return Crypto.randomBytes(8).toString("hex");
}
