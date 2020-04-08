import { writable } from "svelte/store";

import { emit } from "./event";
import * as message from "./message";
import { createRemoteDataStore } from "./RemoteDataStore";

// Store management & type definitions

export interface Project {
  id: string;
  metadata: {
    name: string;
    default_branch: string;
    description?: string;
  };
};

type Projects = Project[]

const projectsStore = createRemoteDataStore<Projects>(
  () => emit({
    kind: message.Kind.Project,
    msg: { kind: Kind.FetchList }
  })
)

// Read-only store accessible to components
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

// Anything related to event loop & messages
export enum Kind {
  FetchList = "FETCH_LIST",
  ListFetched = "LIST_FETCHED",
}

interface MsgInterface {
  kind: Kind,
}

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
}

interface ListFetched extends MsgInterface {
  kind: Kind.ListFetched;
  projects: Projects;
}

export type Msg = FetchList | ListFetched

// TODO(sos): error state
// Similar to reducer in Redux
export function update(msg: Msg) {
  switch (msg.kind) {
    case Kind.FetchList:
      Api.fetchList();
      projectsStore.loading()
      break;
    case Kind.ListFetched:
      projectsStore.success(msg.projects);
      break;
  }
}

namespace Api {
  export function fetchList(): void {
    fetch(
      "http://localhost:8080/v1/projects", {
      method: "GET",
      cache: "no-cache",
    })
      .then(res => res.json())
      .then(data => {
        // simulate a loading time
        const loading = setTimeout(() => {
          emit({
            kind: message.Kind.Project,
            msg: {
              kind: Kind.ListFetched,
              projects: data,
            }
          })
        }, 4000)
      });
  }
}
