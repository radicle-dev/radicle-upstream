// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as stream from "stream";
import { StringDecoder } from "string_decoder";
import * as path from "path";
import exitHook from "exit-hook";
import fetch, { FetchError } from "node-fetch";
import waitOn from "wait-on";
import * as fs from "fs-extra";
import execa from "execa";
import * as cookie from "cookie";

import { retryOnError } from "ui/src/retryOnError";

import type {
  ConnectNodeOptions,
  NodeId,
  NodeManagerPlugin,
  NodeSession,
  OnboardNodeOptions,
  PeerId,
} from "./shared";
import { pluginMethods } from "./shared";

type PeerAddress = string;
type AuthToken = string;

const ROOT_PATH = path.join(__dirname, "../../../");

const PROXY_BINARY_PATH = path.join(ROOT_PATH, "target/debug/radicle-proxy");

// IP to which all started processes will bind to.
const HOST = "127.0.0.1";

class Logger {
  #prefix: string;
  #indentationLevel: number;

  public constructor({ prefix = "", indentationLevel = 1 }) {
    this.#prefix = prefix;
    this.#indentationLevel = indentationLevel;
  }

  public log(message: string) {
    const indentation = " ".repeat(this.#indentationLevel * 2);
    console.log(indentation + this.#prefix + message);
  }
}

enum StateKind {
  Configured = "configured",
  Started = "started",
  Onboarded = "onboarded",
}

// Because it's not possible to mix tagged union types and extending
// interfaces, we have to repeat the "inherited" attributes in each node state.
interface ConfiguredNode {
  kind: StateKind.Configured;
}

interface StartedNode {
  kind: StateKind.Started;
  process: execa.ExecaChildProcess;
}

interface OnboardedNode {
  kind: StateKind.Onboarded;
  process: execa.ExecaChildProcess;
  authToken: AuthToken;
  peerAddress: PeerAddress;
  peerId: PeerId;
}

type NodeState = ConfiguredNode | StartedNode | OnboardedNode;

class Node {
  private state: NodeState = { kind: StateKind.Configured };
  private logger: Logger;
  private dataDir: string;

  public id: NodeId;
  public httpPort: number;
  public peerPort: number;
  public radHome: string;

  public get authToken(): AuthToken {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.authToken;
  }

  public get peerAddress(): PeerAddress {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.peerAddress;
  }

  public get peerId(): PeerId {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.peerId;
  }

  public get currentState(): StateKind {
    return this.state.kind;
  }

  public constructor(id: NodeId, dataDir: string) {
    this.dataDir = dataDir;
    this.logger = new Logger({
      prefix: `[node ${id}]: `,
      indentationLevel: 2,
    });

    this.id = id;
    this.httpPort = id;
    this.peerPort = id;
    this.radHome = path.resolve(dataDir, `node-${id}`);
  }

  public async start() {
    this.logger.log(`starting node`);

    await fs.mkdirs(this.radHome);

    const process = execa(
      PROXY_BINARY_PATH,
      [
        "--http-listen",
        `${HOST}:${this.httpPort}`,
        "--peer-listen",
        `${HOST}:${this.peerPort}`,
        "--skip-remote-helper-install",
        "--unsafe-fast-keystore",
      ],
      {
        buffer: false,
        env: {
          RAD_HOME: this.radHome,
          RUST_LOG: [
            "info",
            "api=debug",
            "librad=debug",
            "radicle_daemon=debug",
            // The following modules generate a lot of noise at the
            // info level
            "librad::net::protocol::io::streams=warn",
            "librad::git::p2p::server=warn",
          ].join(","),
        },
        stdio: ["ignore", "inherit", "pipe"],
      }
    );

    process.on("exit", async () => {
      this.logger.log(`node terminated`);
    });

    const stderrLogPath = path.join(this.radHome, "stderr.log");
    this.logger.log(`writing output to ${stderrLogPath}`);
    const stderrLog = fs.createWriteStream(stderrLogPath);
    // We know that `stderr` is set because of the `stdio` spawn options
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const stderr = process.stderr!;
    stderr.pipe(stderrLog);

    const combinedLogPath = path.join(this.dataDir, "combined-node.log");
    const combinedLog = fs.createWriteStream(combinedLogPath, { flags: "a" });
    const prependNodeId = new LinePrefix(`Node ${this.id}: `);

    stderr.pipe(prependNodeId).pipe(combinedLog);

    this.state = { ...this.state, kind: StateKind.Started, process: process };

    await waitOn({ resources: [`tcp:${HOST}:${this.httpPort}`] });

    this.logger.log("node started successfully");
  }

  public async onboard(options: {
    handle: string;
    passphrase: string;
  }): Promise<NodeSession> {
    this.logger.log("onboarding node");

    if (this.state.kind !== StateKind.Started) {
      throw new Error("Tried to onboard a node that wasn't started yet");
    }

    const gitConfigSet = (name: string, value: string) =>
      execa("git", ["config", "--global", name, value], {
        env: { HOME: this.radHome },
      });

    await gitConfigSet(
      "credential.helper",
      `!f() { test "$1" = get && echo "password=${options.passphrase}"; }; f`
    );
    await gitConfigSet("user.name", options.handle);
    await gitConfigSet("user.email", `${options.handle}@example.com`);

    const keystoreResponse = await fetch(
      `http://${HOST}:${this.id}/v1/keystore`,
      {
        method: "post",
        body: JSON.stringify({ passphrase: options.passphrase }),
        headers: { "Content-Type": "application/json" },
      }
    );

    if (!keystoreResponse) {
      throw new Error("No response from keystore request");
    }

    const cookieData = keystoreResponse.headers.get("set-cookie");
    const cookies = cookie.parse(cookieData || "");
    const authToken = cookies["auth-token"];
    if (!authToken) {
      throw new Error("Response did not contain an auth cookie");
    }

    const identitiesResponse = await retryOnError(
      () =>
        fetch(`http://${HOST}:${this.id}/v1/identities`, {
          method: "post",
          body: JSON.stringify({ handle: options.handle }),
          headers: {
            Cookie: `auth-token=${authToken}`,
            "Content-Type": "application/json",
          },
        }),
      err => err instanceof FetchError && err.code === "ECONNREFUSED",
      25,
      40
    );
    const json = await identitiesResponse.json();
    const peerId = json.peerId;

    this.state = {
      ...this.state,
      kind: StateKind.Onboarded,
      authToken: authToken,
      peerAddress: `${json.peerId}@${HOST}:${this.peerPort}`,
      peerId,
    };

    this.logger.log("node onboarded successfully");

    return {
      id: this.id,
      authToken,
      httpPort: this.httpPort,
      radHome: this.radHome,
      peerId,
    };
  }

  public async stop(): Promise<void> {
    if (this.state.kind !== StateKind.Configured) {
      this.logger.log("stopping node");
      // We don’t shutdown the process properly to make it faster. We don’t care
      // about corrupted state because it has no impact on the test.
      if (!this.state.process.kill("SIGKILL")) {
        this.logger.log(`could not stop process ${this.state.process.pid}`);
      }
    } else {
      this.logger.log("ignoring stop node command, node wasn't running");
    }
  }
}

class NodeManager implements NodeManagerPlugin {
  private managedNodes: Node[] = [];
  private logger: Logger;
  // A radicle-proxy is running on port 30000 for any other Cypress tests
  // that aren't managed by nodeManager.
  private nextPort: number = 30001;

  public constructor() {
    this.logger = new Logger({ prefix: `[nodeManager] ` });
  }

  private getNode(id: NodeId) {
    const node = this.managedNodes.find(node => {
      return node.id === id;
    });

    if (!node) {
      throw new Error(`Could not find node by id ${id}`);
    }

    return node;
  }

  public async startNode(dataDir: string): Promise<number> {
    const id = this.nextPort++;
    const node = new Node(id, dataDir);
    await node.start();
    this.managedNodes.push(node);

    return id;
  }

  public async onboardNode(options: OnboardNodeOptions): Promise<NodeSession> {
    this.logger.log("onboardNode");

    const node = this.getNode(options.id);

    return node.onboard({
      handle: options.handle,
      passphrase: options.passphrase,
    });
  }

  public async connectNodes(options: ConnectNodeOptions): Promise<null> {
    this.logger.log("connectNodes");

    if (options.nodeIds.length < 2) {
      throw new Error("Supply at least 2 node IDs");
    }

    this.managedNodes.forEach(node => {
      if (node.currentState !== StateKind.Onboarded) {
        throw new Error("Can't connect nodes that are not onboarded");
      }
    });

    const firstNode = this.getNode(options.nodeIds[0]);
    const remainingNodes = this.managedNodes.filter(node => {
      return firstNode.id !== node.id;
    });

    await fetch(`http://${HOST}:${firstNode.httpPort}/v1/session/seeds`, {
      method: "PUT",
      body: JSON.stringify(remainingNodes.map(node => node.peerAddress)),
      headers: {
        Cookie: `auth-token=${firstNode.authToken}`,
        "Content-Type": "application/json",
      },
    });

    return null;
  }

  public async stopAllNodes(): Promise<null> {
    this.logger.log("stopAllNodes");

    await Promise.all(this.managedNodes.map(node => node.stop()));

    this.managedNodes = [];

    // A radicle-proxy is running on port 30000 for any other Cypress tests
    // that aren't managed by nodeManager.
    this.nextPort = 30001;

    return null;
  }
}

const nodeManager = new NodeManager();

function createNodeManagerPlugin(plugin: NodeManagerPlugin): NodeManagerPlugin {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const nodeManagerPlugin: any = {};
  pluginMethods.forEach(method => {
    nodeManagerPlugin[method] = plugin[method].bind(plugin);
  });
  return nodeManagerPlugin;
}

export const nodeManagerPlugin = createNodeManagerPlugin(nodeManager);

// Clean up any lingering radicle-proxy processes when closing Cypress.
exitHook(() => {
  nodeManager.stopAllNodes();
});

// A transform that prefixes each line from the source with the given
// string and pushes it to the sink.
class LinePrefix extends stream.Transform {
  private buffer: string = "";
  private stringDecoder = new StringDecoder();
  public constructor(private prefix: string) {
    super();
  }

  public _transform(data: Buffer, _encoding: string, next: () => void) {
    const str = this.buffer + this.stringDecoder.write(data);
    const lines = str.split(/\r?\n/);
    this.buffer = lines.pop() || "";
    lines.forEach(line => this.push(`${this.prefix}${line}\n`));
    next();
  }

  public _flush(done: () => void) {
    this.push(`${this.prefix}${this.buffer}${this.stringDecoder.end()}\n`);
    done();
  }
}
