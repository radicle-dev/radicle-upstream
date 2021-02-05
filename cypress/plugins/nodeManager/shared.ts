// Types and constants shared between `plugin.ts` and `commands.ts`. These types can't be in
// `plugin.ts`, because Cypress plugins run in a separate Nodejs environment
// and can directly use Nodejs libraries, the Cypress tests don't have access
// to those, so indlucing `plugin.ts` inside `commands.ts` leads to errors.

import * as path from "path";

export type PeerId = string;

export interface NodeSession {
  id: number;
  peerId: PeerId;
  authToken: string;
  httpPort: number;
  storagePath: string;
}

export enum Commands {
  StartNode = "startNode",
  OnboardNode = "onboardNode",
  StopAllNodes = "stopAllNodes",
  GetOnboardedNodes = "getOnboardedNodes",
  ConnectNodes = "connectNodes",
}

// A directory that can be used for temporary test data.
//
// It is located within this repository so that there is no extra setup necessary
// when using it locally or on CI. To avoid committing any left-over temp data
// this directory ignored via .gitignore.
export const CYPRESS_WORKSPACE_PATH = path.join(__dirname, "../../workspace");
