import { get, writable } from "svelte/store";

import * as error from "./error";
import * as api from "./api";
import * as config from "./config";
import * as event from "./event";
import type * as identity from "./identity";
import * as remote from "./remote";
import * as source from "./source";
import type { Urn } from "./urn";
import * as validation from "./validation";
import type * as waitingRoom from "./waitingRoom";

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

export enum Role {
  Contributor = "contributor",
  Maintainer = "maintainer",
  Tracker = "tracker",
}

export enum ReplicationStatusType {
  NotReplicated = "notReplicated",
  Replicated = "replicated",
}

export interface NotReplicated {
  type: ReplicationStatusType.NotReplicated;
}

export interface Replicated {
  type: ReplicationStatusType.Replicated;
  role: Role;
  user: identity.Identity;
}

export type ReplicationStatus = NotReplicated | Replicated;

export enum PeerType {
  Local = "local",
  Remote = "remote",
}

export interface Local {
  type: PeerType.Local;
  peerId: identity.PeerId;
  status: ReplicationStatus;
}

export interface Remote {
  type: PeerType.Remote;
  peerId: identity.PeerId;
  status: ReplicationStatus;
}

export type Peer = Local | Remote;

export interface User {
  peerId: identity.PeerId;
  type: PeerType;
  identity: identity.Identity;
  role: Role;
}

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

export interface Project {
  urn: Urn;
  shareableEntityIdentifier: string;
  metadata: Metadata;
  stats: Stats;
}

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

export interface User {
  identity: identity.Identity;
  role: Role;
}

type Projects = Project[];

// STATE
const creationStore = remote.createStore<Project>();
export const creation = creationStore.readable;

const localStateStore = remote.createStore<source.LocalState>();
export const localState = localStateStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

// EVENTS
enum Kind {
  ClearLocalState = "CLEAR_LOCAL_STATE",
  Create = "CREATE",
  FetchList = "FETCH_LIST",
  FetchTracked = "FETCH_TRACKED",
  FetchLocalState = "FETCH_LOCAL_STATE",
}

interface ClearLocalState extends event.Event<Kind> {
  kind: Kind.ClearLocalState;
}

interface Create extends event.Event<Kind> {
  kind: Kind.Create;
  input: CreateInput;
}

interface FetchList extends event.Event<Kind> {
  kind: Kind.FetchList;
  urn?: string;
}

interface FetchLocalState extends event.Event<Kind> {
  kind: Kind.FetchLocalState;
  path: string;
}

type Msg = ClearLocalState | Create | FetchList | FetchLocalState;

// REQUEST INPUTS
interface CreateInput {
  repo: Repo;
  description?: string;
  defaultBranch: string;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.ClearLocalState:
      localStateStore.reset();
      localStateError.set("");

      break;

    case Kind.Create:
      creationStore.loading();
      api
        .post<CreateInput, Project>(`projects`, msg.input)
        .then(creationStore.success)
        .catch((err: Error) => creationStore.error(error.fromException(err)));

      break;

    case Kind.FetchList:
      projectsStore.loading();

      api
        .get<Projects>("projects/contributed")
        .then(projectsStore.success)
        .catch((err: Error) => projectsStore.error(error.fromException(err)));

      break;

    case Kind.FetchLocalState:
      localStateStore.loading();
      source
        .getLocalState(msg.path)
        .then(localStateStore.success)
        .catch((err: Error) => localStateStore.error(error.fromException(err)));
      break;
  }
};

export const create = (input: CreateInput): Promise<Project> => {
  return api.post<CreateInput, Project>(`projects`, input);
};

interface CheckoutInput {
  peerId?: string;
  path: string;
}

export const checkout = (
  urn: Urn,
  path: string,
  peerId?: identity.PeerId
): Promise<string> => {
  return api.post<CheckoutInput, string>(`projects/${urn}/checkout`, {
    path,
    peerId,
  });
};

export const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);

export const clearLocalState = event.create<Kind, Msg>(
  Kind.ClearLocalState,
  update
);
export const fetchLocalState = event.create<Kind, Msg>(
  Kind.FetchLocalState,
  update
);

export const cancelRequest = (urn: string): Promise<null> => {
  return api.del(`projects/requests/${urn}`);
};

export const fetch = (projectUrn: Urn): Promise<Project> => {
  return api.get<Project>(`projects/${projectUrn}`);
};

export const fetchFailed = (): Promise<Project[]> => {
  return api.get<Project[]>("projects/failed");
};

export const fetchPeers = (
  projectUrn: Urn,
  signal?: AbortSignal
): Promise<Peer[]> => {
  return api.get<Peer[]>(`projects/${projectUrn}/peers`, { signal });
};

export const fetchSearching = (): Promise<waitingRoom.ProjectRequest[]> => {
  return api.get<waitingRoom.ProjectRequest[]>("projects/requests");
};

export const fetchTracking = (): Promise<Project[]> => {
  return api.get<Project[]>("projects/tracked");
};

export const fetchUserList = (urn: string): Promise<Project[]> => {
  return api.get<Projects>(`projects/user/${urn}`);
};

export const trackPeer = (
  projectUrn: Urn,
  peerId: identity.PeerId
): Promise<boolean> => {
  return api.put<null, boolean>(`projects/${projectUrn}/track/${peerId}`, null);
};

export const untrackPeer = (
  projectUrn: Urn,
  peerId: identity.PeerId
): Promise<boolean> => {
  return api.put<null, boolean>(
    `projects/${projectUrn}/untrack/${peerId}`,
    null
  );
};

// NEW PROJECT
export const localStateError = writable<string>("");
export const defaultBranch = writable<string>(config.UPSTREAM_DEFAULT_BRANCH);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

export const formatNameInput = (input: string): string =>
  input.replace(" ", "-");
export const extractName = (repoPath: string): string =>
  repoPath.split("/").slice(-1)[0];

// The default branches supported for preselection when importing
// a new project. Sorted by preference of preselection.
const DEFAULT_BRANCHES = [
  config.UPSTREAM_DEFAULT_BRANCH,
  config.GIT_DEFAULT_BRANCH,
];

const fetchBranches = async (path: string) => {
  fetchLocalState({ path });

  localStateError.set("");

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (!path.length) {
    return;
  }

  let state: source.LocalState;
  try {
    state = await source.getLocalState(path);
  } catch (err) {
    error.log({
      code: error.Code.LocalStateFetchFailure,
      message: err.message,
      source: err,
    });
    localStateError.set(err.message);
    return;
  }

  const foundDefaultBranch = DEFAULT_BRANCHES.find(defaultBranch =>
    state.branches.includes(defaultBranch)
  );

  defaultBranch.set(foundDefaultBranch || state.branches[0]);
};

const validateExistingRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(() => {
    return (
      !get(localStateError).match("could not find repository") &&
      !get(localStateError).match("repository has no branches")
    );
  });
};

const validateNewRepository = async (path: string): Promise<boolean> => {
  await fetchBranches(path);
  return !!get(localStateError).match("could not find repository");
};

const projectNameConstraints = {
  presence: {
    message: "You must provide a display name",
    allowEmpty: false,
  },
  firstHandleChar: {
    valueName: "project name",
  },
  length: {
    minimum: 2,
    maximum: 64,
    tooShort: "Your project name should be at least 2 characters long.",
    tooLong: "Your project name should not be longer than 64 characters.",
  },
  format: {
    pattern: new RegExp(projectNameMatch, "i"),
    message:
      "Your project name has unsupported characters in it. You can only use basic letters, numbers, and the _ , - and . characters.",
  },
};

const projectDescriptionConstraints = {
  length: {
    maximum: 256,
    tooLong:
      "Your project description should not be longer than 256 characters.",
  },
};

export const nameValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(projectNameConstraints);
};

export const descriptionValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(projectDescriptionConstraints);
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
          validationMessage:
            "The directory should contain a git repository with at least one branch",
        },
      ]
    );
  }
};

// Checks if the provided user is part of the maintainer list of the project.
export const isMaintainer = (userUrn: Urn, project: Project): boolean => {
  return project.metadata.maintainers.includes(userUrn);
};

// Checks if any of the contributors in the list is the current user.
export const isContributor = (users: User[]): boolean => {
  return !!users.find(
    u =>
      u.type === PeerType.Local &&
      (u.role === Role.Maintainer || u.role == Role.Contributor)
  );
};
