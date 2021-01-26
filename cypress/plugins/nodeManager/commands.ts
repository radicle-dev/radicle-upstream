import type {
  ConnectNodesOptions,
  OnboardNodeOptions,
  OnboardedNode,
  StartNodeOptions,
} from "./shared";

import { Commands } from "./shared";

const startNode = (options: StartNodeOptions): void => {
  cy.task(Commands.StartNode, options);
};

const onboardNode = (options: OnboardNodeOptions): void => {
  cy.task(Commands.OnboardNode, options);
};

const connectNodes = (options: ConnectNodesOptions): void => {
  cy.task(Commands.ConnectNodes, options);
};

const stopAllNodes = (): void => {
  cy.task(Commands.StopAllNodes);
};

export const withTwoConnectedNodes = (
  callback: (node0: OnboardedNode, node1: OnboardedNode) => void
): void => {
  startNode({ id: 17000 });
  onboardNode({ id: 17000 });

  startNode({ id: 18000 });
  onboardNode({ id: 18000 });

  connectNodes({ nodeIds: [17000, 18000] });

  cy.task<OnboardedNode[]>(Commands.GetOnboardedNodes).then(nodes => {
    callback(nodes[0], nodes[1]);
  });

  stopAllNodes();
};

export const asNode = (node: OnboardedNode): void => {
  cy.setCookie("auth-token", node.authToken);
  // NB: it is important that we pass `localhost` instead of `127.0.0.1` here.
  // I haven't figured out why, but when we use `127.0.0.1` instead of
  // `localhost`, the app loads with a auth-cookie mismatch error.
  cy.visit(`./public/index.html?backend=localhost:${node.httpPort}`);
};
