// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as identity from "ui/src/proxy/identity";
import type * as proxyProject from "ui/src/proxy/project";

import * as error from "./error";
import * as proxy from "./proxy";
import * as remote from "./remote";
import * as router from "ui/src/router";
import * as session from "ui/src/session";

const projectsStore = remote.createStore<proxyProject.Project[]>();
export const projects = projectsStore.readable;

const userStore = remote.createStore<identity.RemoteIdentity>();
export const user = userStore.readable;

export const fetchProjects = (urn: string): void => {
  proxy.client.project
    .listForUser(urn)
    .then(projectsStore.success)
    .catch(err => projectsStore.error(error.fromUnknown(err)));
};

export const fetchUser = (urn: string): void => {
  proxy.client
    .remoteIdentityGet(urn)
    .then(userStore.success)
    .catch(err => userStore.error(error.fromUnknown(err)));
};

export const openUserProfile = async (userUrn: string): Promise<void> => {
  const unsealed = session.unsealed();
  if (!unsealed) {
    throw new error.Error({
      message: "Expected unsealed session",
    });
  }
  if (userUrn === unsealed.identity.urn) {
    router.push({ type: "profile", activeTab: "projects" });
  } else {
    router.push({
      type: "userProfile",
      urn: userUrn,
    });
  }
};
