import { Readable, writable } from "svelte/store";

import * as api from "./api";
import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "./config";
import * as event from "./event";
import * as identity from "./identity";
import * as remote from "./remote";

// TYPES
interface Person {
  avatar: string;
  email: string;
  name: string;
}

interface Commit {
  sha1: string;
  branch: string;
  author: Person;
  committer: Person;
  committerTime: number;
  description: string;
  summary: string;
  changeset: Record<string, unknown>;
}

interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

interface Commits {
  headers: CommitSummary[];
  stats: Stats;
}

export interface CommitsStore {
  history: CommitHistory;
  stats: Stats;
}

interface CommitSummary {
  sha1: string;
  author: Person;
  committer: Person;
  committerTime: number;
  summary: string;
  description: string;
}

interface CommitGroup {
  time: number;
  commits: CommitSummary[];
}

type CommitHistory = CommitGroup[];

export enum ObjectType {
  Blob = "BLOB",
  Tree = "TREE",
}

interface Info {
  name: string;
  objectType: ObjectType;
  lastCommit: {
    author: Person;
    summary: string;
    sha1: string;
    committerTime: number;
  };
}

export interface LocalState {
  branches: string[];
  managed: boolean;
}

interface SourceObject {
  path: string;
  info: Info;
}

interface Blob extends SourceObject {
  binary?: boolean;
  html?: boolean;
  content: string;
}

interface Tree extends SourceObject {
  entries: SourceObject[];
  info: Info;
  path: string;
}

// All revisions of project from a particular peer
export interface PeerRevisions {
  // The identity of the peer
  identity: identity.Identity;
  // All known branches
  branches: Branch[];
  // All known tags
  tags: Tag[];
  // Default branch, if we have it
  defaultBranch?: string;
}

interface Readme {
  content: string;
  path?: string;
}

export enum RevisionType {
  Branch = "branch",
  Tag = "tag",
  Sha = "sha",
}

// Client representation of a git branch
export interface Branch {
  type: RevisionType.Branch;
  name: string;
  // The id of the project this branch belongs to
  projectId: string;
  // The id of the peer this branch belongs to
  peerId: string;
}

// Client representation of a git tag
export interface Tag {
  type: RevisionType.Tag;
  name: string;
  // The id of the project this tag belongs to
  peerId: string;
  // The id of the project this tag belongs to
  projectId: string;
}

export interface Sha {
  type: RevisionType.Sha;
  sha: string;
}

// Currently supported revision types in UI:
export type SupportedRevision = Branch | Tag;

export type RevisionQuery = Branch | Tag | Sha;

// Proxy representation of all known revisions of a project from a particular peer
type PeerRevisionsResponse = {
  // The identity of the peer who owns these revisions
  identity: identity.Identity;
  // Names of associated branches and tags
  branches: string[];
  tags: string[];
}[];

// Remote stores
const commitStore = remote.createStore<Commit>();
export const commit = commitStore.readable;

const commitsStore = remote.createStore<CommitsStore>();
export const commits = commitsStore.readable;

const objectStore = remote.createStore<SourceObject>();
export const object = objectStore.readable;

// Updated when the project store changes (i.e. when the user navigates to a new project)
const revisionsStore = remote.createStore<PeerRevisions[]>();
export const revisions = revisionsStore.readable;
revisions.subscribe(store => {
  if (store.status === remote.Status.Success) {
    // The first `PeerRevisions` in the response belongs to the default peer.
    const defaultRevisions = store.data[0];

    const peerId = defaultRevisions.identity.peerId;
    updateCurrentPeerId({ peerId });

    // Now that we have a peer, set the current revision to the default branch.
    // If not found, use the first branch returned from proxy.
    const defaultRevision =
      defaultRevisions.branches.find(
        branch => branch.name === defaultRevisions.defaultBranch
      ) || defaultRevisions.branches[0];
    updateCurrentRevision({ revision: defaultRevision });
  }
});

// Local stores
export const objectType = writable(ObjectType.Tree);
export const resetObjectType = (): void => objectType.set(ObjectType.Tree);
export const objectPath = writable(null);
export const resetObjectPath = () => objectPath.set(null);

export const currentRevision = writable<Branch | Tag | undefined>(undefined);
currentRevision.subscribe(revision => {
  if (revision) {
    // Fetch commits when the current revision is changed
    fetchCommits({ projectId: revision.projectId, revision });
  }
});

export const resetCurrentRevision = () => currentRevision.set(undefined);

export const currentPeerId = writable<string | undefined>(undefined);
export const resetCurrentPeerId = () =>
  updateCurrentPeerId({ peerId: undefined });

// EVENTS
enum Kind {
  FetchCommit = "FETCH_COMMIT",
  FetchCommits = "FETCH_COMMITS",
  FetchRevisions = "FETCH_REVISIONS",
  FetchObject = "FETCH_OBJECT",

  UpdateCurrentPeerId = "UPDATE_CURRENT_PEER_ID",
  UpdateCurrentRevision = "UPDATE_CURRENT_REVISION",
}

interface FetchCommit extends event.Event<Kind> {
  kind: Kind.FetchCommit;
  projectId: string;
  peerId: string;
  sha1: string;
}

interface FetchCommits extends event.Event<Kind> {
  kind: Kind.FetchCommits;
  projectId: string;
  revision: Branch | Tag;
}

interface FetchRevisions extends event.Event<Kind> {
  kind: Kind.FetchRevisions;
  projectId: string;
  defaultBranch?: string;
}

interface FetchObject extends event.Event<Kind> {
  kind: Kind.FetchObject;
  path: string;
  peerId: string;
  projectId: string;
  revision: RevisionQuery;
  type: ObjectType;
}

interface UpdateCurrentPeerId extends event.Event<Kind> {
  kind: Kind.UpdateCurrentPeerId;
  peerId: string;
}

interface UpdateCurrentRevision extends event.Event<Kind> {
  kind: Kind.UpdateCurrentRevision;
  revision: Branch | Tag;
}

const groupCommits = (history: CommitSummary[]): CommitHistory => {
  const days: CommitHistory = [];
  let groupDate: Date | undefined = undefined;

  for (const commit of history) {
    const time = commit.committerTime;
    const date = new Date(time * 1000);
    const isNewDay =
      !days.length ||
      !groupDate ||
      date.getDate() < groupDate.getDate() ||
      date.getMonth() < groupDate.getMonth() ||
      date.getFullYear() < groupDate.getFullYear();

    if (isNewDay) {
      days.push({
        time: time,
        commits: [],
      });
      groupDate = date;
    }
    days[days.length - 1].commits.push(commit);
  }
  return days;
};

type Msg =
  | FetchCommit
  | FetchCommits
  | FetchRevisions
  | FetchObject
  | UpdateCurrentPeerId
  | UpdateCurrentRevision;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.FetchCommit:
      commitStore.loading();

      api
        .get<Commit>(`source/commit/${msg.projectId}/${msg.sha1}`)
        .then(commitStore.success)
        .catch(commitStore.error);
      break;

    case Kind.FetchCommits:
      commitsStore.loading();

      api
        .get<Commits>(`source/commits/${msg.projectId}/`, {
          query: {
            peerId: msg.revision.peerId,
            branch: msg.revision.name,
          },
        })
        .then(response => {
          commitsStore.success({
            stats: response.stats,
            history: groupCommits(response.headers),
          });
        })
        .catch(commitsStore.error);
      break;

    case Kind.FetchRevisions:
      api
        .get<PeerRevisionsResponse>(`source/revisions/${msg.projectId}`)
        .then(response => {
          const revisions: PeerRevisions[] = response.map(rev => ({
            identity: rev.identity,
            branches: rev.branches.map(name => ({
              name,
              type: RevisionType.Branch,
              projectId: msg.projectId,
              peerId: rev.identity.peerId,
            })),
            tags: rev.tags.map(name => ({
              name,
              type: RevisionType.Tag,
              projectId: msg.projectId,
              peerId: rev.identity.peerId,
            })),
            defaultBranch: msg.defaultBranch || DEFAULT_BRANCH_FOR_NEW_PROJECTS,
          }));
          revisionsStore.success(revisions);
        })
        .catch(revisionsStore.error);
      break;

    case Kind.FetchObject:
      objectStore.loading();

      switch (msg.type) {
        case ObjectType.Blob:
          api
            .get<SourceObject>(`source/blob/${msg.projectId}`, {
              query: {
                peerId: msg.peerId,
                revision: msg.revision,
                path: msg.path,
                highlight: !isMarkdown(msg.path),
              },
            })
            .then(objectStore.success)
            .catch(objectStore.error);
          break;

        case ObjectType.Tree:
          api
            .get<SourceObject>(`source/tree/${msg.projectId}`, {
              query: {
                peerId: msg.peerId,
                revision: msg.revision,
                prefix: msg.path,
              },
            })
            .then(objectStore.success)
            .catch(objectStore.error);
          break;
      }
      break;

    case Kind.UpdateCurrentPeerId:
      currentPeerId.set(msg.peerId);
      break;

    case Kind.UpdateCurrentRevision:
      currentRevision.set(msg.revision);
  }
};

export const fetchCommit = event.create<Kind, Msg>(Kind.FetchCommit, update);
export const fetchCommits = event.create<Kind, Msg>(Kind.FetchCommits, update);
export const fetchRevisions = event.create<Kind, Msg>(
  Kind.FetchRevisions,
  update
);
export const fetchObject = event.create<Kind, Msg>(Kind.FetchObject, update);
export const updateCurrentPeerId = event.create<Kind, Msg>(
  Kind.UpdateCurrentPeerId,
  update
);
export const updateCurrentRevision = event.create<Kind, Msg>(
  Kind.UpdateCurrentRevision,
  update
);

export const getLocalState = (path: string): Promise<LocalState> => {
  return api.get<LocalState>(`source/local-state/${path}`);
};

export const tree = (
  projectId: string,
  peerId: string,
  revision: RevisionQuery,
  prefix: string
): Readable<remote.Data<Tree>> => {
  const treeStore = remote.createStore<Tree>();

  api
    .get<Tree>(`source/tree/${projectId}`, {
      query: { peerId: peerId, revision, prefix },
    })
    .then(treeStore.success)
    .catch(treeStore.error);

  return treeStore.readable;
};

const blob = (
  projectId: string,
  peerId: string,
  revision: RevisionQuery,
  path: string,
  highlight: boolean
): Promise<Blob> =>
  api.get<Blob>(`source/blob/${projectId}`, {
    query: { revision, peerId, path, highlight },
  });

const findReadme = (tree: Tree): string | null => {
  for (const entry of tree.entries) {
    if (entry.info.objectType != ObjectType.Blob) {
      continue;
    }
    if (/^readme\b/i.test(entry.path)) {
      return entry.path;
    }
  }
  return null;
};

export const isMarkdown = (path: string): boolean => {
  return /\.(md|mkd|markdown)$/i.test(path);
};

export const formatTime = (t: number): string => {
  return new Date(t).toLocaleDateString("en-US", {
    month: "long",
    weekday: "long",
    day: "numeric",
    year: "numeric",
  });
};

export const revisionQueryEq = (
  query1: RevisionQuery,
  query2: RevisionQuery
): boolean => {
  if (
    query1.type === RevisionType.Branch &&
    query2.type === RevisionType.Branch
  ) {
    return query1.name === query2.name && query1.peerId === query2.peerId;
  } else if (
    query1.type === RevisionType.Tag &&
    query2.type === RevisionType.Tag
  ) {
    return query1.name === query2.name;
  } else if (
    query1.type === RevisionType.Sha &&
    query2.type === RevisionType.Sha
  ) {
    return query1.sha === query2.sha;
  } else {
    return false;
  }
};

export const readme = (
  projectId: string,
  peerId: string,
  revision: RevisionQuery
): Readable<remote.Data<Readme | null>> => {
  const readme = remote.createStore<Readme | null>();

  remote
    .chain(objectStore.readable, readme)
    .then((object: SourceObject) => {
      if (object.info.objectType === ObjectType.Tree) {
        const path = findReadme(object as Tree);

        if (path) {
          return blob(projectId, peerId, revision, path, false);
        }
      }

      return null;
    })
    .then(blob => (blob && !blob.binary ? blob : null))
    .then(readme.success)
    .catch(readme.error);

  return readme.readable;
};
