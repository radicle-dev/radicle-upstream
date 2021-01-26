export type NodeId = number;

export interface StartNodeOptions {
  id: NodeId;
  proxyBinaryPath?: string;
}

export interface OnboardNodeOptions {
  id: NodeId;
  handle?: string;
  passphrase?: string;
}

export interface OnboardedNode {
  authToken: string;
  peerAddress: string;
  httpPort: string;
}

export interface ConnectNodesOptions {
  nodeIds: NodeId[];
}

export enum Commands {
  StartNode = "startNode",
  OnboardNode = "onboardNode",
  StopAllNodes = "stopAllNodes",
  GetOnboardedNodes = "getOnboardedNodes",
  ConnectNodes = "connectNodes",
}
