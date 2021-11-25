// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as proxyIdentity from "proxy-client/identity";
import type * as proxyProject from "proxy-client/project";

import * as Session from "ui/src/session";
import * as proxy from "ui/src/proxy";

export interface Params {
  urn: string;
}

export interface LoadedRoute {
  type: "userProfile";
  ownUserUrn: string;
  user: proxyIdentity.RemoteIdentity;
  projects: proxyProject.Project[];
}

export async function load(params: Params): Promise<LoadedRoute> {
  const session = await Session.waitUnsealed();
  const user = await proxy.client.personGet(params.urn);
  const projects = await proxy.client.project.listForUser(params.urn);
  return {
    type: "userProfile",
    ownUserUrn: session.identity.urn,
    user,
    projects,
  };
}
