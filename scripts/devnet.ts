#!/usr/bin/env -S node --require ts-node/register/transpile-only

// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as yargs from "yargs";
import execa from "execa";
import * as Path from "path";
import * as Fs from "fs/promises";
import * as Crypto from "crypto";
import multibase from "multibase";
import assert from "assert";
import TweetNacl from "tweetnacl";

async function main() {
  yargs
    .command(upstreamCommand)
    .command(seedCommand)
    // Don’t show a version and the --version flag
    .version(false)
    .strict()
    .fail((msg, err, yargs) => {
      if (err === undefined) {
        yargs.showHelp("error");
        console.error("");
        console.error(msg);
      } else {
        console.error(err);
      }
      process.exit(1);
    })
    .wrap(Math.min(100, yargs.terminalWidth()))
    .demandCommand().argv;
}

const upstreamCommand: yargs.CommandModule<
  unknown,
  {
    id: number;
    reset: boolean;
    bootstrap?: number;
    headless: boolean;
    seedBootstrap: boolean;
  }
> = {
  command: "upstream <id>",
  describe: "Run an upstream instance",
  builder: yargs => {
    return yargs
      .positional("id", {
        demandOption: true,
        type: "number",
        describe:
          "Number identifying the instance. This is used to derive the peer configuration. Must be between 1 and 100",
      })
      .options({
        bootstrap: {
          type: "number",
          describe: "Use the instance identified by the ID as a boostrap peer",
        },
        seedBootstrap: {
          type: "boolean",
          default: true,
          describe: "Add the seed node to the bootstrap addresses",
        },
        reset: {
          type: "boolean",
          default: false,
          describe:
            "Delete existing data for the peer and re-initialize the peer",
        },
        headless: {
          type: "boolean",
          default: false,
          describe: "Only run radicle-proxy and not the frontend",
        },
      });
  },
  handler: async opts => {
    const peerConfig = makePeerConfig(opts.id);
    const missing = await Fs.access(peerConfig.radHome).catch(() => true);
    if (opts.reset && !missing) {
      await Fs.rm(peerConfig.radHome, { recursive: true });
    }
    if (missing || opts.reset) {
      await execa(
        "cargo",
        ["run", "--bin", "radicle-proxy-init", "--", peerConfig.userHandle],
        {
          stdio: "inherit",
          env: {
            RAD_HOME: peerConfig.radHome,
          },
        }
      );
    }

    const seedAddresses: string[] = [];
    if (opts.seedBootstrap) {
      seedAddresses.push(getPeerAddress(makePeerSeedConfig()));
    }
    if (opts.bootstrap !== undefined) {
      seedAddresses.push(getPeerAddress(makePeerConfig(opts.bootstrap)));
    }

    if (opts.headless) {
      await execa(
        "cargo",
        [
          "run",
          "--bin=radicle-proxy",
          "--",
          "--skip-remote-helper-install",
          "--insecure-http-api",
          "--unsafe-fast-keystore",
          "--dev-log",
        ],
        {
          stdio: "inherit",
          env: getProxyEnv(peerConfig, seedAddresses),
        }
      );
    } else {
      await execa("yarn", ["run", "electron", "./native/index.js"], {
        stdio: "inherit",
        env: {
          NODE_ENV: "development",
          RADICLE_UPSTREAM_HTTP_PORT: peerConfig.httpPort.toString(),
          ...getProxyEnv(peerConfig, seedAddresses),
        },
      });
    }
  },
};

const seedCommand: yargs.CommandModule<
  unknown,
  { reset: boolean; project?: string }
> = {
  command: "seed",
  describe: "Run upstream-see",
  builder: yargs => {
    return yargs.options({
      reset: {
        type: "boolean",
        default: false,
        describe: "Delete existing data for the seed and re-initialize it",
      },
      project: {
        type: "string",
        describe: "URN of the project to track",
      },
    });
  },
  handler: async opts => {
    const peerConfig = makePeerSeedConfig();
    const missing = await Fs.access(peerConfig.radHome).catch(() => true);
    if (opts.reset && !missing) {
      await Fs.rm(peerConfig.radHome, { recursive: true });
    }

    const keyPath = Path.join(peerConfig.radHome, "identity.key");
    if ((await Fs.access(keyPath).catch(() => false)) === false) {
      await Fs.mkdir(Path.dirname(keyPath), { recursive: true });
      const seedHash = Crypto.createHash("sha256");
      seedHash.update("seed");
      const seedDigest = seedHash.digest();
      await Fs.writeFile(keyPath, seedDigest, "binary");
    }

    await execa(
      "cargo",
      [
        "run",
        "--bin=upstream-seed",
        "--",
        "--identity-key",
        keyPath,
        `--listen=127.0.0.1:${peerConfig.p2pPort}`,
        "--rad-home",
        peerConfig.radHome,
      ],
      {
        stdio: "inherit",
      }
    );
  },
};

main();

interface PeerConfig {
  userHandle: string;
  peerId: string;
  httpPort: number;
  p2pPort: number;
  radHome: string;
}

// Get a peer ID from a private key seed.
//
// Uses the same algorithm as `radicle-proxy-init`.
function peerIdFromKeySeed(seed: string): string {
  const seedHash = Crypto.createHash("sha256");
  seedHash.update(seed);
  const seedDigest = seedHash.digest();
  const key = TweetNacl.sign.keyPair.fromSeed(new Uint8Array(seedDigest));

  const paddedPkData = new Uint8Array([0, ...key.publicKey]);
  const peerIdBytes = multibase.encode("base32z", paddedPkData);
  return new TextDecoder("utf-8").decode(peerIdBytes);
}

function makePeerConfig(id: number): PeerConfig {
  assert(id > 0 && id < 100, `peer id ${id} is not in range`);
  const radHome = Path.resolve(
    __dirname,
    "..",
    "sandbox",
    "devnet",
    id.toString()
  );
  return {
    userHandle: id.toString(),
    peerId: peerIdFromKeySeed(id.toString()),
    httpPort: 24500 + id,
    p2pPort: 24600 + id,
    radHome,
  };
}

function makePeerSeedConfig(): PeerConfig {
  const id = "seed";
  const radHome = Path.resolve(__dirname, "..", "sandbox", "devnet", id);
  return {
    userHandle: id,
    peerId: peerIdFromKeySeed(id),
    httpPort: 24500,
    p2pPort: 24600,
    radHome,
  };
}

function getPeerAddress(peerConfig: PeerConfig) {
  return `${peerConfig.peerId}@127.0.0.1:${peerConfig.p2pPort}`;
}

function getProxyEnv(
  peerConfig: PeerConfig,
  seedAddresses: string[] = []
): Record<string, string | undefined> {
  const seeds = seedAddresses.length > 0 ? seedAddresses.join(",") : undefined;
  return {
    RAD_HOME: peerConfig.radHome,
    RADICLE_PROXY_HTTP_LISTEN: `127.0.0.1:${peerConfig.httpPort}`,
    RADICLE_PROXY_PEER_LISTEN: `127.0.0.1:${peerConfig.p2pPort}`,
    RADICLE_PROXY_INSECURE_HTTP_API: "true",
    RADICLE_PROXY_SEEDS: seeds,
    RADICLE_PROXY_KEY_PASSPHRASE: "asdf",
  };
}
