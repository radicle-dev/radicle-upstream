// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

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
  | { type: "network" }
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
  | { type: "network" }
  | { type: "settings" };

export type LoadedOrgTab =
  | {
      type: "projects";
      anchors: org.OrgAnchors;
      gnosisSafeAddress: string;
      projectCount: number;
    }
  | {
      type: "members";
      threshold: number;
      members: org.Member[];
    };

interface MultiSigOrgLoadedRoute {
  type: "multiSigOrg";
  address: string;
  gnosisSafeAddress: string;
  activeTab: LoadedOrgTab;
  threshold: number;
  members: org.Member[];
}

interface SingleSigOrgLoadedRoute {
  type: "singleSigOrg";
  address: string;
  owner: string;
  projectCount: number;
  anchors: org.OrgAnchors;
}

type OrgLoadedRoute = SingleSigOrgLoadedRoute | MultiSigOrgLoadedRoute;

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
  const owner = await org.getOwner(route.address);
  const isMultiSig = await org.isMultiSig(owner);

  if (isMultiSig) {
    if (route.activeTab === "projects") {
      const [projectCount, { members, threshold }] = await Promise.all([
        org.getProjectCount(),
        org.fetchSafeMembers(owner),
      ]);
      const anchors = await org.resolveProjectAnchors({
        orgAddress: route.address,
        gnosisSafeAddress: owner,
        members,
        threshold,
      });

      return {
        type: "multiSigOrg",
        address: route.address,
        gnosisSafeAddress: owner,
        members,
        threshold,
        activeTab: {
          type: "projects",
          anchors,
          gnosisSafeAddress: owner,
          projectCount,
        },
      };
    } else if (route.activeTab === "members") {
      const { members, threshold } = await org.fetchSafeMembers(owner);
      return {
        type: "multiSigOrg",
        address: route.address,
        gnosisSafeAddress: owner,
        members,
        threshold,
        activeTab: {
          type: "members",
          members,
          threshold,
        },
      };
    } else {
      return unreachable(route.activeTab);
    }
  } else {
    const projectCount = await org.getProjectCount();
    const anchors = await org.resolveProjectAnchors({
      orgAddress: route.address,
      // TODO The data we pass in here only serves as dummy data that
      // enriches the project anchor data
      gnosisSafeAddress: owner,
      members: [],
      threshold: 0,
    });
    return {
      type: "singleSigOrg",
      address: route.address,
      owner,
      projectCount,
      anchors,
    };
  }
}
