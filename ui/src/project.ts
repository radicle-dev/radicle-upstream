import { get, writable } from "svelte/store";

import * as api from "./api";
import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "./config";
import * as currency from "./currency";
import * as event from "./event";
import * as org from "./org";
import * as remote from "./remote";
import { getLocalState, LocalState } from "./source";
import * as transaction from "./transaction";
import * as user from "./user";
import * as validation from "./validation";

// TYPES.
export interface Metadata {
  name: string;
  defaultBranch: string;
  description?: string;
  maintainers: string[];
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
  registration?: org.Org | user.User;
}

type Projects = Project[];

// The domain under which a registered project falls
export enum Domain {
  User = "user",
  Org = "org",
}

export interface Registered {
  domainType: Domain;
  domainId: string;
  name: string;
  maybeProjectId?: string;
}

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
  urn?: string;
}

type Msg = Create | Fetch | FetchList;

interface CreateInput {
  repo: Repo;
  description?: string;
  defaultBranch: string;
}

interface RegisterInput {
  transactionFee: currency.MicroRad;
  maybeCocoId?: string;
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
        .get<Projects>(msg.urn ? `projects/?user=${msg.urn}` : "projects")
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

export const getOrgProject = (
  orgId: string,
  projectName: string
): Promise<Registered> => {
  return api.get<Registered>(`orgs/${orgId}/projects/${projectName}`);
};

// Resolve the api base for the given project domain
const apiBase = (domain: Domain): string => {
  switch (domain) {
    case Domain.Org:
      return "orgs";
    case Domain.User:
      return "users";
  }
};

export const register = (
  domainType: Domain,
  domainId: string,
  projectName: string,
  transactionFee: currency.MicroRad,
  maybeCocoId?: string
): Promise<transaction.Transaction> => {
  const base = apiBase(domainType);
  return api.post<RegisterInput, transaction.Transaction>(
    `${base}/${domainId}/projects/${projectName}`,
    {
      transactionFee,
      maybeCocoId,
    }
  );
};

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);

// Fetch initial list when the store has been subcribed to for the first time.
projectsStore.start(fetchList);

// NEW PROJECT

export const localState = writable<LocalState | string>("");
export const localStateError = writable<string>("");
export const defaultBranch = writable<string>(DEFAULT_BRANCH_FOR_NEW_PROJECTS);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

const fetchBranches = async (path: string) => {
  // Revert to defaults whenever the path changes in case this query fails
  // or the user clicks cancel in the directory selection dialog.
  localState.set("");
  localStateError.set("");
  defaultBranch.set(DEFAULT_BRANCH_FOR_NEW_PROJECTS);

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (path === "") {
    return;
  }

  try {
    const state = await getLocalState(path);
    localState.set(state);
    if (!state.branches.includes(get(defaultBranch))) {
      defaultBranch.set(state.branches[0]);
    }
  } catch (error) {
    localStateError.set(error.message);
  }
};

const validateExistingRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(
    () => !get(localStateError).match("could not find repository")
  );
};

const validateNewRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(() =>
    get(localStateError).match("could not find repository")
  );
};

export const nameValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore({
    format: {
      pattern: new RegExp(projectNameMatch, "i"),
      message: `Project name should match ${projectNameMatch}`,
    },
  });
};

export const repositoryPathValidationStore = (
  newRepository: boolean
): validation.ValidationStore => {
  if (newRepository) {
    return validation.createValidationStore(
      {
        presence: {
          message: "Pick a directory for the new project",
          allowEmpty: false,
        },
      },
      [
        {
          promise: validateNewRepository,
          validationMessage:
            "Please choose a directory that's not already a git repository.",
        },
      ]
    );
  } else {
    return validation.createValidationStore(
      {
        presence: {
          message: "Pick a directory with an existing repository",
          allowEmpty: false,
        },
      },
      [
        {
          promise: validateExistingRepository,
          validationMessage: "The directory should contain a git repository",
        },
      ]
    );
  }
};
