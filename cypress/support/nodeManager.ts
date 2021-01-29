import type { NodeSession } from "../plugins/nodeManager/shared";
import { Commands } from "../plugins/nodeManager/shared";

const withNodeManager = (callback: () => void): void => {
  cy.task(Commands.StopAllNodes, {}, { log: false });
  callback();
  cy.task(Commands.StopAllNodes, {}, { log: false });
};

interface WithTwoOnboardedNodesOptions {
  node1Handle: string;
  node2Handle: string;
}

export const connectTwoNodes = (
  node1: NodeSession,
  node2: NodeSession
): void => {
  cy.log(`adding node ${node2.id} as seed to node ${node1.id}`);
  cy.task(
    Commands.ConnectNodes,
    { nodeIds: [node1.id, node2.id] },
    { log: false }
  );
};

export const withTwoOnboardedNodes = (
  callback: (node1: NodeSession, node2: NodeSession) => void,
  options: WithTwoOnboardedNodesOptions = {
    node1Handle: "ele",
    node2Handle: "cloudhead",
  }
): void => {
  withNodeManager(() => {
    const NODE1_ID = 17000;
    const NODE2_ID = 18000;

    cy.log(`starting and onboarding node ${NODE1_ID}`);

    cy.task(Commands.StartNode, { id: NODE1_ID }, { log: false });
    cy.task(
      Commands.OnboardNode,
      { id: NODE1_ID, handle: options.node1Handle },
      { log: false }
    );

    cy.log(`starting and onboarding node ${NODE2_ID}`);

    cy.task(Commands.StartNode, { id: NODE2_ID }, { log: false });
    cy.task(
      Commands.OnboardNode,
      { id: NODE2_ID, handle: options.node2Handle },
      { log: false }
    );

    cy.task<NodeSession[]>(Commands.GetOnboardedNodes, {}, { log: false }).then(
      nodes => {
        callback(nodes[0], nodes[1]);
      }
    );
  });
};

export const asNode = (node: NodeSession): void => {
  cy.log(`switching UI to node ${node.id}`);

  cy.setCookie("auth-token", node.authToken);
  // NB: it is important that we pass `localhost` instead of `127.0.0.1` here.
  // I haven't figured out why, but when we use `127.0.0.1` instead of
  // `localhost`, the app loads with a auth-cookie mismatch error.
  cy.visit(`./public/index.html?backend=localhost:${node.httpPort}`);
};
