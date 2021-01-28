import type { NodeSession } from "../plugins/nodeManager/shared";
import { Commands } from "../plugins/nodeManager/shared";

const withNodeManager = (callback: () => void) => {
  cy.task(Commands.StopAllNodes, {}, { log: false });
  callback();
  cy.task(Commands.StopAllNodes, {}, { log: false });
};

interface WithTwoConnectedNodesOptions {
  node1Handle: string;
  node2Handle: string;
}

export const withTwoConnectedNodes = (
  callback: (node0: NodeSession, node1: NodeSession) => void,
  options: WithTwoConnectedNodesOptions
): void => {
  withNodeManager(() => {
    const NODE0_ID = 17000;
    const NODE1_ID = 18000;

    cy.log(`running node manager with two nodes: ${NODE0_ID} ${NODE1_ID}`);

    cy.task(Commands.StartNode, { id: NODE0_ID }, { log: false });
    cy.task(
      Commands.OnboardNode,
      { id: NODE0_ID, handle: options.node1Handle },
      { log: false }
    );

    cy.task(Commands.StartNode, { id: NODE1_ID }, { log: false });
    cy.task(
      Commands.OnboardNode,
      { id: NODE1_ID, handle: options.node2Handle },
      { log: false }
    );

    cy.task(
      Commands.ConnectNodes,
      { nodeIds: [NODE0_ID, NODE1_ID] },
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
