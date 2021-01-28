// Types shared between `plugin.ts` and `commands.ts`. These types can't be in
// `plugin.ts`, because Cypress plugins run in a separate Nodejs environment
// and can directly use Nodejs libraries, the Cypress tests don't have access
// to those, so indlucing `plugin.ts` inside `commands.ts` leads to errors.

export interface NodeSession {
  id: number;
  authToken: string;
  httpPort: number;
}

export enum Commands {
  StartNode = "startNode",
  OnboardNode = "onboardNode",
  StopAllNodes = "stopAllNodes",
  GetOnboardedNodes = "getOnboardedNodes",
  ConnectNodes = "connectNodes",
}
