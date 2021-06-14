import type * as project from "ui/src/project";
import { unreachable } from "ui/src/unreachable";

import * as org from "ui/src/org";

export type NetworkDiagnosticsTab = "peers" | "requests";
export type ProfileTab = "projects" | "following";
export type ProjectView =
  | { type: "files" }
  | { type: "commits" }
  | { type: "commit"; commitHash: string }
  | { type: "patches"; filter: "open" | "closed" | "all" }
  | { type: "patch"; id: string; peerId: string };
export type WalletTab = "transactions" | "tokenStreams";

export type Route =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | OrgRoute
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      urn: string;
      activeView: ProjectView;
    }
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "settings" };

interface OrgRoute {
  type: "org";
  address: string;
  activeTab: OrgTab;
}

export type OrgTab = "projects" | "members";

export type LoadedRoute =
  | { type: "boot" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | OrgLoadedRoute
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      urn: string;
      activeView: ProjectView;
    }
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "settings" };

export type LoadedOrgTab =
  | {
      type: "projects";
      anchoredProjects: project.Project[];
      unresolvedAnchors: project.Anchor[];
      gnosisSafeAddress: string;
    }
  | {
      type: "members";
      threshold: number;
      members: org.Member[];
    };

interface OrgLoadedRoute {
  type: "org";
  address: string;
  gnosisSafeAddress: string;
  activeTab: LoadedOrgTab;
  threshold: number;
  members: org.Member[];
}

export function routeToPath(route: Route): string {
  let subRoute = "";

  if (route.type === "profile" || route.type === "networkDiagnostics") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "project") {
    subRoute = `/${route.activeView.type}`;
  }

  return `#/${route.type}${subRoute}`;
}

export async function loadRoute(route: Route): Promise<LoadedRoute> {
  switch (route.type) {
    case "org":
      return loadOrgRoute(route);
    default:
      return route;
  }
}

async function loadOrgRoute(route: OrgRoute): Promise<OrgLoadedRoute> {
  if (route.activeTab === "projects") {
    const orgScreen = await org.fetchOrg(route.address);
    const projectAnchors = await org.resolveProjectAnchors(route.address);
    return {
      type: "org",
      address: route.address,
      gnosisSafeAddress: orgScreen.gnosisSafeAddress,
      members: orgScreen.members,
      threshold: orgScreen.threshold,
      activeTab: {
        type: "projects",
        anchoredProjects: projectAnchors.anchoredProjects,
        unresolvedAnchors: projectAnchors.unresolvedAnchors,
        gnosisSafeAddress: orgScreen.gnosisSafeAddress,
      },
    };
  } else if (route.activeTab === "members") {
    const orgScreen = await org.fetchOrg(route.address);
    return {
      type: "org",
      address: route.address,
      gnosisSafeAddress: orgScreen.gnosisSafeAddress,
      members: orgScreen.members,
      threshold: orgScreen.threshold,
      activeTab: {
        type: "members",
        members: orgScreen.members,
        threshold: orgScreen.threshold,
      },
    };
  } else {
    return unreachable(route.activeTab);
  }
}
