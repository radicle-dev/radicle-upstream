import * as path from "path";
import * as childProcess from "child_process";
import fetch from "node-fetch";
import * as util from "util";
import waitOn from "wait-on";
import * as uuid from "uuid";
import * as fs from "fs-extra";

const exec = util.promisify(childProcess.exec);

const HOST = "127.0.0.1";
const ROOT_PATH = path.join(__dirname, "../../");
const CYPRESS_WORKSPACE_PATH = path.join(ROOT_PATH, "cypress/workspace");
const PROXY_PATH = path.join(ROOT_PATH, "proxy/target/debug/radicle-proxy");

let nodes: Node[] = [];

const getNode = (id: NodeId) => {
  const node = nodes.find(node => {
    return node.id === id;
  });

  if (!node) {
    throw `Could not find node by id ${id}`;
  }

  return node;
};

const sleep = async (ms: number) => {
  await new Promise(resolve => setTimeout(resolve, ms));
};

type NodeId = number;

interface Node {
  id: NodeId;
  httpListen: number;
  peerListen: number;
  workspacePath: string;
  pid?: number;
  authToken?: string;
  peerAddress?: string;
}

export interface OnboardedNode {
  authToken: string;
  peerAddress: string;
  workspacePath: string;
}

export default (
  on: Cypress.PluginEvents,
  _config: Cypress.PluginConfigOptions
): void => {
  on("task", {
    async killAllNodes() {
      nodes.forEach(async node => {
        try {
          await exec(`kill -9 ${node.pid}`);
        } catch (err) {
          console.log("    ERROR: ", err.message);
        }
      });
      nodes = [];
      return null;
    },
    async startNode(id: NodeId) {
      // Start a new Upstream proxy, the `id` is used as a port number for both
      // API TCP connections as well as UDP peer connections.
      //
      // This also sets up an empty workspace directory where we can create new
      // project and checkout directories or store any other data necessary for
      // a testcase:
      //   ./radicle-upstream/cypress/workspace/<RANDOM_UUID>
      //
      // This command waits on the proxy process to start listening on its HTTP
      // API port before returning.

      console.log(`  ### node start ${id}`);

      const node: Node = {
        id: id,
        httpListen: id,
        peerListen: id,
        workspacePath: path.join(CYPRESS_WORKSPACE_PATH, uuid.v4()),
      };

      try {
        await fs.mkdirs(node.workspacePath);
      } catch {
        console.log(
          `    ERROR: could not create temporary workspace for node ${node.workspacePath}`
        );
      }

      const process = childProcess.spawn(
        PROXY_PATH,
        [
          "--test",
          "--http-listen",
          `${HOST}:${node.httpListen}`,
          "--peer-listen",
          `${HOST}:${node.peerListen}`,
        ],
        {}
      );

      node.pid = process.pid;

      process.on("exit", async () => {
        console.log(`    [${id}] node terminated`);
        if (node.workspacePath) {
          console.log(
            `    [${id}] cleaning up temp directory ${node.workspacePath}`
          );
          await exec(`rm -rf ${node.workspacePath}`);
        }

        return null;
      });

      process.stderr.setEncoding("utf8");
      process.stderr.on("data", data => {
        console.log(`    [${id}] STDERR: ${data.trim()}`);
      });

      process.stdout.setEncoding("utf8");
      process.stdout.on("data", data => {
        console.log(`    [${id}] STDOUT: ${data.trim()}`);
      });

      await waitOn({ resources: [`tcp:${HOST}:${id}`] });

      console.log(`    [${id}] node started successfully`);
      nodes.push(node);

      return null;
    },
    async onboardNode(id: NodeId) {
      // Onboard a node by requesting and storing an auth token and creating
      // an identity.

      console.log(`  ### node onboard ${id}`);

      const node = getNode(id);

      const keystoreResponse = await fetch(`http://${HOST}:${id}/v1/keystore`, {
        method: "post",
        body: JSON.stringify({ passphrase: "radicle-upstream" }),
        headers: { "Content-Type": "application/json" },
      });

      if (!keystoreResponse) {
        throw "No response from keystore request";
      }

      const cookie = keystoreResponse.headers.get("set-cookie");
      if (!cookie) {
        throw "Response did not contain an auth cookie";
      }

      const match = cookie.match(/auth-token=(.*);/);
      if (match) {
        node.authToken = match[1];
      } else {
        throw "Auth cookie does not match the expected shape";
      }

      // FIXME: this needs a retry until successful, because proxy restarts
      // its internal machinery after the keystore endpoint is queried.
      await sleep(500);

      const identitiesResponse = await fetch(
        `http://${HOST}:${id}/v1/identities`,
        {
          method: "post",
          body: JSON.stringify({ handle: `user-${node.id}` }),
          headers: {
            Cookie: `auth-token=${node.authToken}`,
            "Content-Type": "application/json",
          },
        }
      );
      const json = await identitiesResponse.json();

      node.peerAddress = `${json.peerId}@${HOST}:${node.peerListen}`;

      return null;
    },
    async connectNodes(nodeIds: NodeId[]) {
      // Establish a network connection between multiple nodes by picking the
      // first node and adding every other node as seed in the settings of the
      // first node.

      console.log(`  ### node connect ${nodeIds}`);
      if (nodeIds.length < 2) {
        console.log("    Supply at least 2 node IDs");
        return null;
      }

      const firstNode = getNode(nodeIds[0]);
      const remainingNodes = nodes.filter(node => {
        return firstNode.id !== node.id;
      });

      await fetch(`http://${HOST}:${firstNode.id}/v1/session/settings`, {
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
    },
    async withNode(id: NodeId): Promise<OnboardedNode> {
      const node = getNode(id);
      if (node.authToken && node.workspacePath && node.peerAddress) {
        return {
          authToken: node.authToken,
          workspacePath: node.workspacePath,
          peerAddress: node.peerAddress,
        };
      } else {
        throw "Node has to be onboarded before you can interact with it";
      }
    },
  });
};
