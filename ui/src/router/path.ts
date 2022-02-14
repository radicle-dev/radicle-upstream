// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Route } from "./definition";

export function routeToPath(route: Route): string {
  let subRoute = "";

  if (route.type === "org") {
    subRoute = `/${route.params.address}/${route.params.view}`;
  } else if (route.type === "diagnostics" || route.type === "wallet") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "userProfile") {
    subRoute = `/${route.params.urn}`;
  } else if (route.type === "project") {
    if (route.params.activeView.type === "patch") {
      subRoute = `/${route.params.urn}/${route.params.activeView.type}/${route.params.activeView.peerId}/${route.params.activeView.id}`;
    } else {
      subRoute = `/${route.params.urn}/${route.params.activeView.type}`;
    }
  }

  return `/${route.type}${subRoute}`;
}

export function pathToRoute(path: string): Route | undefined {
  const match = path.match(
    /project\/(rad:git:[1-9A-HJ-NP-Za-km-z]{37})\/patch\/(.*)\/(.*)/
  );
  if (!match) {
    return undefined;
  }
  const [projectUrn, peerId, patchId] = match.slice(1);
  return {
    type: "project",
    params: {
      urn: projectUrn,
      activeView: { type: "patch", id: patchId, peerId },
    },
  };
}

export function routeToCustomProtocolUrl(route: Route): string {
  return `radicle://upstream/v0${routeToPath(route)}`;
}
