// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Route } from "./definition";

const URI_PREFIX = "radicle://upstream/v0/";

export function routeToUri(route: Route): string {
  const path = routeToPath(route);
  return `${URI_PREFIX}${path.substring(1)}`;
}

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

export function uriToRoute(url: string): Route | undefined {
  if (url.startsWith(URI_PREFIX)) {
    const path = url.substring(URI_PREFIX.length);
    return pathToRoute(path);
  } else {
    return undefined;
  }
}

function pathToRoute(path: string): Route | undefined {
  const segments = path.split("/");
  const type = segments.shift();
  switch (type) {
    case "project": {
      const urn = segments.shift();
      const resource = segments.shift();
      if (urn && resource === "patch") {
        const peerId = segments.shift();
        const patchId = segments.join("/");
        if (peerId && patchId) {
          return {
            type: "project",
            params: {
              urn,
              activeView: { type: "patch", id: patchId, peerId },
            },
          };
        } else {
          return undefined;
        }
      } else {
        return undefined;
      }
    }
    default:
      return undefined;
  }
}
