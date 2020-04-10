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

// State.
const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

// State transitions.
enum Kind {
  FetchList = "FETCH_LIST",
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.FetchList;
}

type Msg = Fetch;

const update = (msg: Msg): void => {
  console.log("project.update", msg)
  switch (msg.kind) {
    case Kind.FetchList:
      projectsStore.loading()
      api.get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
}

// Events.

const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);

// Fetch initial list when the store has been subcribed to for the first time.
projectsStore.start(fetchList);
