import { writable, Writable } from "svelte/store";

import * as api from "./api";
import * as error from "./error";
import type * as project from "./project";
import * as remote from "./remote";
import type * as waitingRoom from "./waitingRoom";

// STATE
export const inputStore: Writable<string> = writable("");

const projectSearchStore = remote.createStore<project.Project>();
export const projectSearch = projectSearchStore.readable;

// FIXME(xla): Use Request type once serialised and returned by the API.
const projectRequestStore = remote.createStore<waitingRoom.ProjectRequest>();

export const projectRequest = projectRequestStore.readable;

export const reset = (): void => {
  projectRequestStore.reset();
  projectSearchStore.reset();
};

export const requestProject = async (urn: string): Promise<void> => {
  projectRequestStore.loading();
  try {
    const projectRequest = await api.put<null, waitingRoom.ProjectRequest>(
      `projects/requests/${urn}`,
      null
    );
    projectRequestStore.success(projectRequest);
  } catch (err) {
    projectRequestStore.error(error.fromException(err));
  }
};

export const searchProject = async (urn: string): Promise<void> => {
  projectSearchStore.loading();
  try {
    const project = await api.get<project.Project>(`projects/${urn}`);
    projectSearchStore.success(project);
  } catch (err) {
    projectSearchStore.error(error.fromException(err));
  }
};
