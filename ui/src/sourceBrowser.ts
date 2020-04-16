import { createStore } from './remote'
import { Event, createEvent } from './event'
import { get } from './api'

import { HIDDEN_BRANCHES } from "../config"

export enum ObjectType {
  Blob = 'BLOB',
  Tree = 'TREE'
}

interface Info {
  name: string;
  objectType: ObjectType;
  lastCommit: {
    author: {
      name: string;
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
}

interface SourceBrowser {
  revisions: {
    tags?: string[];
    branches?: string[];
  };
  currentRevision: string;
  object: SourceObject;
}

const aBlob: Blob = {
  path: '~/elsewhere/radicle-upstream/src/file.sth',
  content: "for i in array i++\n\nfor x in y x--",
  info: {
    name: "index.html",
    objectType: ObjectType.Blob,
    lastCommit: {
      author: {
        name: "rafalca romney",
        avatar:
          "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg"
      },
      summary: "a great full commit",
      sha1: "38490328ujijk43849732948239",
      committerTime: 139394343
    }
  }
}

const aTree: Tree = {
  path: '~/elsewhere/radicle-upstream/src',
  info: {
    name: "src",
    objectType: ObjectType.Tree,
    lastCommit: {
      author: {
        name: "rafalca romney",
        avatar:
          "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg"
      },
      summary: "broke all of the tests",
      sha1: "8390djk3ij329489302",
      committerTime: 84390489
    }
  },
  entries: [aBlob]
}

const anotherTree: Tree = {
  path: '~/elsewhere/radicle-upstream',
  info: {
    name: "radicle-upstream",
    objectType: ObjectType.Tree,
    lastCommit: {
      author: {
        name: "rafalca romney",
        avatar:
          "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg"
      },
      summary: "broke all of the tests",
      sha1: "8390djk3ij329489302",
      committerTime: 84390489
    }
  },
  entries: [aBlob, aTree]
}

const dummySourceBrowser = {
  revisions: {
    branches: [
      'master',
      'xla/220-warp-implementation'
    ]
  },
  currentRevision: 'master',
  object: anotherTree
}

const store = createStore<SourceBrowser>()
store.success(dummySourceBrowser)
export const source = store.readable

// TODO(sos): filter revisions before passing to store
const filterRevisions = (revisions: { tags: string[]; branches: string[] }) => [
  ...revisions.tags,
  ...revisions.branches.filter(branch => !HIDDEN_BRANCHES.includes(branch))
]

enum Kind {
  UpdateRevision = "UPDATE_REVISION",
  UpdateSourcePath = "UPDATE_SOURCE_PATH"
}

interface UpdateRevision extends Event<Kind> {
  kind: Kind.UpdateRevision;
  newRevision: string;
}

interface UpdateSourcePath extends Event<Kind> {
  kind: Kind.UpdateSourcePath;
  newPath: string;
}

type Msg = UpdateRevision | UpdateSourcePath

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.UpdateRevision:
      console.log('updating revision: ', msg.newRevision)
      // sourceBrowserStore.loading();
      // get<SourceBrowser>(`/source/browser/endpoint/${msg.newRevision}`)
      //   .then(sourceBrowserStore.success)
      //   .catch(sourceBrowserStore.error)
      break
    case Kind.UpdateSourcePath:
      console.log('updating source path: ', msg.newPath)
    // sourceBrowserStore.success()
  }
}

export const updateRevision = createEvent<Kind, Msg>(Kind.UpdateRevision, update)
export const updateSourcePath = createEvent<Kind, Msg>(Kind.UpdateSourcePath, update)
