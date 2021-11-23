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

import {
  pluginMethods,
  type NodeManagerPlugin,
  type NodeSession,
  type StartNodeOptions,
} from "./shared";

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

class NodeManager implements NodeManagerPlugin {
  private logger: Logger;
  // A radicle-proxy is running on port 30000 for any other Cypress tests
  // that aren't managed by nodeManager.
  private nextPort: number = 30001;
  #processes: execa.ExecaChildProcess[] = [];

  public constructor() {
    this.logger = new Logger({ prefix: `[nodeManager] ` });
  }

  public async startNode(options: StartNodeOptions): Promise<NodeSession> {
    const id = this.nextPort++;
    const logger = new Logger({
      prefix: `[node ${id}]: `,
      indentationLevel: 2,
    });

    const radHome = path.resolve(options.baseDataDir, `node-${id}`);
    await fs.mkdirs(radHome);

    const process = execa(
      PROXY_BINARY_PATH,
      [
        "--http-listen",
        `${HOST}:${id}`,
        "--peer-listen",
        `${HOST}:${id}`,
        "--skip-remote-helper-install",
        "--unsafe-fast-keystore",
        "--insecure-http-api",
      ],
      {
        buffer: false,
        env: {
          RAD_HOME: radHome,
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

    this.#processes.push(process);

    process.on("exit", async () => {
      logger.log(`node terminated`);
    });

    const stderrLogPath = path.join(radHome, "stderr.log");
    logger.log(`writing output to "${stderrLogPath}"`);
    const stderrLog = fs.createWriteStream(stderrLogPath);
    // We know that `stderr` is set because of the `stdio` spawn options
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const stderr = process.stderr!;
    stderr.pipe(stderrLog);

    const combinedLogPath = path.join(options.baseDataDir, "combined-node.log");
    const combinedLog = fs.createWriteStream(combinedLogPath, { flags: "a" });
    const prependNodeId = new LinePrefix(`Node ${id}: `);

    stderr.pipe(prependNodeId).pipe(combinedLog);

    await waitOn({ resources: [`tcp:${HOST}:${id}`] });

    logger.log("node started successfully");

    const gitConfigSet = (name: string, value: string) =>
      execa("git", ["config", "--global", name, value], {
        env: { HOME: radHome },
      });

    await gitConfigSet(
      "credential.helper",
      `!f() { test "$1" = get && echo "password=${options.passphrase}"; }; f`
    );
    await gitConfigSet("user.name", options.handle);
    await gitConfigSet("user.email", `${options.handle}@example.com`);

    const keystoreResponse = await fetch(`http://${HOST}:${id}/v1/keystore`, {
      method: "post",
      body: JSON.stringify({ passphrase: options.passphrase }),
      headers: { "Content-Type": "application/json" },
    });

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
        fetch(`http://${HOST}:${id}/v1/identities`, {
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

    logger.log("node onboarded successfully");

    return {
      id,
      authToken,
      httpPort: id,
      radHome: radHome,
      peerId,
    };
  }

  public async stopAllNodes(): Promise<null> {
    this.logger.log("stopAllNodes");
    for (const process of this.#processes) {
      if (!process.killed) {
        process.kill("SIGKILL");
      }
    }
    this.#processes = [];

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
