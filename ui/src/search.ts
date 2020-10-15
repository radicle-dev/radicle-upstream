import * as api from "./api";
import * as event from "./event";
import * as project from "./project";
import * as remote from "./remote";
import * as validation from "./validation";

// STATE
const projectSearchStore = remote.createStore<project.Project>();
export const projectSearch = projectSearchStore.readable;

// FIXME(xla): Use Request type once serialised and returned by the API.
const projectRequestStore = remote.createStore<boolean>();
export const projectRequest = projectRequestStore.readable;

enum Kind {
  Reset = "RESET",
  RequestProject = "REQUEST_PROJECT",
  SearchProject = "SEARCH_PROJECT",
}

interface Reset extends event.Event<Kind> {
  kind: Kind.Reset;
}

interface RequestProject extends event.Event<Kind> {
  kind: Kind.RequestProject;
  urn: string;
}

interface SearchProject extends event.Event<Kind> {
  kind: Kind.SearchProject;
  urn: string;
}

type Msg = Reset | RequestProject | SearchProject;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Reset:
      projectSearchStore.reset();

      break;

    case Kind.RequestProject:
      projectRequestStore.loading();
      // FIXME(xla): This truly belongs in project.ts.
      api
        .put<null, boolean>(`projects/request/${msg.urn}`, null)
        .then(projectRequestStore.success)
        .catch(projectRequestStore.error);

      break;
    case Kind.SearchProject:
      projectSearchStore.loading();
      // FIXME(xla): A verbatim copy from project.ts fetch, it should be consolidated.
      api
        .get<project.Project>(`projects/${msg.urn}`)
        .then(res => {
          console.log(res);
          projectSearchStore.success(res);
        })
        .catch(projectSearchStore.error);

      break;
  }
};

export const reset = event.create<Kind, Msg>(Kind.Reset, update);
export const requestProject = event.create<Kind, Msg>(
  Kind.RequestProject,
  update
);
export const searchProject = event.create<Kind, Msg>(
  Kind.SearchProject,
  update
);

// URN validation.
const VALID_URN_MATCH = /^rad:git:[1-9A-HJ-NP-Za-km-z]{59}/;
const urnConstraints = {
  format: {
    pattern: VALID_URN_MATCH,
    message: `Not a valid project URN`,
  },
};

export const urnValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(urnConstraints);
