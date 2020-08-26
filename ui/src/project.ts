import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// TYPES.
export interface Metadata {
  name: string;
  defaultBranch: string;
  description?: string;
}

export enum RepoType {
  New = "new",
  Existing = "existing",
}

export interface New {
  type: RepoType.New;
  path: string;
  name: string;
}

export interface Existing {
  type: RepoType.Existing;
  path: string;
}

type Repo = New | Existing;

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

export interface Project {
  id: string;
  shareableEntityIdentifier: string;
  metadata: Metadata;
  stats: Stats;
}

type Projects = Project[];

// STATE
const creationStore = remote.createStore<Project>();
export const creation = creationStore.readable;

const projectStore = remote.createStore<Project>();
export const project = projectStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

// EVENTS
enum Kind {
  Create = "CREATE",
  Fetch = "FETCH",
  FetchList = "FETCH_LIST",
}

interface Create extends event.Event<Kind> {
  kind: Kind.Create;
  input: CreateInput;
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
  repo: Repo;
  description?: string;
  defaultBranch: string;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Create:
      creationStore.loading();
      api
        .post<CreateInput, Project>(`projects`, msg.input)
        .then(creationStore.success)
        .catch(creationStore.error);

      break;
    case Kind.Fetch:
      projectStore.loading();
      api
        .get<Project>(`projects/${msg.id}`)
        .then(projectStore.success)
        .catch(projectStore.error);

      break;

    case Kind.FetchList:
      projectsStore.loading();
      api
        .get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
};

export const create = (input: CreateInput): Promise<Project> => {
  return api.post<CreateInput, Project>(`projects`, input);
};

interface CheckoutInput {
  remote: string;
  branch: string;
  path: string;
}

export const checkout = (
  id: string,
  path: string,
  remote: string,
  branch: string
): Promise<boolean> => {
  return api.post<CheckoutInput, boolean>(`projects/${id}`, {
    branch,
    path,
    remote,
  });
};

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);

// Fetch initial list when the store has been subcribed to for the first time.
projectsStore.start(fetchList);
