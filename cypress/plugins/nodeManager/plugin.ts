import * as path from "path";
import * as childProcess from "child_process";
import fetch from "node-fetch";
import waitOn from "wait-on";
import type {
  ConnectNodesOptions,
  NodeId,
  StartNodeOptions,
  OnboardNodeOptions,
  OnboardedNode,
} from "./shared";
import { Commands } from "./shared";

const ROOT_PATH = path.join(__dirname, "../../../");

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

interface NodeStartOptions {
  id: NodeId;
  proxyBinaryPath?: string;
}

interface NodeOnboardOptions {
  handle: string;
  passphrase: string;
}

class Node {
  id: NodeId;
  authToken: string | undefined = undefined;
  peerAddress: string | undefined = undefined;
  httpPort: number;
  host: string = "127.0.0.1";

  private peerPort: number;
  private process: childProcess.ChildProcess | undefined;
  private proxyBinaryPath: string;
  private logger: Logger;

  constructor({
    id,
    proxyBinaryPath = "proxy/target/release/radicle-proxy",
  }: NodeStartOptions) {
    this.id = id;
    this.httpPort = id;
    this.peerPort = id;
    this.proxyBinaryPath = path.join(ROOT_PATH, proxyBinaryPath);
    this.logger = new Logger({ prefix: `[${id}]: `, indentationLevel: 2 });
  }

  async start() {
    this.logger.log("starting node");

    const process = childProcess.spawn(
      this.proxyBinaryPath,
      [
        "--test",
        "--http-listen",
        `${this.host}:${this.httpPort}`,
        "--peer-listen",
        `${this.host}:${this.peerPort}`,
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

    this.process = process;

    await waitOn({ resources: [`tcp:${this.host}:${this.httpPort}`] });

    this.logger.log("node started successfully");
  }

  async onboard({
    handle = `user-${this.id}`,
    passphrase = "radicle-upstream",
  }: NodeOnboardOptions) {
    this.logger.log("onboarding node");

    const keystoreResponse = await fetch(
      `http://${this.host}:${this.id}/v1/keystore`,
      {
        method: "post",
        body: JSON.stringify({ passphrase }),
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
    if (match) {
      this.authToken = match[1];
    } else {
      throw "Auth cookie does not match the expected shape";
    }

    // We have to wait here because proxy restarts its internal machinery
    // after the keystore endpoint is queried.
    await sleep(500);

    const identitiesResponse = await fetch(
      `http://${this.host}:${this.id}/v1/identities`,
      {
        method: "post",
        body: JSON.stringify({ handle }),
        headers: {
          Cookie: `auth-token=${this.authToken}`,
          "Content-Type": "application/json",
        },
      }
    );
    const json = await identitiesResponse.json();

    this.peerAddress = `${json.peerId}@${this.host}:${this.peerPort}`;

    this.logger.log("node onboarded successfully");
  }

  async stop(): Promise<void> {
    if (!this.process) {
      throw "Tried to stop node before it was started";
    }

    this.process.kill();
  }
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

    // FIXME
    await node.onboard(options as NodeOnboardOptions);
  }

  async connectNodes(options: ConnectNodesOptions) {
    this.logger.log("connectNodes");

    if (options.nodeIds.length < 2) {
      throw "Supply at least 2 node IDs";
    }

    const firstNode = this.getNode(options.nodeIds[0]);
    const remainingNodes = this.managedNodes.filter(node => {
      return firstNode.id !== node.id;
    });

    await fetch(
      `http://${firstNode.host}:${firstNode.httpPort}/v1/session/settings`,
      {
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
      }
    );

    return null;
  }

  async getOnboardedNodes(): Promise<OnboardedNode[]> {
    this.logger.log("getOnboardedNodes");

    const onboardedNodes: OnboardedNode[] = [];

    this.managedNodes.forEach(node => {
      if (node.authToken && node.peerAddress && node.httpPort) {
        onboardedNodes.push({
          authToken: node.authToken,
          peerAddress: node.peerAddress,
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
  [Commands.StartNode]: async (options: StartNodeOptions): Promise<null> => {
    await nodeManager.startNode(options);

    return null;
  },
  [Commands.OnboardNode]: async (
    options: OnboardNodeOptions
  ): Promise<null> => {
    await nodeManager.onboardNode(options);

    return null;
  },
  [Commands.GetOnboardedNodes]: async (): Promise<OnboardedNode[]> => {
    return nodeManager.getOnboardedNodes();
  },
  [Commands.ConnectNodes]: async (
    options: ConnectNodesOptions
  ): Promise<null> => {
    await nodeManager.connectNodes(options);

    return null;
  },
  [Commands.StopAllNodes]: async (): Promise<null> => {
    await nodeManager.stopAllNodes();

    return null;
  },
};
