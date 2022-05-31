// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Config } from "ui/src/config";

import * as Fs from "node:fs/promises";
import * as Os from "node:os";
import * as Path from "node:path";
import EventSource from "eventsource";
import execa from "execa";
import fetch from "node-fetch";
import getPort from "get-port";
import qs from "qs";
import waitOn from "wait-on";

import * as ProxyClient from "proxy-client";

import * as Process from "./process";
import { randomTag } from "../support";

const ROOT_PATH = Path.join(__dirname, "..", "..");
const CARGO_TARGET_DIR =
  process.env.CARGO_TARGET_DIR ?? Path.join(ROOT_PATH, "target");
const BIN_PATH = Path.join(CARGO_TARGET_DIR, "debug");
const PATH = [BIN_PATH, process.env.PATH].join(Path.delimiter);

export const SEED_URL = "http://127.0.0.1:8778";
export const UI_PORT = 30002;

interface StartPeerParams {
  // Name to quickly identify this peer.
  //
  // Used as a prefix for directories, as a prefix for the user
  // handle, and as a prefix for the console logs.
  name: string;
  disableSshAgent?: boolean;
}

export interface PeerManager {
  startPeer(params: StartPeerParams): Promise<UpstreamPeer>;
  teardown(): Promise<void>;
}

export async function createPeerManager({
  dataPath,
}: {
  dataPath: string;
}): Promise<PeerManager> {
  const { sshAuthSock, process: sshAgentProcess } = await startSshAgent();

  const peers: UpstreamPeer[] = [];
  return {
    async startPeer(params: StartPeerParams): Promise<UpstreamPeer> {
      const peer = await UpstreamPeer.create({
        name: params.name,
        dataPath,
        sshAuthSock:
          params.disableSshAgent === true ? "/dev/null" : sshAuthSock,
      });
      peers.push(peer);
      await peer.start();
      return peer;
    },

    async teardown(): Promise<void> {
      for (const peer of peers) {
        await peer.shutdown();
      }
      sshAgentProcess.kill("SIGKILL");
    },
  };
}

interface UpstreamPeerParams {
  dataPath: string;
  name: string;
  sshAuthSock: string;
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

export class UpstreamPeer {
  public checkoutPath: string;
  public peerId: string;
  public proxyClient: ProxyClient.ProxyClient;
  public userHandle: string;

  #childProcess: execa.ExecaChildProcess | undefined = undefined;
  #httpSocketAddr: string;
  #lnkHome: string;
  #name: string;
  #sshAuthSock: string;

  public uiUrl(config: { fakeClock?: boolean } = {}): string {
    const uiConfig: Partial<Config> = {
      ...config,
      proxyAddress: this.#httpSocketAddr,
      isDev: true,
      e2eTest: true,
    };

    const query = qs.stringify({
      config: JSON.stringify(uiConfig),
    });

    return `http://127.0.0.1:${UI_PORT}/?${query}`;
  }

  public static async create({
    dataPath,
    name,
    sshAuthSock,
  }: UpstreamPeerParams): Promise<UpstreamPeer> {
    const httpPort = await getPort();
    const httpSocketAddr = `127.0.0.1:${httpPort}`;

    const checkoutPath = Path.join(dataPath, `${name}-checkouts`);
    await Fs.mkdir(checkoutPath, { recursive: true });

    const lnkHome = Path.join(dataPath, `${name}-lnk-home`);
    await Fs.mkdir(lnkHome, { recursive: true });

    // We need a random user handle so that the Radicle identity IDs
    // are different between runs
    const userHandle = `${name}-${randomTag()}`;

    const initResult = await execa(
      Path.join(BIN_PATH, "upstream-proxy-dev"),
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
      userHandle,
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
    userHandle: string;
  }) {
    this.checkoutPath = props.checkoutPath;
    this.peerId = props.peerId;
    this.proxyClient = props.proxyClient;
    this.userHandle = props.userHandle;

    this.#httpSocketAddr = props.httpSocketAddr;
    this.#lnkHome = props.lnkHome;
    this.#name = props.name;
    this.#sshAuthSock = props.sshAuthSock;
  }

  public async start(): Promise<void> {
    if (this.#childProcess) {
      throw new Error("Tried to start a process that already was running.");
    }

    const bin = Path.join(BIN_PATH, "upstream-proxy");
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

  public async shutdown(): Promise<void> {
    if (this.#childProcess) {
      // We send SIGKILL instead of SIGTERM so the proxy doest not do
      // any cleanup and shuts down faster.
      this.#childProcess.kill("SIGKILL");
      this.#childProcess = undefined;
    }
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

async function startSshAgent(): Promise<{
  sshAuthSock: string;
  process: execa.ExecaChildProcess;
}> {
  // We’re not using the state directory because of the size limit on
  // the socket path.
  const dir = await Fs.mkdtemp(Path.join(Os.tmpdir(), "upstream-test"));
  const sshAuthSock = Path.join(dir, "ssh-agent.sock");
  const process = Process.spawn("ssh-agent", ["-D", "-a", sshAuthSock], {
    stdio: "inherit",
  });
  await waitOn({ resources: [sshAuthSock], timeout: 5000 });
  return { sshAuthSock, process };
}
