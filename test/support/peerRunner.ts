// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Config } from "ui/src/config";

import * as Crypto from "node:crypto";
import * as ProxyClient from "proxy-client";
import * as fs from "node:fs/promises";
import * as path from "path";
import EventSource from "eventsource";
import Polka from "polka";
import Sirv from "sirv";
import execa from "execa";
import fetch from "node-fetch";
import getPort from "get-port";
import qs from "qs";
import waitOn from "wait-on";
import { Server } from "http";

import * as Process from "./process";

export { killAllProcesses } from "./process";

const ROOT_PATH = path.join(__dirname, "..", "..");
const CARGO_TARGET_DIR =
  process.env.CARGO_TARGET_DIR ?? path.join(ROOT_PATH, "target");
const BIN_PATH = path.join(CARGO_TARGET_DIR, "debug");
const PATH = [BIN_PATH, process.env.PATH].join(path.delimiter);

export const SEED_URL = "http://127.0.0.1:8778";

interface UpstreamPeerParams {
  dataPath: string;
  // Name to quickly identify this peer.
  //
  // Used as a prefix for directories, as a prefix for the user
  // handle, and as a prefix for the console logs.
  name: string;
  sshAuthSock?: string;
}

// Builds the `upstream-proxy` when this is run locally, we skip the build if
// the `CI` environment variable is "true".
export async function buildProxy(): Promise<void> {
  if (process.env.CI === "true") {
    return;
  }

  // Because we’re using `--quiet` to build the proxy we want to show
  // some progress to the user. But only after some initial delay so
  // that we don’t show it when we don’t need to rebuild or the rebuild
  // is quick.
  const notifyTimeout = setTimeout(() => {
    // We’re not using `console.log()` because it is patched by Jest
    process.stdout.write("Building upstream-proxy…");
  }, 3000);
  await execa("cargo", ["build", "--bin", "upstream-proxy", "--quiet"], {
    stdio: "inherit",
  });
  clearTimeout(notifyTimeout);
}

export async function buildUi(): Promise<void> {
  if (process.env.CI === "true") {
    return;
  }

  // Because we’re using `--quiet` to build the proxy we want to show
  // some progress to the user. But only after some initial delay so
  // that we don’t show it when we don’t need to rebuild or the rebuild
  // is quick.
  const notifyTimeout = setTimeout(() => {
    console.log("Building UI…");
  }, 3000);
  await execa("webpack", ["build", "--config-name", "ui", "--progress"], {
    stdio: "inherit",
  });
  clearTimeout(notifyTimeout);
}

export class UpstreamPeer {
  public checkoutPath: string;
  public peerId: string;
  public proxyClient: ProxyClient.ProxyClient;

  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #httpSocketAddr: string;
  #lnkHome: string;
  #name: string;
  #sshAuthSock: string;
  #uiServer?: Server;
  #uiUrl?: string;

  public get uiUrl(): string {
    if (this.#uiUrl) {
      return this.#uiUrl;
    } else {
      throw new Error("Trying to access UI before it is started.");
    }
  }

  public static async create({
    dataPath,
    name,
    sshAuthSock = "/dev/null",
  }: UpstreamPeerParams): Promise<UpstreamPeer> {
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

    return new UpstreamPeer({
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

    httpSocketAddr: string;
    lnkHome: string;
    name: string;
    sshAuthSock: string;
  }) {
    this.checkoutPath = props.checkoutPath;
    this.peerId = props.peerId;
    this.proxyClient = props.proxyClient;

    this.#httpSocketAddr = props.httpSocketAddr;
    this.#lnkHome = props.lnkHome;
    this.#name = props.name;
    this.#sshAuthSock = props.sshAuthSock;
  }

  public async startProxy(): Promise<void> {
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

  public async stopProxy(): Promise<void> {
    if (!this.#childProcess) {
      throw new Error(
        "Tried to run stopProxy() on a process that wasn't started."
      );
    }

    this.#childProcess.kill("SIGTERM");
    await this.#childProcess;
    this.#childProcess = undefined;
  }

  private async startUi(): Promise<void> {
    const uiPort = (await getPort()).toString();

    const uiConfig: Config = {
      proxyAddress: this.#httpSocketAddr,
      isDev: true,
    };

    const query = qs.stringify({
      config: JSON.stringify(uiConfig),
    });

    this.#uiUrl = `http://127.0.0.1:${uiPort}/?${query}`;

    const polka = Polka().use(Sirv("public"));

    await new Promise<void>((resolve, reject) => {
      polka.listen(uiPort, (err: unknown) => {
        if (err) {
          reject(err);
        } else {
          resolve();
        }
      });
    });

    console.log(`UI ready at ${this.#uiUrl}`);
    this.#uiServer = polka.server;
  }

  private async stopUi(): Promise<void> {
    await new Promise<void>((resolve, reject) => {
      if (this.#uiServer) {
        this.#uiServer.close(err => {
          if (err) {
            reject(err);
          } else {
            resolve();
          }
        });
      } else {
        reject(
          new Error("Tried to run stopUi() on a process that wasn't started.")
        );
      }
    });

    this.#uiServer = undefined;
  }

  public async start(): Promise<void> {
    await this.startProxy();
    await this.startUi();
  }

  public async stop(): Promise<void> {
    await this.stopUi();
    await this.stopProxy();
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
