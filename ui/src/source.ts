import { derived, get, writable, Readable } from 'svelte/store'

import * as api from './api'
import * as event from './event'
import * as remote from './remote'

import { HIDDEN_BRANCHES } from "../config"

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
  branches?: string[];
  tags?: string[];
}

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
  FetchRevisions = "FETCH_REVISIONS",
  UpdateRevision = "UPDATE_REVISION",
  UpdatePath = "UPDATE_PATH"
}

interface FetchRevisions extends event.Event<Kind> {
  kind: Kind.FetchRevisions;
  projectId: string;
}

interface UpdateRevision extends event.Event<Kind> {
  kind: Kind.UpdateRevision;
  revision: string;
}

interface UpdatePath extends event.Event<Kind> {
  kind: Kind.UpdatePath;
  path: string;
  projectId: string;
  type: ObjectType;
}

type Msg = FetchRevisions | UpdateRevision | UpdatePath

function update(msg: Msg): void {
  console.log("source.update", msg)

  switch (msg.kind) {
    case Kind.FetchRevisions:
      api.get<string[]>(
        `source/branches/${msg.projectId}`
      )
      .then(branches => revisionsStore.success({ branches }))
      .catch(revisionsStore.error);
      break

    case Kind.UpdateRevision:
      currentRevisionStore.update(() => msg.revision);
      break;

    case Kind.UpdatePath:
      currentPathStore.update(() => msg.path)
      objectStore.loading();

      switch (msg.type) {
        case ObjectType.Blob:
          console.log("FETCH BLOB");

          api.get<SourceObject>(
            `source/blob/${msg.projectId}`,
            {
              query: { revision: get(currentRevisionStore), path: msg.path }
            },
          )
            .then(objectStore.success)
            .catch(objectStore.error);
          break;

        case ObjectType.Tree:
          console.log("FETCH TREE", currentRevisionStore);

          api.get<SourceObject>(
            `source/tree/${msg.projectId}`,
            {
              query: { revision: get(currentRevisionStore), prefix: msg.path },
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
export const updateRevision = event.create<Kind, Msg>(Kind.UpdateRevision, update);
export const updatePath = event.create<Kind, Msg>(Kind.UpdatePath, update);

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

// TOOLING
// TODO(sos): filter revisions before passing to store
const filterRevisions = (revisions: { tags: string[]; branches: string[] }) => [
  ...revisions.tags,
  ...revisions.branches.filter(branch => !HIDDEN_BRANCHES.includes(branch))
]
