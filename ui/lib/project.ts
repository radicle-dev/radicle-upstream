import { emit } from './event'
import { GlobalMessageKind } from './messages'
import { RemoteData, RemoteDataStatus, createRemoteDataStore, derivedStore } from './RemoteDataStore'

// Anything related to event loop & messages
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

// Similar to reducer in Redux
export function update(msg: Msg) {
  switch (msg.kind) {
    case Kind.FetchList:
      Api.fetchList();
      projectsStore.updateStatus(RemoteDataStatus.Loading)
      break;
    case Kind.ListFetched:
      projectsStore.update((state: any) => state = { status: RemoteDataStatus.Success, data: { projects: msg.projects } });
      break;
  }
}

// Store management & type definitions

export interface Project {
  id: any
  metadata: {
    name: string
    default_branch: string
    description?: string
    img_url?: string
  }
}

interface ProjectData extends RemoteData {
  data: {
    projects: Project[]
  }
}

const initialState: ProjectData = { status: RemoteDataStatus.NotAsked, data: { projects: [] } }
const projectsStore = createRemoteDataStore(
  initialState,
  () => emit({
    kind: GlobalMessageKind.Project,
    msg: { kind: Kind.FetchList }
  })
)

// Read-only store accessible to components
export const projects = derivedStore(projectsStore)

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
            kind: GlobalMessageKind.Project,
            msg: {
              kind: Kind.ListFetched,
              projects: data,
            }
          })
        }, 4000)
      })
  }
}
