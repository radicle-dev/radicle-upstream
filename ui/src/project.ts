import { derived, get, writable, Readable } from "svelte/store";

import * as api from "./api";
import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "./config";
import * as event from "./event";
import * as identity from "./identity";
import * as remote from "./remote";
import { getLocalState, LocalState } from "./source";
import * as urn from "./urn";
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
  peerId: identity.PeerId;
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
  status: ReplicationStatus;
}

export interface Remote {
  type: PeerType.Remote;
  status: ReplicationStatus;
}

export type Peer = Local | Remote;
export interface User {
  type: PeerType;
  identity: identity.Identity;
  role: Role;
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

const peersStore = remote.createStore<Peer[]>();
export const peerSelection: Readable<remote.Data<{
  default: User;
  peers: User[];
}>> = derived(peersStore, store => {
  if (store.status === remote.Status.Success) {
    const peers = store.data
      .filter(peer => peer.status.type === ReplicationStatusType.Replicated)
      .map(peer => {
        const { role, user } = peer.status as Replicated;
        return { type: peer.type, identity: user, role };
      });

    // TODO(xla): Apply proper heuristic to set default.
    return {
      status: remote.Status.Success,
      data: { default: peers[0], peers },
    };
  }

  return store;
});

export const pendingPeers: Readable<remote.Data<{
  peers: User[];
}>> = derived(peersStore, store => {
  if (store.status === remote.Status.Success) {
    const peers = store.data
      .filter(peer => peer.status.type === ReplicationStatusType.NotReplicated)
      .map(peer => {
        const { peerId } = peer as NotReplicated;
        return { peerId };
      });

    return {
      status: remote.Status.Success,
      data: { peers },
    };
  }

  return store;
});

const projectStore = remote.createStore<Project>();
export const project = projectStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

const trackedStore = remote.createStore<Projects>();
export const tracked = trackedStore.readable;

const localStateStore = remote.createStore<LocalState>();
export const localState = localStateStore.readable;

// EVENTS
enum Kind {
  ClearLocalState = "CLEAR_LOCAL_STATE",
  Create = "CREATE",
  Fetch = "FETCH",
  FetchList = "FETCH_LIST",
  FetchPeers = "FETCH_PEERS",
  FetchTracked = "FETCH_TRACKED",
  FetchLocalState = "FETCH_LOCAL_STATE",
  TrackPeer = "TRACK_PEER",
  UntrackPeer = "UNTRACK_PEER",
}

interface ClearLocalState extends event.Event<Kind> {
  kind: Kind.ClearLocalState;
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

interface FetchPeers extends event.Event<Kind> {
  kind: Kind.FetchPeers;
  urn: urn.Urn;
}

interface FetchTracked extends event.Event<Kind> {
  kind: Kind.FetchTracked;
}

interface FetchLocalState extends event.Event<Kind> {
  kind: Kind.FetchLocalState;
  path: string;
}

interface TrackPeer extends event.Event<Kind> {
  kind: Kind.TrackPeer;
  urn: urn.Urn;
  peerId: identity.PeerId;
}

interface UntrackPeer extends event.Event<Kind> {
  kind: Kind.UntrackPeer;
  urn: urn.Urn;
  peerId: identity.PeerId;
}

type Msg =
  | ClearLocalState
  | Create
  | Fetch
  | FetchList
  | FetchPeers
  | FetchLocalState
  | FetchTracked
  | TrackPeer
  | UntrackPeer;

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
        .catch(creationStore.error);

      break;
    case Kind.Fetch:
      projectStore.loading();
      peersStore.reset();
      api
        .get<Project>(`projects/${msg.id}`)
        .then((project: Project) => {
          projectStore.success(project);
          fetchPeers({ urn: msg.id });
        })
        .catch(projectStore.error);

      break;

    // TODO(sos): determine if viewing another user's profile shows you tracked || contributed || (tracked && contributed)
    case Kind.FetchList:
      projectsStore.loading();
      api
        .get<Projects>("projects/contributed")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;

    case Kind.FetchPeers:
      peersStore.loading();

      api
        .get<Peer[]>(`projects/${msg.urn}/peers`)
        .then(peers => {
          peersStore.success(peers);
        })
        .catch(peersStore.error);

      break;

    case Kind.FetchTracked:
      trackedStore.loading();
      api
        .get<Projects>("projects/tracked")
        .then(trackedStore.success)
        .catch(trackedStore.error);
      break;

    case Kind.FetchLocalState:
      localStateStore.loading();
      getLocalState(msg.path)
        .then(localStateStore.success)
        .catch(localStateStore.error);
      break;

    case Kind.TrackPeer:
      peersStore.loading();
      api
        .put<null, boolean>(`projects/${msg.urn}/track/${msg.peerId}`, null)
        .then(() => {
          fetchPeers({ urn: msg.urn });
        })
        .catch(e => {
          console.log(e);
        });
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
  id: string,
  path: string,
  peerId?: string
): Promise<boolean> => {
  return api.post<CheckoutInput, boolean>(`projects/${id}/checkout`, {
    path,
    peerId,
  });
};

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);
export const fetchPeers = event.create<Kind, Msg>(Kind.FetchPeers, update);
export const fetchLocalState = event.create<Kind, Msg>(
  Kind.FetchLocalState,
  update
);

export const fetchTracked = event.create<Kind, Msg>(Kind.FetchTracked, update);
export const clearLocalState = event.create<Kind, Msg>(
  Kind.ClearLocalState,
  update
);

export const fetchUserList = (urn: string): Promise<Project[]> => {
  return api.get<Projects>(`projects/user/${urn}`);
};

export const trackPeer = (urn: urn.Urn, peerId: identity.PeerId): void =>
  event.create<Kind, Msg>(
    Kind.TrackPeer,
    update
  )({
    urn: urn,
    peerId: peerId,
  });

// NEW PROJECT
export const localStateError = writable<string>("");
export const defaultBranch = writable<string>(DEFAULT_BRANCH_FOR_NEW_PROJECTS);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

export const formatNameInput = (input: string): string =>
  input.replace(" ", "-");
export const extractName = (repoPath: string): string =>
  repoPath.split("/").slice(-1)[0];

const fetchBranches = async (path: string) => {
  fetchLocalState({ path });

  localStateError.set("");
  defaultBranch.set(DEFAULT_BRANCH_FOR_NEW_PROJECTS);

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (!path.length) {
    return;
  }

  try {
    const state = await getLocalState(path);
    if (!state.branches.includes(get(defaultBranch))) {
      defaultBranch.set(state.branches[0]);
    }
  } catch (error) {
    localStateError.set(error.message);
  }
};

const validateExistingRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(() => {
    return (
      !get(localStateError).match("could not find repository") &&
      !get(localStateError).match("repository has no branches")
    );
  });
};

const validateNewRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(() =>
    get(localStateError).match("could not find repository")
  );
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
// FIXME(xla): Urns should be properly typed.
export const isMaintainer = (userUrn: string, project: Project): boolean => {
  return project.metadata.maintainers.includes(userUrn);
};
