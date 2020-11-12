import { Readable, writable } from "svelte/store";

import * as api from "./api";
import type { PeerId } from "./identity";
import * as remote from "./remote";
import type { Urn } from "./urn";

// TYPES
export interface Person {
  avatar: string;
  email: string;
  name: string;
}

export interface Commit {
  sha1: string;
  branch: string;
  author: Person;
  committer: Person;
  committerTime: number;
  description: string;
  summary: string;
  changeset: Record<string, unknown>;
}

export interface LastCommit {
  author: Person;
  summary: string;
  sha1: string;
  committerTime: number;
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

export interface CommitsHistory {
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
  lastCommit: LastCommit;
}

export interface LocalState {
  branches: string[];
  managed: boolean;
}

export interface SourceObject {
  path: string;
  info: Info;
}

export interface Blob extends SourceObject {
  binary?: boolean;
  html?: boolean;
  content: string;
}

export interface Tree extends SourceObject {
  entries: SourceObject[];
  info: Info;
  path: string;
}

export interface Readme {
  content: string;
  path?: string;
}

export interface Revisions {
  branches: Branch[];
  tags: Tag[];
}

export enum RevisionType {
  Branch = "branch",
  Tag = "tag",
  Sha = "sha",
}

export interface Branch {
  type: RevisionType.Branch;
  name: string;
}

export interface Tag {
  type: RevisionType.Tag;
  name: string;
}

export interface Sha {
  type: RevisionType.Sha;
  sha: string;
}

export type Revision = Branch | Tag | Sha;

// STATE
export const objectType = writable(ObjectType.Tree);
export const resetObjectType = (): void => objectType.set(ObjectType.Tree);
export const objectPath = writable(null);
export const resetObjectPath = (): void => objectPath.set(null);

export const fetchCommit = (projectUrn: Urn, sha1: string): Promise<Commit> => {
  return api.get<Commit>(`source/commit/${projectUrn}/${sha1}`);
};

export const fetchCommits = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision
): Promise<CommitsHistory> => {
  return api
    .get<Commits>(`source/commits/${projectUrn}/`, {
      query: {
        revision: { ...revision, peerId },
      },
    })
    .then(response => {
      return {
        stats: response.stats,
        history: groupCommits(response.headers),
      };
    });
};

export const fetchObject = (
  type: ObjectType,
  projectUrn: Urn,
  peerId: PeerId,
  path: string,
  revision: Revision
): Promise<SourceObject> => {
  switch (type) {
    case ObjectType.Blob: {
      return api.get<SourceObject>(`source/blob/${projectUrn}`, {
        query: {
          path: encodeURIComponent(path),
          peerId,
          revision,
          highlight: !isMarkdown(path),
        },
      });
    }

    case ObjectType.Tree: {
      return api.get<SourceObject>(`source/tree/${projectUrn}`, {
        query: {
          peerId,
          revision,
          prefix: path,
        },
      });
    }
  }
};

export const fetchBranches = (
  projectUrn: Urn,
  peerId?: PeerId
): Promise<Branch[]> => {
  return api
    .get<string[]>(`source/branches/${projectUrn}`, {
      query: {
        peerId,
      },
    })
    .then(names =>
      names.map(name => {
        return { type: RevisionType.Branch, name };
      })
    );
};

export const fetchTags = (projectUrn: Urn, peerId?: PeerId): Promise<Tag[]> => {
  return api
    .get<string[]>(`source/tags/${projectUrn}`, {
      query: {
        peerId,
      },
    })
    .then(names =>
      names.map(name => {
        return { type: RevisionType.Tag, name };
      })
    );
};

export const fetchRevisions = (
  projectUrn: Urn,
  peerId?: PeerId
): Promise<Revisions> => {
  return Promise.all([
    fetchBranches(projectUrn, peerId),
    fetchTags(projectUrn, peerId),
  ]).then(([branches, tags]) => {
    return { branches, tags };
  });
};

export const fetchTree = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision,
  prefix: string
): Promise<Tree> => {
  return api.get<Tree>(`source/tree/${projectUrn}`, {
    query: { peerId, revision: { ...revision, peerId }, prefix },
  });
};

export const getLocalState = (path: string): Promise<LocalState> => {
  return api.get<LocalState>(`source/local-state/${path}`);
};

export const tree = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision,
  prefix: string
): Readable<remote.Data<Tree>> => {
  const treeStore = remote.createStore<Tree>();

  fetchTree(projectUrn, peerId, revision, prefix)
    .then(treeStore.success)
    .catch(treeStore.error);

  return treeStore.readable;
};

const blob = (
  projectId: string,
  peerId: string,
  revision: Revision,
  path: string,
  highlight: boolean
): Promise<Blob> =>
  api.get<Blob>(`source/blob/${projectId}`, {
    query: { highlight, peerId, path, revision: { peerId, ...revision } },
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
  query1: Revision,
  query2: Revision
): boolean => {
  if (
    query1.type === RevisionType.Branch &&
    query2.type === RevisionType.Branch
  ) {
    return query1.name === query2.name;
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

export const fetchReadme = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision,
  tree: Tree
): Promise<Readme | null> => {
  const path = findReadme(tree);

  return new Promise((resolve, _reject) => {
    if (!path) {
      return resolve(null);
    }

    blob(projectUrn, peerId, revision, path, false)
      .then(blob => (blob && !blob.binary ? blob : null))
      .then(resolve)
      .catch(() => resolve(null));
  });
};

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
