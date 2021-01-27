import type { OnboardedNode } from "./shared";

import { Commands } from "./shared";

export const withTwoConnectedNodes = (
  callback: (node0: OnboardedNode, node1: OnboardedNode) => void
): void => {
  cy.task(Commands.StartNode, { id: 17000 });
  cy.task(Commands.OnboardNode, { id: 17000 });

  cy.task(Commands.StartNode, { id: 18000 });
  cy.task(Commands.OnboardNode, { id: 18000 });

  cy.task(Commands.ConnectNodes, { nodeIds: [17000, 18000] });

  cy.task<OnboardedNode[]>(Commands.GetOnboardedNodes).then(nodes => {
    callback(nodes[0], nodes[1]);
  });

  cy.task(Commands.StopAllNodes);
};

export const asNode = (node: OnboardedNode): void => {
  cy.setCookie("auth-token", node.authToken);
  // NB: it is important that we pass `localhost` instead of `127.0.0.1` here.
  // I haven't figured out why, but when we use `127.0.0.1` instead of
  // `localhost`, the app loads with a auth-cookie mismatch error.
  cy.visit(`./public/index.html?backend=localhost:${node.httpPort}`);
};
