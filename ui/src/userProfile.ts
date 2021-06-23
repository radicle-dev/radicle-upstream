// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as identity from "./identity";
import * as remote from "./remote";
import * as error from "./error";
import * as proxy from "./proxy";
import type { Project } from "./proxy/project";

const projectsStore = remote.createStore<Project[]>();
export const projects = projectsStore.readable;

const userStore = remote.createStore<identity.Identity>();
export const user = userStore.readable;

export const fetchProjects = (urn: string): void => {
  proxy.client.project
    .listForUser(urn)
    .then(projectsStore.success)
    .catch(err => projectsStore.error(error.fromUnknown(err)));
};

export const fetchUser = (urn: string): void => {
  identity
    .fetch(urn)
    .then(userStore.success)
    .catch(err => userStore.error(error.fromUnknown(err)));
};
