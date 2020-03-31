import * as event from './event'

export interface Metadata {
  name: string;
};

export interface Project {
  metadata: Metadata;
};

export enum Kind {
  FetchList,
  ListFetched,
};

interface MsgInterface {
  kind: Kind,
};

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
};

interface ListFetched extends MsgInterface {
  kind: Kind.ListFetched;
  projects: Project[];
};

export type Msg = FetchList | ListFetched;

export type State = {
  projects: Project[]
};

export function init(): State {
  return { projects: [] }
};

export function update(state: State, msg: Msg): State {
  switch (msg.kind) {
    case Kind.FetchList:
      Api.fetchList();
      break;
    case Kind.ListFetched:
      state.projects = msg.projects;
      break;
  }

  return state;
};

namespace Api {
  export function fetchList(): void {
    fetch(
      "http://localhost:8080/v1/projects", {
        method: "GET",
        cache: "no-cache",
      })
      .then(res => {
        console.log(res)

        return res.json()
      })
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
};
