// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import qs from "qs";

import type { Config } from "ui/src/config";

import { createPlugin } from "cypress/support/plugin";
import {
  pluginMethods,
  NodeSession,
  NodeManagerPlugin,
} from "cypress/plugins/nodeManager/shared";

const nodeManagerPlugin = createPlugin<NodeManagerPlugin>(
  "nodeManager",
  pluginMethods
);

const startAndOnboardNode = (
  dataDir: string,
  onboardedUser: OnboardedUser
): Cypress.Chainable<NodeSession> => {
  return nodeManagerPlugin.startNode(dataDir).then(id => {
    cy.log(`Started node ${id}`);
    return nodeManagerPlugin.onboardNode({
      id,
      handle: onboardedUser.handle || "secretariat",
      passphrase: onboardedUser.passphrase || "radicle-upstream",
    });
  });
};

const withNodeManager = (callback: () => void): void => {
  nodeManagerPlugin.stopAllNodes();
  callback();
  nodeManagerPlugin.stopAllNodes();
};

interface OnboardedUser {
  handle?: string;
  passphrase?: string;
}

interface WithTwoOnboardedNodesOptions {
  dataDir: string;
  node1User: OnboardedUser;
  node2User: OnboardedUser;
}

export const connectTwoNodes = (
  node1: NodeSession,
  node2: NodeSession
): void => {
  cy.log(`adding node ${node2.id} as seed to node ${node1.id}`);
  nodeManagerPlugin.connectNodes({ nodeIds: [node1.id, node2.id] });
};

// Executes a shell command in the context of a node session.
//
// In particular, `.gitconfig` is properly set for the node.
export const exec = (cmd: string, session: NodeSession): void => {
  cy.exec(
    `set -euo pipefail
export PATH="$PWD/target/debug:$PATH"
export RADICLE_UNSAFE_FAST_KEYSTORE=1
${cmd}`,
    {
      env: {
        HOME: session.radHome,
        RAD_HOME: session.radHome,
      },
    }
  );
};

export const withOneOnboardedNode = (
  options: {
    dataDir: string;
    handle?: string;
  },
  callback: (node: NodeSession) => void
): void => {
  withNodeManager(() => {
    startAndOnboardNode(options.dataDir, {
      handle: options.handle,
    }).then(node => {
      callback(node);
    });
  });
};

export const withTwoOnboardedNodes = (
  options: WithTwoOnboardedNodesOptions,
  callback: (node1: NodeSession, node2: NodeSession) => void
): void => {
  withNodeManager(() => {
    startAndOnboardNode(options.dataDir, options.node1User).then(node0 => {
      startAndOnboardNode(options.dataDir, options.node2User).then(node1 => {
        callback(node0, node1);
      });
    });
  });
};

export const asNode = (node: NodeSession): void => {
  cy.log(`switching UI to node ${node.id}`);

  cy.setCookie("auth-token", node.authToken);

  const config: Partial<Config> = {
    // NB: it is important that we pass `localhost` instead of `127.0.0.1` here.
    // I haven't figured out why, but when we use `127.0.0.1` instead of
    // `localhost`, the app loads with a auth-cookie mismatch error.
    proxyAddress: `localhost:${node.httpPort}`,
  };

  const query = qs.stringify({
    config: JSON.stringify(config),
  });
  cy.visit(`./public/index.html?${query}`);
};
