import { derived, get, writable, Readable } from "svelte/store";

import * as error from "./error";
import * as api from "./api";
import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "./config";
import * as event from "./event";
import type * as identity from "./identity";
import * as remote from "./remote";
import { getLocalState, LocalState } from "./source";
import type * as urn from "./urn";
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
        return { type: peer.type, peerId: peer.peerId, identity: user, role };
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
  peers: Peer[];
}>> = derived(peersStore, store => {
  if (store.status === remote.Status.Success) {
    const peers = store.data.filter(
      peer => peer.status.type === ReplicationStatusType.NotReplicated
    );

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
        .catch((err: Error) => creationStore.error(error.fromException(err)));

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
        .catch((err: Error) => projectStore.error(error.fromException(err)));
      break;

    // TODO(sos): determine if viewing another user's profile shows you tracked || contributed || (tracked && contributed)
    case Kind.FetchList:
      projectsStore.loading();
      api
        .get<Projects>("projects/contributed")
        .then(projectsStore.success)
        .catch((err: Error) => projectStore.error(error.fromException(err)));

      break;

    case Kind.FetchPeers:
      peersStore.loading();

      api
        .get<Peer[]>(`projects/${msg.urn}/peers`)
        .then(peers => {
          peersStore.success(peers);
        })
        .catch((err: Error) => peersStore.error(error.fromException(err)));

      break;

    case Kind.FetchLocalState:
      localStateStore.loading();
      getLocalState(msg.path)
        .then(localStateStore.success)
        .catch((err: Error) => localStateStore.error(error.fromException(err)));
      break;

    case Kind.TrackPeer:
      api
        .put<null, boolean>(`projects/${msg.urn}/track/${msg.peerId}`, null)
        .then(() => {
          fetchPeers({ urn: msg.urn });
        })
        .catch((err: Error) => peersStore.error(error.fromException(err)));
      break;

    case Kind.UntrackPeer:
      api
        .put<null, boolean>(`projects/${msg.urn}/untrack/${msg.peerId}`, null)
        .then(() => {
          fetchPeers({ urn: msg.urn });
        })
        .catch((err: Error) => peersStore.error(error.fromException(err)));
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

export const clearLocalState = event.create<Kind, Msg>(
  Kind.ClearLocalState,
  update
);

export const cancelRequest = (urn: string): Promise<null> => {
  return api.del(`projects/requests/${urn}`);
};

export const fetchFailed = (): Promise<Project[]> => {
  return api.get<Project[]>("projects/failed");
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

export const trackPeer = (urn: urn.Urn, peerId: identity.PeerId): void =>
  event.create<Kind, Msg>(
    Kind.TrackPeer,
    update
  )({
    urn: urn,
    peerId: peerId,
  });

export const untrackPeer = (urn: urn.Urn, peerId: identity.PeerId): void =>
  event.create<Kind, Msg>(
    Kind.UntrackPeer,
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

  let state;
  try {
    state = await getLocalState(path);
  } catch (err) {
    error.log({
      code: error.Code.LocalStateFetchFailure,
      message: err.message,
      source: err,
    });
    localStateError.set(err.message);
    return;
  }

  if (!state.branches.includes(get(defaultBranch))) {
    defaultBranch.set(state.branches[0]);
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

export const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;

const checkPeerUniqueness = async (peer: string): Promise<boolean> => {
  const thisPeer = get(peersStore);
  if (thisPeer.status === "SUCCESS") {
    return !thisPeer.data.map((peer: Peer) => peer.peerId).includes(peer);
  } else {
    return true;
  }
};

export const peerValidation = validation.createValidationStore(
  {
    format: {
      pattern: VALID_PEER_MATCH,
      message: "This is not a valid remote",
    },
  },
  [
    {
      promise: checkPeerUniqueness,
      validationMessage: "This remote is already being followed",
    },
  ]
);

export const addPeer = async (
  projectId: urn.Urn,
  newRemote: identity.PeerId
): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await peerValidation.validate(newRemote);
  if (get(peerValidation).status !== validation.ValidationStatus.Success)
    return false;

  trackPeer(projectId, newRemote);
  return true;
};

export const removePeer = (
  projectId: urn.Urn,
  remote: identity.PeerId
): void => {
  untrackPeer(projectId, remote);
};

// Checks if the provided user is part of the maintainer list of the project.
// FIXME(xla): Urns should be properly typed.
export const isMaintainer = (userUrn: string, project: Project): boolean => {
  return project.metadata.maintainers.includes(userUrn);
};
