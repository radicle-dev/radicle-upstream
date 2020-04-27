import { Readable, writable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";
import * as transaction from "./transaction";

// TYPES.
export interface Metadata {
  name: string;
  default_branch: string;
  description?: string;
}

export interface Project {
  id: string;
  metadata: Metadata;
}

type Projects = Project[]

// STATE
const creationStore = remote.createStore<Project>();
export const creation = creationStore.readable;

const projectStore = remote.createStore<Project>();
export const project = projectStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

// EVENTS
enum Kind {
  Create = "CREATE",
  Fetch = "FETCH",
  FetchList = "FETCH_LIST",
}

interface Create extends event.Event<Kind> {
  kind: Kind.Create;
  metadata: Metadata;
  path: string;
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

interface FetchList extends event.Event<Kind> {
  kind: Kind.FetchList;
}

type Msg = Create | Fetch | FetchList;

interface CreateInput {
  metadata: Metadata;
  path: string;
}

interface RegisterInput {
  orgId: string;
  projectName: string;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Create:
      creationStore.loading();
      api.post<CreateInput, Project>(`projects`, {
        metadata: msg.metadata,
        path: msg.path,
      })
        .then(creationStore.success)
        .catch(creationStore.error);

      break;
    case Kind.Fetch:
      projectStore.loading();
      api.get<Project>(`projects/${msg.id}`)
        .then(projectStore.success)
        .catch(projectStore.error)

      break;

    case Kind.FetchList:
      projectsStore.loading()
      api.get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
}

export const create = (
  metadata: Metadata,
  path: string,
): Readable<remote.Data<Project>> => {
  const store = remote.createStore<Project>();

  api.post<CreateInput, Project>(`projects`, {
    metadata,
    path,
  })
    .then(store.success)
    .catch(store.error);

  return store.readable;
}

export const register = (
  orgId: string,
  projectName: string,
): Readable<remote.Data<transaction.Transaction>> => {
  const store = remote.createStore<transaction.Transaction>();

  api.post<RegisterInput, transaction.Transaction>(`projects/register`, {
    orgId,
    projectName,
  })
    .then(store.success)
    .catch(store.error)

  return store.readable;
}

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);

// Fetch initial list when the store has been subcribed to for the first time.
projectsStore.start(fetchList);
