// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as projectRoute from "ui/App/ProjectScreen/route";
import * as orgRoute from "ui/App/OrgScreen/route";

export type NetworkDiagnosticsTab = "peers" | "requests";
export type ProfileTab = "projects" | "following";
export type WalletTab = "transactions";

export type Route =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "org"; params: orgRoute.Params }
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      params: projectRoute.Params;
    }
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "network" }
  | { type: "explore" }
  | { type: "settings" };

export type LoadedRoute =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | orgRoute.LoadedRoute
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | projectRoute.LoadedRoute
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "network" }
  | { type: "explore" }
  | { type: "settings" };

export function routeToPath(route: Route): string {
  let subRoute = "";

  if (route.type === "profile" || route.type === "networkDiagnostics") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "project") {
    subRoute = `/${route.params.activeView.type}`;
  }

  return `#/${route.type}${subRoute}`;
}

export async function loadRoute(route: Route): Promise<LoadedRoute> {
  switch (route.type) {
    case "org":
      return orgRoute.load(route.params);
    case "project":
      return projectRoute.load(route.params);
    default:
      return route;
  }
}
