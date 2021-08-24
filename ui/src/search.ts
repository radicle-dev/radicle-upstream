// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { writable, Writable } from "svelte/store";

import * as proxy from "./proxy";
import * as error from "./error";
import type { Request, Project } from "./proxy/project";
import * as remote from "./remote";

// STATE
export const inputStore: Writable<string> = writable("");

const projectSearchStore = remote.createStore<Project>();
export const projectSearch = projectSearchStore.readable;

const projectRequestStore = remote.createStore<Request>();

export const projectRequest = projectRequestStore.readable;

export const reset = (): void => {
  projectRequestStore.reset();
  projectSearchStore.reset();
};

export const requestProject = async (urn: string): Promise<void> => {
  projectRequestStore.loading();
  try {
    const projectRequest = await proxy.client.project.requestSubmit(urn);
    projectRequestStore.success(projectRequest);
  } catch (err: unknown) {
    projectRequestStore.error(error.fromUnknown(err));
  }
};

export const searchProject = async (urn: string): Promise<void> => {
  projectSearchStore.loading();
  try {
    const project = await proxy.client.project.get(urn);
    projectSearchStore.success(project);
  } catch (err: unknown) {
    projectSearchStore.error(error.fromUnknown(err));
  }
};
