import * as path from "path";
import * as childProcess from "child_process";
import fetch from "node-fetch";
import waitOn from "wait-on";
import type { NodeSession } from "./shared";
import { Commands } from "./shared";

type NodeId = number;
type PeerAddress = string;
type AuthToken = string;

const ROOT_PATH = path.join(__dirname, "../../../");

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

// Because it's not possible to mix tagged union types and extending
// interfaces, we have to repeat the "inherited" attributes in each state.
interface ConfiguredNode {
  kind: "configured";

  // ConfiguredNode
  id: NodeId;
  httpPort: number;
  peerPort: number;
  proxyBinaryPath: string;
}

interface StartedNode {
  kind: "started";

  // ConfiguredNode
  id: NodeId;
  httpPort: number;
  peerPort: number;
  proxyBinaryPath: string;

  // StartedNode
  process: childProcess.ChildProcess;
}

interface OnboardedNode {
  kind: "onboarded";

  // ConfiguredNode
  id: NodeId;
  httpPort: number;
  peerPort: number;
  proxyBinaryPath: string;

  // StartedNode
  process: childProcess.ChildProcess;

  // OnboardedNode
  authToken: AuthToken;
  peerAddress: PeerAddress;
}

type NodeState = ConfiguredNode | StartedNode | OnboardedNode;

class Node {
  private state: NodeState;
  private logger: Logger;

  get id() {
    return this.state.id;
  }

  get httpPort() {
    return this.state.httpPort;
  }

  get peerAddress(): PeerAddress {
    if (this.state.kind !== "onboarded") {
      throw "Can't get peerAddress before node is onboarded";
    }

    return this.state.peerAddress;
  }

  get authToken(): AuthToken {
    if (this.state.kind !== "onboarded") {
      throw "Can't get peerAddress before node is onboarded";
    }

    return this.state.authToken;
  }

  constructor(options: { id: NodeId; proxyBinaryPath: string }) {
    this.logger = new Logger({
      prefix: `[${options.id}]: `,
      indentationLevel: 2,
    });
    this.state = {
      kind: "configured",
      id: options.id,
      httpPort: options.id,
      peerPort: options.id,
      proxyBinaryPath: path.join(ROOT_PATH, options.proxyBinaryPath),
    };
  }

  async start() {
    this.logger.log("starting node");

    const process = childProcess.spawn(
      this.state.proxyBinaryPath,
      [
        "--test",
        "--http-listen",
        `${HOST}:${this.state.httpPort}`,
        "--peer-listen",
        `${HOST}:${this.state.peerPort}`,
      ],
      {}
    );

    process.on("exit", async () => {
      this.logger.log(`node terminated`);
    });

    process.stderr.setEncoding("utf8");
    process.stderr.on("data", data => {
      this.logger.log(`  STDERR: ${data.trim()}`);
    });

    process.stdout.setEncoding("utf8");
    process.stdout.on("data", data => {
      this.logger.log(`  STDOUT: ${data.trim()}`);
    });

    this.state = { ...this.state, kind: "started", process: process };

    await waitOn({ resources: [`tcp:${HOST}:${this.state.httpPort}`] });

    this.logger.log("node started successfully");
  }

  async onboard(options: { handle: string; passphrase: string }) {
    this.logger.log("onboarding node");

    if (this.state.kind !== "started") {
      throw "Tried to onboard a node that wasn't started yet";
    }

    const keystoreResponse = await fetch(
      `http://${HOST}:${this.state.id}/v1/keystore`,
      {
        method: "post",
        body: JSON.stringify({ passphrase: options.passphrase }),
        headers: { "Content-Type": "application/json" },
      }
    );

    if (!keystoreResponse) {
      throw "No response from keystore request";
    }

    const cookie = keystoreResponse.headers.get("set-cookie");
    if (!cookie) {
      throw "Response did not contain an auth cookie";
    }

    const match = cookie.match(/auth-token=(.*);/);
    let authToken;
    if (match && match[1]) {
      authToken = match[1];
    } else {
      throw "Auth cookie does not match the expected shape";
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

    this.state = {
      ...this.state,
      kind: "onboarded",
      authToken: authToken,
      peerAddress: `${json.peerId}@${HOST}:${this.state.peerPort}`,
    };

    this.logger.log("node onboarded successfully");
  }

  async stop(): Promise<void> {
    if (this.state.kind === "configured") {
      throw "Tried to stop a node that wasn't started yet";
    }

    this.state.process.kill();
  }
}

interface StartNodeOptions {
  id: NodeId;
  proxyBinaryPath: string;
}

interface OnboardNodeOptions {
  id: NodeId;
  handle: string;
  passphrase: string;
}

interface ConnectNodeOptions {
  nodeIds: NodeId[];
}

class NodeManager {
  private managedNodes: Node[] = [];
  private logger: Logger;

  constructor() {
    this.logger = new Logger({ prefix: `[nodeManager] ` });
  }

  private getNode = (id: NodeId) => {
    const node = this.managedNodes.find(node => {
      return node.id === id;
    });

    if (!node) {
      throw `Could not find node by id ${id}`;
    }

    return node;
  };

  async startNode(options: StartNodeOptions): Promise<void> {
    this.logger.log("startNode");

    const node = new Node(options);
    await node.start();
    this.managedNodes.push(node);
  }

  async onboardNode(options: OnboardNodeOptions): Promise<void> {
    this.logger.log("onboardNode");

    const node = this.getNode(options.id);

    await node.onboard({
      handle: options.handle,
      passphrase: options.passphrase,
    });
  }

  async connectNodes(options: ConnectNodeOptions) {
    this.logger.log("connectNodes");

    if (options.nodeIds.length < 2) {
      throw "Supply at least 2 node IDs";
    }

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

  async getOnboardedNodes(): Promise<NodeSession[]> {
    this.logger.log("getOnboardedNodes");

    const onboardedNodes: NodeSession[] = [];

    this.managedNodes.forEach(node => {
      if (node.authToken && node.httpPort) {
        onboardedNodes.push({
          authToken: node.authToken,
          httpPort: node.httpPort,
        });
      }
    });

    return onboardedNodes;
  }

  async stopAllNodes(): Promise<void> {
    this.logger.log("stopAllNodes");

    this.managedNodes.forEach(async node => {
      await node.stop();
    });

    this.managedNodes = [];
  }
}

const nodeManager = new NodeManager();

export const nodeManagerPlugin = {
  [Commands.StartNode]: async ({
    id,
    proxyBinaryPath = "proxy/target/debug/radicle-proxy",
  }: StartNodeOptions): Promise<null> => {
    await nodeManager.startNode({ id, proxyBinaryPath });

    return null;
  },
  [Commands.OnboardNode]: async ({
    id,
    handle = "secretariat",
    passphrase = "radicle-upstream",
  }: OnboardNodeOptions): Promise<null> => {
    await nodeManager.onboardNode({ id, handle, passphrase });

    return null;
  },
  [Commands.GetOnboardedNodes]: async (): Promise<NodeSession[]> => {
    return nodeManager.getOnboardedNodes();
  },
  [Commands.ConnectNodes]: async (
    options: ConnectNodeOptions
  ): Promise<null> => {
    await nodeManager.connectNodes(options);

    return null;
  },
  [Commands.StopAllNodes]: async (): Promise<null> => {
    await nodeManager.stopAllNodes();

    return null;
  },
};
