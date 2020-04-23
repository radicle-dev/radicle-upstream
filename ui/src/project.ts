import { writable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";
import * as transaction from "./transaction";

// Types.
export interface Project {
  id: string;
  metadata: {
    name: string;
    default_branch: string;
    description?: string;
  };
}

type Projects = Project[];

// State.
const projectStore = remote.createStore<Project>();
export const project = projectStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

export const projectNameStore = writable(null);

const registrationStore = remote.createStore<transaction.Transaction>();
export const registration = registrationStore.readable;

// State transitions.
enum Kind {
  Fetch = "FETCH",
  FetchList = "FETCH_LIST",
  Register = "REGISTER",
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

interface FetchList extends event.Event<Kind> {
  kind: Kind.FetchList;
}

interface Register extends event.Event<Kind> {
  kind: Kind.Register;
  orgId: string;
  name: string;
  cocoId?: string;
}

type Msg = Fetch | FetchList | Register;

interface RegisterInput {
  projectName: string;
  orgId: string;
  maybeCocoId?: string;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Fetch:
      projectStore.loading();
      api.get<Project>(`projects/${msg.id}`)
        .then(projectStore.success)
        .catch(projectStore.error)

      break;

    case Kind.FetchList:
      projectsStore.loading();
      api
        .get<Projects>("projects")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;

    case Kind.Register:
      registrationStore.loading();
      api.post<RegisterInput, transaction.Transaction>("projects/register", {
        projectName: msg.name,
        orgId: msg.orgId,
        maybeCocoId: msg.cocoId,
      })
        .then(registrationStore.success)
        .catch(registrationStore.error)

      break;
  }
};

// Events.
export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);
export const register = event.create<Kind, Msg>(Kind.Register, update);

// Fetch initial list when the store has been subcribed to for the first time.
projectsStore.start(fetchList);
