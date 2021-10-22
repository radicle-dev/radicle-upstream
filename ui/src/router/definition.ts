// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as orgRoute from "ui/App/OrgScreen/route";
import * as projectRoute from "ui/App/ProjectScreen/route";
import * as userProfileRoute from "ui/App/UserProfileScreen/route";

export type NetworkDiagnosticsTab = "peers" | "requests";
export type WalletTab = "transactions";

export type Route =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "org"; params: orgRoute.Params }
  | { type: "profile" }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; params: userProfileRoute.Params }
  | {
      type: "project";
      params: projectRoute.Params;
    }
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "network" }
  | { type: "orgs" }
  | { type: "settings" };

export type LoadedRoute =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | orgRoute.LoadedRoute
  | { type: "profile" }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | userProfileRoute.LoadedRoute
  | projectRoute.LoadedRoute
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "network" }
  | { type: "orgs" }
  | { type: "settings" };

export function routeToPath(route: Route): string {
  let subRoute = "";

  if (route.type === "org") {
    subRoute = `/${route.params.address}/${route.params.view}`;
  } else if (route.type === "networkDiagnostics" || route.type === "wallet") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "userProfile") {
    subRoute = `/${route.params.urn}`;
  } else if (route.type === "project") {
    subRoute = `/${route.params.urn}/${route.params.activeView.type}`;
  }

  return `#/${route.type}${subRoute}`;
}

export async function loadRoute(route: Route): Promise<LoadedRoute> {
  switch (route.type) {
    case "org":
      return orgRoute.load(route.params);
    case "project":
      return projectRoute.load(route.params);
    case "userProfile":
      return userProfileRoute.load(route.params);
    default:
      return route;
  }
}
