import exitHook from "exit-hook";
import * as path from "path";
import * as childProcess from "child_process";
import fetch from "node-fetch";
import waitOn from "wait-on";
import * as fs from "fs-extra";

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

const PROXY_BINARY_PATH = path.join(ROOT_PATH, "target/release/radicle-proxy");

// IP to which all started processes will bind to.
const HOST = "127.0.0.1";

const sleep = async (ms: number) => {
  await new Promise(resolve => setTimeout(resolve, ms));
};

class Logger {
  prefix: string;
  indentationLevel: number;

  constructor({ prefix = "", indentationLevel = 1 }) {
    this.prefix = prefix;
    this.indentationLevel = indentationLevel;
  }

  log(message: string) {
    const indentation = " ".repeat(this.indentationLevel * 2);
    console.log(indentation + this.prefix + message);
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
  process: childProcess.ChildProcess;
}

interface OnboardedNode {
  kind: StateKind.Onboarded;
  process: childProcess.ChildProcess;
  authToken: AuthToken;
  peerAddress: PeerAddress;
  peerId: PeerId;
}

type NodeState = ConfiguredNode | StartedNode | OnboardedNode;

class Node {
  private state: NodeState = { kind: StateKind.Configured };
  private logger: Logger;

  id: NodeId;
  httpPort: number;
  peerPort: number;
  radHome: string;

  get authToken(): AuthToken {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.authToken;
  }

  get peerAddress(): PeerAddress {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.peerAddress;
  }

  get peerId(): PeerId {
    if (this.state.kind !== StateKind.Onboarded) {
      throw new Error("Can't get peerAddress before node is onboarded");
    }

    return this.state.peerId;
  }

  get currentState(): StateKind {
    return this.state.kind;
  }

  constructor(id: NodeId, dataDir: string) {
    this.logger = new Logger({
      prefix: `[${id}]: `,
      indentationLevel: 2,
    });

    this.id = id;
    this.httpPort = id;
    this.peerPort = id;
    this.radHome = path.resolve(dataDir, `node-${id}`);
  }

  async start() {
    this.logger.log(`starting node ${this.id}`);

    await fs.mkdirs(this.radHome);

    const process = childProcess.spawn(
      PROXY_BINARY_PATH,
      [
        "--http-listen",
        `${HOST}:${this.httpPort}`,
        "--peer-listen",
        `${HOST}:${this.peerPort}`,
      ],
      { env: { ...global.process.env, RAD_HOME: this.radHome } }
    );

    process.on("exit", async () => {
      this.logger.log(`node terminated`);
      await this.cleanup();
    });

    process.stderr.setEncoding("utf8");
    process.stderr.on("data", data => {
      this.logger.log(`  STDERR: ${data.trim()}`);
    });

    process.stdout.setEncoding("utf8");
    process.stdout.on("data", data => {
      this.logger.log(`  STDOUT: ${data.trim()}`);
    });

    this.state = { ...this.state, kind: StateKind.Started, process: process };

    await waitOn({ resources: [`tcp:${HOST}:${this.httpPort}`] });

    this.logger.log("node started successfully");
  }

  async onboard(options: {
    handle: string;
    passphrase: string;
  }): Promise<NodeSession> {
    this.logger.log("onboarding node");

    if (this.state.kind !== StateKind.Started) {
      throw new Error("Tried to onboard a node that wasn't started yet");
    }

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

    const cookie = keystoreResponse.headers.get("set-cookie");
    if (!cookie) {
      throw new Error("Response did not contain an auth cookie");
    }

    const match = cookie.match(/auth-token=(.*);/);
    let authToken;
    if (match && match[1]) {
      authToken = match[1];
    } else {
      throw new Error("Auth cookie does not match the expected shape");
    }

    // We have to wait here because proxy restarts its internal machinery
    // after the keystore endpoint is queried.
    await sleep(500);

    const identitiesResponse = await fetch(
      `http://${HOST}:${this.id}/v1/identities`,
      {
        method: "post",
        body: JSON.stringify({ handle: options.handle }),
        headers: {
          Cookie: `auth-token=${authToken}`,
          "Content-Type": "application/json",
        },
      }
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

  stop(): void {
    if (this.state.kind !== StateKind.Configured) {
      this.logger.log("stopping node");
      if (!this.state.process.kill()) {
        this.logger.log(`could not stop process ${this.state.process.pid}`);
      }
    } else {
      this.logger.log("ignoring stop node command, node wasn't running");
    }
  }

  private async cleanup(): Promise<void> {
    this.logger.log("cleaning up state");
    await fs.remove(this.radHome);
  }
}

class NodeManager implements NodeManagerPlugin {
  private managedNodes: Node[] = [];
  private logger: Logger;
  private nextPort: number = 17000;

  constructor() {
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

  async startNode(dataDir: string): Promise<number> {
    const id = this.nextPort++;
    const node = new Node(id, dataDir);
    await node.start();
    this.managedNodes.push(node);

    return id;
  }

  async onboardNode(options: OnboardNodeOptions): Promise<NodeSession> {
    this.logger.log("onboardNode");

    const node = this.getNode(options.id);

    return node.onboard({
      handle: options.handle,
      passphrase: options.passphrase,
    });
  }

  async connectNodes(options: ConnectNodeOptions): Promise<null> {
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

    await fetch(`http://${HOST}:${firstNode.httpPort}/v1/session/settings`, {
      method: "post",
      body: JSON.stringify({
        appearance: { theme: "dark", hints: { showRemoteHelper: true } },
        coco: {
          seeds: remainingNodes.map(node => node.peerAddress),
        },
      }),
      headers: {
        Cookie: `auth-token=${firstNode.authToken}`,
        "Content-Type": "application/json",
      },
    });

    return null;
  }

  async stopAllNodes(): Promise<null> {
    this.logger.log("stopAllNodes");

    this.managedNodes.forEach(node => {
      node.stop();
    });

    this.managedNodes = [];

    this.nextPort = 17000;

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
