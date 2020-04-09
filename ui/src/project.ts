import { writable } from "svelte/store";

import * as api from "./api";
import { emit } from "./event";
import * as message from "./message";
import { createStore } from "./remote";

// Anything related to event loop & messages
export enum Kind {
  FetchList = "FETCH_LIST",
  ListFetched = "LIST_FETCHED",
}

interface MsgInterface {
  kind: Kind;
}

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
}

export type Msg = FetchList

// Store management & type definitions

export interface Project {
  id: string;
  metadata: {
    name: string;
    default_branch: string;
    description?: string;
  };
}

type Projects = Project[]

const projectsStore = createStore<Projects>(
  () => emit({
    kind: message.Kind.Project,
    msg: { kind: Kind.FetchList }
  })
)

// Read-only store accessible to components
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

// TODO(sos): error state
// Similar to reducer in Redux
export function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.FetchList:
      projectsStore.loading()
      api.get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
}
