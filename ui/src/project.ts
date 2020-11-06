import { derived, get, writable, Readable } from "svelte/store";

import * as api from "./api";
import * as config from "./config";
import * as event from "./event";
import * as identity from "./identity";
import * as remote from "./remote";
import * as source from "./source";
import * as urn from "./urn";
import * as validation from "./validation";
import * as waitingRoom from "./waitingRoom";

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
  urn: urn.Urn;
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

const objectStore = remote.createStore<source.SourceObject>();
export const object = objectStore.readable;

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

const selectedPeerStore = writable<User | null>(null);
export const selectedPeer = derived(
  [selectedPeerStore, peerSelection],
  ([selected, selection]) => {
    if (selected) {
      return selected;
    }

    if (selection.status === remote.Status.Success) {
      return selection.data.default;
    }

    return null;
  }
);

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

const revisionsStore = remote.createStore<source.Revisions>();
export const revisionSelection: Readable<remote.Data<{
  default: source.Branch;
  branches: source.Branch[];
  tags: source.Tag[];
}>> = derived([projectStore, revisionsStore], ([project, store]) => {
  if (store.status === remote.Status.Success) {
    let defaultBranch = store.data.branches[0];

    if (project.status === remote.Status.Success) {
      const projectDefault = store.data.branches.find(
        (branch: source.Branch) =>
          branch.name === project.data.metadata.defaultBranch
      );

      console.log("project", project, projectDefault);

      if (projectDefault) {
        defaultBranch = projectDefault;
      }
    }

    return {
      status: remote.Status.Success,
      data: { ...store.data, default: defaultBranch },
    };
  }

  return store;
});

const selectedRevisionStore = writable<source.Revision | null>(null);
export const selectedRevision = derived(
  [selectedRevisionStore, revisionSelection],
  ([selected, selection]) => {
    if (selected) {
      return selected;
    }

    if (selection.status === remote.Status.Success) {
      return selection.data.default;
    }

    return null;
  }
);

// EVENTS
enum Kind {
  ClearLocalState = "CLEAR_LOCAL_STATE",
  Create = "CREATE",
  Fetch = "FETCH",
  FetchList = "FETCH_LIST",
  FetchObject = "FETCH_OBJECT",
  FetchPeers = "FETCH_PEERS",
  FetchRevisions = "FETCH_REVISIONS",
  FetchTracked = "FETCH_TRACKED",
  FetchLocalState = "FETCH_LOCAL_STATE",
  SelectPeer = "SELECT_PEER",
  SelectRevision = "SELECT_REVISION",
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
  urn: string;
}

interface FetchList extends event.Event<Kind> {
  kind: Kind.FetchList;
  urn?: string;
}

interface FetchLocalState extends event.Event<Kind> {
  kind: Kind.FetchLocalState;
  path: string;
}

interface FetchObject extends event.Event<Kind> {
  kind: Kind.FetchObject;
  type: source.ObjectType;
  projectUrn: urn.Urn;
  peerId: identity.PeerId;
  path: string;
  revision: source.Revision;
}

interface FetchPeers extends event.Event<Kind> {
  kind: Kind.FetchPeers;
  urn: urn.Urn;
}

interface FetchRevisions extends event.Event<Kind> {
  kind: Kind.FetchRevisions;
  urn: urn.Urn;
}

interface SelectPeer extends event.Event<Kind> {
  kind: Kind.SelectPeer;
  peer: User;
}

interface SelectRevision extends event.Event<Kind> {
  kind: Kind.SelectRevision;
  revision: source.Revision;
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
  | FetchLocalState
  | FetchObject
  | FetchPeers
  | FetchRevisions
  | SelectPeer
  | SelectRevision
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
        .get<Project>(`projects/${msg.urn}`)
        .then((project: Project) => {
          projectStore.success(project);
          fetchPeers(msg.urn);
        })
        .catch(projectStore.error);

      break;

    case Kind.FetchList:
      projectsStore.loading();

      api
        .get<Projects>("projects/contributed")
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;

    case Kind.FetchLocalState:
      localStateStore.loading();
      source
        .getLocalState(msg.path)
        .then(localStateStore.success)
        .catch(localStateStore.error);
      break;

    case Kind.FetchObject: {
      objectStore.loading();

      source
        .fetchObject(
          msg.type,
          msg.projectUrn,
          msg.peerId,
          msg.path,
          msg.revision
        )
        .then(objectStore.success)
        .catch(objectStore.error);

      break;
    }

    case Kind.FetchPeers:
      peersStore.loading();

      api
        .get<Peer[]>(`projects/${msg.urn}/peers`)
        .then(peers => {
          peersStore.success(peers);
          fetchRevisions(msg.urn);
        })
        .catch(peersStore.error);

      break;

    case Kind.FetchRevisions: {
      revisionsStore.loading();

      const currentPeer = get(selectedPeer);

      if (!currentPeer) {
        console.log("Can't fetch revisions without selected peer");
        return;
      }

      source
        .fetchRevisions(msg.urn, currentPeer.peerId)
        .then(revisionsStore.success)
        .catch(revisionsStore.error);

      break;
    }

    case Kind.SelectPeer: {
      const current = get(selectedPeer);

      if (msg.peer.peerId !== current.peerId) {
        const currentProject = get(projectStore);
        selectedPeerStore.set(msg.peer);
        fetchRevisions(currentProject.data.urn);
      }

      break;
    }

    case Kind.SelectRevision: {
      const selected = msg.revision as source.Branch | source.Tag;
      const current = get(selectedRevision);

      if (selected.type !== current.type || selected.name !== current.name) {
        selectedRevisionStore.set(msg.revision);
      }

      break;
    }

    case Kind.TrackPeer:
      api
        .put<null, boolean>(`projects/${msg.urn}/track/${msg.peerId}`, null)
        .then(() => {
          fetchPeers(msg.urn);
        })
        .catch(peersStore.error);
      break;

    case Kind.UntrackPeer:
      api
        .put<null, boolean>(`projects/${msg.urn}/untrack/${msg.peerId}`, null)
        .then(() => {
          fetchPeers(msg.urn);
        })
        .catch(peersStore.error);
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
  urn: urn.Urn,
  path: string,
  peerId?: identity.PeerId
): Promise<boolean> => {
  return api.post<CheckoutInput, boolean>(`projects/${urn}/checkout`, {
    path,
    peerId,
  });
};

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchList = event.create<Kind, Msg>(Kind.FetchList, update);
export const fetchObject = (
  type: source.ObjectType,
  projectUrn: urn.Urn,
  peerId: identity.PeerId,
  path: string,
  revision: source.Revision
): void =>
  event.create<Kind, Msg>(
    Kind.FetchObject,
    update
  )({
    type,
    projectUrn,
    peerId,
    path,
    revision,
  });
export const fetchPeers = (projectUrn: urn.Urn): void =>
  event.create<Kind, Msg>(Kind.FetchPeers, update)({ urn: projectUrn });
export const fetchRevisions = (projectUrn: urn.Urn): void =>
  event.create<Kind, Msg>(Kind.FetchRevisions, update)({ urn: projectUrn });
export const selectPeer = (peer: User): void =>
  event.create<Kind, Msg>(Kind.SelectPeer, update)({ peer });
export const selectRevision = (revision: source.Revision): void =>
  event.create<Kind, Msg>(Kind.SelectRevision, update)({ revision });

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
export const defaultBranch = writable<string>(
  config.DEFAULT_BRANCH_FOR_NEW_PROJECTS
);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

export const formatNameInput = (input: string): string =>
  input.replace(" ", "-");
export const extractName = (repoPath: string): string =>
  repoPath.split("/").slice(-1)[0];

const fetchBranches = async (path: string) => {
  fetchLocalState({ path });

  localStateError.set("");
  defaultBranch.set(config.DEFAULT_BRANCH_FOR_NEW_PROJECTS);

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (!path.length) {
    return;
  }

  try {
    const state = await source.getLocalState(path);
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

const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;

const checkPeerUniqueness = (peer: string): Promise<boolean> => {
  return Promise.resolve(
    !get(peersStore)
      .data.map((peer: Peer) => {
        return peer.peerId;
      })
      .includes(peer)
  );
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
