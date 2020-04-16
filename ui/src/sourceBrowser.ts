import { createStore } from './remote'
import { HIDDEN_BRANCHES } from "../config"

interface Info {
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

enum SourceObjectType {
  Blob = 'BLOB',
  Tree = 'TREE'
}

interface SourceObject {
  path: string;
  type: SourceObjectType;
}

interface Blob extends SourceObject {
  type: SourceObjectType.Blob;
  binary?: string;
  content: string;
  info: Info;
}

interface SourceBrowser {
  revisions: {
    tags?: string[];
    branches?: string[];
  };
  currentRevision: string;
  sourceObject: SourceObject;
}

const dummySourceBrowser = {
  revisions: {
    branches: [
      'master',
      'xla/220-warp-implementation'
    ]
  },
  currentRevision: 'master',
  sourceObject: {
    type: SourceObjectType.Blob,
    path: '~/somewhere/file.sth',
    content: "for i in array i++\n\nfor x in y x--",
    info: {
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
  } as Blob
}

const sourceBrowserStore = createStore<SourceBrowser>()
sourceBrowserStore.success(dummySourceBrowser)
export const sourceBrowser = sourceBrowserStore.readable

// TODO(sos or xla): filter revisions before passing to store
const filterRevisions = (revisions: { tags: string[]; branches: string[] }) => [...revisions.tags, ...revisions.branches.filter(branch => !HIDDEN_BRANCHES.includes(branch))]
export const updateRevision = (r: string) => console.log('updating revision', r)

