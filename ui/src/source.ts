import { Readable } from "svelte/store";

import * as api from "./api";
import * as config from "./config";
import * as event from "./event";
import * as identity from "./identity";
import * as remote from "./remote";

// TOOLING
const filterBranches = (branches: string[]): string[] =>
  branches.filter(branch => !config.HIDDEN_BRANCHES.includes(branch));

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
  changeset: object;
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

interface LocalState {
  branches: string[];
  managed: boolean;
}

interface SourceObject {
  path: string;
  info: Info;
}

interface Blob extends SourceObject {
  binary?: boolean;
  content: string;
}

interface Tree extends SourceObject {
  entries: SourceObject[];
  info: Info;
  path: string;
}

interface Revision {
  user: identity.Identity;
  branches: string[];
  tags: string[];
}

type Revisions = Revision[];

interface Readme {
  content: string;
  path?: string;
}

// STATE
const commitStore = remote.createStore<Commit>();
export const commit = commitStore.readable;

const commitsStore = remote.createStore<CommitHistory>();
export const commits = commitsStore.readable;

const objectStore = remote.createStore<SourceObject>();
export const object = objectStore.readable;

const revisionsStore = remote.createStore<Revisions>();
export const revisions = revisionsStore.readable;

// EVENTS
enum Kind {
  FetchCommit = "FETCH_COMMIT",
  FetchCommits = "FETCH_COMMITS",
  FetchRevisions = "FETCH_REVISIONS",
  FetchObject = "FETCH_OBJECT",
}

interface FetchCommit extends event.Event<Kind> {
  kind: Kind.FetchCommit;
  projectId: string;
  userId: string;
  sha1: string;
}

interface FetchCommits extends event.Event<Kind> {
  kind: Kind.FetchCommits;
  projectId: string;
  userId: string;
  branch: string;
}

interface FetchRevisions extends event.Event<Kind> {
  kind: Kind.FetchRevisions;
  projectId: string;
}

interface FetchObject extends event.Event<Kind> {
  kind: Kind.FetchObject;
  path: string;
  user: string;
  projectId: string;
  revision: string;
  type: ObjectType;
}

const groupCommits = (history: CommitSummary[]): CommitHistory => {
  const days: CommitHistory = [];
  let groupDate = null;

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

type Msg = FetchCommit | FetchCommits | FetchRevisions | FetchObject;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.FetchCommit:
      commitStore.loading();

      api
        // TODO(rudolfs): adjust backend api to accept user msg.userId
        .get<Commit>(`source/commit/${msg.projectId}/${msg.sha1}`)
        .then(commit => {
          commitStore.success({
            // TODO(cloudhead): Fetch branch from backend.
            branch: "master",
            ...commit,
          });
        })
        .catch(commitStore.error);
      break;

    case Kind.FetchCommits:
      commitsStore.loading();

      api
        // TODO(rudolfs): adjust backend api to accept user msg.userId
        .get<CommitSummary[]>(`source/commits/${msg.projectId}/`, {
          query: { branch: msg.branch },
        })
        .then(history => {
          commitsStore.success(groupCommits(history));
        })
        .catch(commitsStore.error);
      break;

    case Kind.FetchRevisions:
      api
        .get<Revisions>(`source/revisions/${msg.projectId}`)
        .then(revisions =>
          revisionsStore.success(
            revisions.map(rev => ({
              ...rev,
              branches: filterBranches(rev.branches),
            }))
          )
        )
        .catch(revisionsStore.error);
      break;

    case Kind.FetchObject:
      objectStore.loading();

      switch (msg.type) {
        case ObjectType.Blob:
          api
            .get<SourceObject>(`source/blob/${msg.projectId}`, {
              query: { revision: msg.revision, path: msg.path },
            })
            .then(objectStore.success)
            .catch(objectStore.error);
          break;

        case ObjectType.Tree:
          api
            .get<SourceObject>(`source/tree/${msg.projectId}`, {
              query: { revision: msg.revision, prefix: msg.path },
            })
            .then(objectStore.success)
            .catch(objectStore.error);
          break;
      }
      break;
  }
};

export const fetchCommit = event.create<Kind, Msg>(Kind.FetchCommit, update);
export const fetchCommits = event.create<Kind, Msg>(Kind.FetchCommits, update);
export const fetchRevisions = event.create<Kind, Msg>(
  Kind.FetchRevisions,
  update
);
export const fetchObject = event.create<Kind, Msg>(Kind.FetchObject, update);

export const getLocalState = (path: string): Promise<LocalState> => {
  return api.get<LocalState>(`source/local-state/${path}`);
};

export const tree = (
  projectId: string,
  revision: string,
  prefix: string
): Readable<remote.Data<Tree>> => {
  const treeStore = remote.createStore<Tree>();

  api
    .get<Tree>(`source/tree/${projectId}`, { query: { revision, prefix } })
    .then(treeStore.success)
    .catch(treeStore.error);

  return treeStore.readable;
};

const blob = (
  projectId: string,
  revision: string,
  path: string
): Promise<Blob> =>
  api.get<Blob>(`source/blob/${projectId}`, { query: { revision, path } });

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

export const readme = (
  projectId: string,
  revision: string
): Readable<remote.Data<Readme | null>> => {
  const readme = remote.createStore<Readme | null>();

  remote
    .chain(objectStore.readable, readme)
    .then((object: SourceObject) => {
      if (object.info.objectType === ObjectType.Tree) {
        const path = findReadme(object as Tree);

        if (path) {
          return blob(projectId, revision, path);
        }
      }

      return null;
    })
    .then(blob => (blob && !blob.binary ? blob : null))
    .then(readme.success)
    .catch(readme.error);

  return readme.readable;
};
