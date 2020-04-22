import { derived, get, writable, Readable } from 'svelte/store'

import * as api from './api'
import * as event from './event'
import * as remote from './remote'

import { HIDDEN_BRANCHES } from "../config"

// TOOLING
const filterBranches = (branches: string[]): string[] =>
  branches.filter(branch => !HIDDEN_BRANCHES.includes(branch));

// TYPES
export enum ObjectType {
  Blob = 'BLOB',
  Tree = 'TREE'
}

interface Info {
  name: string;
  objectType: ObjectType;
  lastCommit: { author: { name: string;
      avatar: string;
    };
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

// STATE
interface Revisions {
  branches: string[];
  tags: string[];
}

const currentPathStore = writable("");
export const currentPath = derived(currentPathStore, $store => $store);

const currentRevisionStore = writable("");
export const currentRevision = derived(currentRevisionStore, $store => $store);

const objectStore = remote.createStore<SourceObject>();
export const object = objectStore.readable;

const revisionsStore = remote.createStore<Revisions>();
export const revisions = revisionsStore.readable;

// EVENTS
enum Kind {
  FetchRevisions = "FETCH_REVISIONS",
  Update = "UPDATE"
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

type Msg = FetchRevisions | Update

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.FetchRevisions:
      api.get<Revisions>(
        `source/revisions/${msg.projectId}`
      )
      .then(revisions => {
        revisionsStore.success({
          branches: filterBranches(revisions.branches),
          tags: revisions.tags,
        })
      })
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

export const fetchRevisions = event.create<Kind, Msg>(Kind.FetchRevisions, update);
export const updateParams = event.create<Kind, Msg>(Kind.Update, update);

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
