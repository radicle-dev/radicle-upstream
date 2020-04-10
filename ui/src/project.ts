import { writable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// Types.
export interface Project {
  id: string;
  metadata: {
    name: string;
    default_branch: string;
    description?: string;
  };
}

type Projects = Project[]

const projectsStore = remote.createStore<Projects>(fetchList)
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

// Events.
export enum Kind {
  FetchList = "FETCH_LIST",
}

export function update(msg: event.Event<Kind, void>): void {
  switch (msg.kind) {
    case Kind.FetchList:
      projectsStore.loading()
      api.get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
}

const fetchList = event.create<Kind, void>(Kind.FetchList, update)
