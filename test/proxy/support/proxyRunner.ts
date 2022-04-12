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
  httpPort?: number;
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
  #httpSocketAddr: string;

  public constructor({
    dataPath,
    ipAddress,
    name,
    gitSeeds,
    httpPort,
  }: RadicleProxyParams) {
    this.#ipAddress = ipAddress ?? "127.0.0.1";
    this.#gitSeeds = gitSeeds;
    this.name = name;
    this.passphrase = name;
    this.#httpSocketAddr = `${this.#ipAddress}:${httpPort ?? 3000}`;

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
      `http://${this.#httpSocketAddr}`,
      fetch,
      EventSource
    );
  }

  public async start(): Promise<void> {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    const bin = path.join(BIN_PATH, "upstream-proxy");
    const args = [
      "--http-listen",
      this.#httpSocketAddr,
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
}
