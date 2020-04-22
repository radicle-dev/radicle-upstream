import { derived, writable, Readable } from 'svelte/store'

import * as api from './api'
import * as event from './event'
import * as remote from './remote'

import { HIDDEN_BRANCHES } from "../config"

const mockChangeset = {
  summary: {
    additions: 32,
    deletions: 24
  },
  files: [
    {
      path: "core/index.js",
      hunks: [
        {
          expanded: true,
          lines: [
            { num: [192, 192], type: "", content: "/*" },
            { num: [193, 193], type: "", content: " * Say hello" },
            { num: [194, 194], type: "", content: " */" },
            {
              num: [195, null],
              type: "-",
              content:
                "Server.prototype.hello = function (req, contentType) {"
            },
            {
              num: [null, 195],
              type: "+",
              content:
                "Server.prototype.hello = function (req, contentType) {"
            },
            {
              num: [196, 196],
              type: "",
              content: "    var enable = this.options.gzip;"
            },
            {
              num: [197, 197],
              type: "",
              content: "    if (enable && (typeof enable === 'boolean' ||"
            }
          ]
        }
      ]
    },
    {
      path: "core/server.js",
      hunks: [
        {
          expanded: true,
          lines: [
            {
              num: [192, 192],
              type: "",
              content:
                "/* Check if we should consider sending a gzip version of the file based on the"
            },
            {
              num: [193, 193],
              type: "",
              content:
                " * file content type and client's Accept-Encoding header value."
            },
            { num: [194, 194], type: "", content: " */" },
            {
              num: [195, null],
              type: "-",
              content:
                "Server.prototype.ok = function (req, contentType) {"
            },
            {
              num: [null, 195],
              type: "+",
              content:
                "Server.prototype.gzipOk = function (req, contentType) {"
            },
            {
              num: [196, 196],
              type: "",
              content: "    var enable = this.options.gzip;"
            },
            { num: [197, 197], type: "", content: "    if (enable &&" },
            {
              num: [198, 198],
              type: "",
              content: "        (typeof enable === 'boolean' ||"
            }
          ]
        },
        {
          expanded: false,
          header:
            "@@ -206,20 +206,17 @@ Server.prototype.gzipOk = function(req, contentType) {",
          lines: [
            {
              num: [199, 199],
              type: "",
              content:
                "            (contentType && (enable instanceof RegExp) && enable.test(contentType)))) {"
            },
            {
              num: [200, 200],
              type: "",
              content:
                "        var acceptEncoding = req.headers['accept-encoding'];"
            },
            {
              num: [201, 201],
              type: "",
              content:
                "        return acceptEncoding && acceptEncoding.indexOf('gzip') >= 0;"
            },
            { num: [202, 202], type: "", content: "    }" }, { num: [203, 203], type: "", content: "    return false;" },
            { num: [204, 204], type: "", content: "}" },
            { num: [205, 205], type: "", content: "" }
          ]
        },
        {
          expanded: true,
          lines: [
            {
              num: [206, null],
              type: "-",
              content:
                "Server.prototype.respond = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {"
            },
            {
              num: [null, 206],
              type: "+",
              content:
                "/* Send a gzipped version of the file if the options and the client indicate gzip is enabled and"
            },
            {
              num: [null, 207],
              type: "+",
              content:
                " * we find a .gz file mathing the static resource requested."
            },
            { num: [null, 208], type: "+", content: " */" },
            {
              num: [null, 209],
              type: "+",
              content:
                "Server.prototype.respondGzip = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {"
            },
            {
              num: [207, 210],
              type: "",
              content: "    var that = this;"
            },
            {
              num: [208, 211],
              type: "",
              content:
                "    if (files.length == 1 && this.gzipOk(req, contentType)) {"
            },
            {
              num: [209, 212],
              type: "",
              content: "        var gzFile = files[0] + '.gz';"
            },
            {
              num: [210, 213],
              type: "",
              content: "        fs.stat(gzFile, function (e, gzStat) {"
            },
            {
              num: [211, 214],
              type: "",
              content: "            if (!e && gzStat.isFile()) {"
            },
            {
              num: [212, 215],
              type: "",
              content: "                var vary = _headers['Vary'];"
            },
            {
              num: [213, null],
              type: "-",
              content:
                "                _headers['Vary'] = (vary && vary != 'Accept-Encoding'?vary+', ':'')+'Accept-Encoding';"
            },
            {
              num: [null, 216],
              type: "+",
              content:
                "                _headers['Vary'] = (vary && vary != 'Accept-Encoding' ? vary + ', ' : '') + 'Accept-Encoding';"
            },
            {
              num: [214, 217],
              type: "",
              content:
                "                _headers['Content-Encoding'] = 'gzip';"
            },
            {
              num: [215, 218],
              type: "",
              content: "                stat.size = gzStat.size;"
            },
            {
              num: [216, 219],
              type: "",
              content: "                files = [gzFile];"
            },
            {
              num: [217, null],
              type: "-",
              content: "            } else {"
            },
            {
              num: [218, null],
              type: "-",
              content:
                "                console.log('gzip file not found or error finding it', gzFile, String(e), stat.isFile());"
            },
            { num: [219, 220], type: "", content: "            }" },
            {
              num: [220, 221],
              type: "",
              content:
                "            that.respondNoGzip(pathname, status, contentType, _headers, files, stat, req, res, finish);"
            },
            { num: [221, 222], type: "", content: "        });" }
          ]
        }
      ]
    }
  ]
};

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

// STATE
interface Revisions {
  branches: string[];
  tags: string[];
}

const commitStore = remote.createStore<Commit>();
export const commit = commitStore.readable;

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
          changeset: mockChangeset,
          ...commit,
        })
      })
      .catch(commitStore.error);
      break;

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
