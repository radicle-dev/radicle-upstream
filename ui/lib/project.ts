import * as event from './event'
import { Writable, writable, derived } from 'svelte/store'

export interface Project {
  id: any
  metadata: {
    name: string
    default_branch: string
    description?: string
    img_url?: string
  }
}

interface ProjectsStore {
  projects: Project[]
}

export enum Kind {
  FetchList,
  ListFetched
}

interface MsgInterface {
  kind: Kind,
}

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
}

interface ListFetched extends MsgInterface {
  kind: Kind.ListFetched;
  projects: Project[];
}

export type Msg = FetchList | ListFetched


// Could use Api.fetchList() here, but this way we can demonstrate the event loop
const projectsStore: Writable<ProjectsStore> = writable(
  { projects: [] },
  set => event.emit({ kind: event.Kind.Project, msg: { kind: Kind.FetchList } })
)

// Read-only store accessible to components
export const projects = derived(
  projectsStore,
  $projectsStore => $projectsStore.projects
)

export function update(msg: Msg) {
  switch (msg.kind) {
    case Kind.FetchList:
      Api.fetchList();
      break;
    case Kind.ListFetched:
      projectsStore.set({ projects: msg.projects });
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
        console.log(data)
        event.emit({
          kind: event.Kind.Project,
          msg: {
            kind: Kind.ListFetched,
            projects: data,
          }
        })
      })
  }
}
