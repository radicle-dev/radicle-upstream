import type { NodeSession } from "../plugins/nodeManager/shared";
import { Commands } from "../plugins/nodeManager/shared";

const withNodeManager = (callback: () => void) => {
  cy.task(Commands.StopAllNodes);
  callback();
  cy.task(Commands.StopAllNodes);
};

export const withTwoConnectedNodes = (
  callback: (node0: NodeSession, node1: NodeSession) => void
): void => {
  withNodeManager(() => {
    cy.task(Commands.StartNode, { id: 17000 });
    cy.task(Commands.OnboardNode, { id: 17000 });

    cy.task(Commands.StartNode, { id: 18000 });
    cy.task(Commands.OnboardNode, { id: 18000 });

    cy.task(Commands.ConnectNodes, { nodeIds: [17000, 18000] });

    cy.task<NodeSession[]>(Commands.GetOnboardedNodes).then(nodes => {
      callback(nodes[0], nodes[1]);
    });
  });
};

export const asNode = (node: NodeSession): void => {
  cy.setCookie("auth-token", node.authToken);
  // NB: it is important that we pass `localhost` instead of `127.0.0.1` here.
  // I haven't figured out why, but when we use `127.0.0.1` instead of
  // `localhost`, the app loads with a auth-cookie mismatch error.
  cy.visit(`./public/index.html?backend=localhost:${node.httpPort}`);
};
