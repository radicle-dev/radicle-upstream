import { derived, writable, Readable } from 'svelte/store'

import * as api from "./api"
import * as event from "./event"
import * as identity from "./identity";
import * as remote from "./remote";

import { mockChangeset } from '../lib/commitMocks'
import { HIDDEN_BRANCHES } from "../config"

// TOOLING
const filterBranches = (branches: string[]): string[] =>
  branches.filter(branch => !HIDDEN_BRANCHES.includes(branch));

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

export enum ObjectType {
  Blob = 'BLOB',
  Tree = 'TREE'
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

const currentPathStore = writable("");
export const currentPath = derived(currentPathStore, $store => $store);

const currentRevisionStore = writable("master");
export const currentRevision = derived(currentRevisionStore, $store => $store);

const objectStore = remote.createStore<SourceObject>();
export const object = objectStore.readable;

const revisionsStore = remote.createStore<Revisions>();
export const revisions = revisionsStore.readable;

// EVENTS
enum Kind {
  FetchCommit = "FETCH_COMMIT",
  FetchRevisions = "FETCH_REVISIONS",
  Update = "UPDATE"
}

interface FetchCommit extends event.Event<Kind> {
  kind: Kind.FetchCommit;
  projectId: string;
  sha1: string;
}

interface FetchRevisions extends event.Event<Kind> {
  kind: Kind.FetchRevisions;
  projectId: string;
}

interface Update extends event.Event<Kind> {
  kind: Kind.Update;
  path: string;
  projectId: string;
  revision: string;
  type: ObjectType;
}

type Msg = FetchCommit | FetchRevisions | Update

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.FetchCommit:
      commitStore.loading();

      api.get<Commit>(
        `source/commit/${msg.projectId}/${msg.sha1}`
      )
      .then(commit => {
        commitStore.success({
          // TODO(cloudhead): Fetch branch from backend.
          branch: "master",
          changeset: mockChangeset, ...commit,
        })
      })
      .catch(commitStore.error);
      break;

    case Kind.FetchRevisions:
      api.get<Revisions>(
        `source/revisions/${msg.projectId}`
      )
      .then(revisions =>
        revisionsStore.success(revisions.map(rev => {
          return { ...rev, branches: filterBranches(rev.branches) }
        }))
      )
      .catch(revisionsStore.error);
      break;

    case Kind.Update:
      currentPathStore.update(() => msg.path)
      currentRevisionStore.update(() => msg.revision);
      objectStore.loading();

      switch (msg.type) {
        case ObjectType.Blob:
          api.get<SourceObject>(
            `source/blob/${msg.projectId}`,
            {
              query: { revision: msg.revision, path: msg.path }
            },
          )
            .then(objectStore.success)
            .catch(objectStore.error);
          break;

        case ObjectType.Tree:
          api.get<SourceObject>(
            `source/tree/${msg.projectId}`,
            {
              query: { revision: msg.revision, prefix: msg.path },
            }
          )
            .then(objectStore.success)
            .catch(objectStore.error);
          break;
      }
      break;
  }
}

export const fetchCommit = event.create<Kind, Msg>(Kind.FetchCommit, update);
export const fetchRevisions = event.create<Kind, Msg>(Kind.FetchRevisions, update);
export const updateParams = event.create<Kind, Msg>(Kind.Update, update);

export const getLocalBranches = (path: string): Promise<string[]> => {
  return api.get<string[]>(`source/local-branches/${path}`)
}

export const tree = (
  projectId: string,
  revision: string,
  prefix: string,
): Readable<remote.Data<Tree>> => {
  const treeStore = remote.createStore<Tree>();

  api.get<Tree>(`source/tree/${projectId}`, { query: { revision, prefix } })
        .then(treeStore.success)
        .catch(treeStore.error);

  return treeStore.readable;
}

const blob = (
  projectId: string,
  revision: string,
  path: string,
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
}

export const readme = (
  projectId: string,
  revision: string,
): Readable<remote.Data<Readme | null>> => {
  const readme = remote.createStore<Readme | null>();

  remote
    .chain(objectStore.readable, readme)
    .then((object: SourceObject) => {
      if (object.info.objectType === ObjectType.Tree) {
        const path = findReadme(object as Tree);

        if (path) {
          return blob(projectId, revision, path)
        }
      }

      return null;
    })
    .then(readme.success)
    .catch(readme.error);

  return readme.readable;
}
