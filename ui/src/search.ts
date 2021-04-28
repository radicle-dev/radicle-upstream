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
  } catch (err) {
    projectRequestStore.error(error.fromUnknown(err));
  }
};

export const searchProject = async (urn: string): Promise<void> => {
  projectSearchStore.loading();
  try {
    const project = await proxy.client.project.get(urn);
    projectSearchStore.success(project);
  } catch (err) {
    projectSearchStore.error(error.fromUnknown(err));
  }
};
