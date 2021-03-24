import { format } from "timeago.js";

import * as api from "./api";
import type { Identity, PeerId } from "./identity";
import * as error from "./error";
import type { Urn } from "./urn";

import type * as diff from "./source/diff";

// TYPES
export type Sha1 = string;

export interface Person {
  avatar: string;
  email: string;
  name: string;
}

export interface CommitHeader {
  author: Person;
  committer: Person;
  committerTime: number;
  description: string;
  sha1: Sha1;
  summary: string;
}

export interface CommitStats {
  additions: number;
  deletions: number;
}

export interface Commit {
  branch: string;
  diff: diff.Diff;
  header: CommitHeader;
  stats: CommitStats;
  changeset: Record<string, unknown>;
}

interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

interface Commits {
  headers: CommitHeader[];
  stats: Stats;
}

export interface CommitsHistory {
  history: CommitHistory;
  stats: Stats;
}

interface CommitGroup {
  time: string;
  commits: CommitHeader[];
}

type CommitHistory = CommitGroup[];

export enum ObjectType {
  Blob = "BLOB",
  Tree = "TREE",
}

interface Info {
  name: string;
  objectType: ObjectType;
  lastCommit: CommitHeader;
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
  path: string;
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

export interface MergeRequest {
  id: string;
  merged: boolean;
  peer_id: string;
  identity?: Identity;
  title?: string;
  description?: string;
  commit: string;
}

export interface MergeRequestDetails {
  mergeRequest: MergeRequest;
  commits: CommitsHistory;
}

export interface Sha {
  type: RevisionType.Sha;
  sha: string;
}

export type Revision = Branch | Tag | Sha;

export interface SelectedPath {
  request: AbortController | null;
  selected: string;
}

export interface SelectedRevision {
  request: AbortController | null;
  selected: Branch | Tag;
}

export const fetchBlob = async (
  projectUrn: Urn,
  peerId: string,
  path: string,
  revision: Revision,
  highlight?: boolean,
  signal?: AbortSignal
): Promise<Blob> => {
  return api.get<Blob>(`source/blob/${projectUrn}`, {
    query: {
      path: encodeURIComponent(path),
      peerId,
      revision: { peerId, ...revision },
      highlight: highlight && highlight && !isMarkdown(path),
    },
    signal,
  });
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

export const fetchCommit = (projectUrn: Urn, sha1: Sha1): Promise<Commit> => {
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

export const fetchReadme = async (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision,
  tree: Tree,
  signal?: AbortSignal
): Promise<Readme | null> => {
  const path = findReadme(tree);
  if (!path) {
    return null;
  }

  try {
    const blob = await fetchBlob(
      projectUrn,
      peerId,
      path,
      revision,
      false,
      signal
    );
    if (blob && !blob.binary) {
      return blob;
    } else {
      return null;
    }
  } catch (err) {
    error.log(err);
    return null;
  }
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

export const fetchMergeRequests = (
  projectUrn: Urn
): Promise<MergeRequest[]> => {
  return api.get<MergeRequest[]>(`source/merge_requests/${projectUrn}`);
};

export const fetchMergeRequest = (
  projectUrn: Urn,
  peerId: string,
  id: string
): Promise<MergeRequestDetails> => {
  return api.get<MergeRequestDetails>(`source/merge_request/${projectUrn}/`, {
    query: { peerId, id },
  });
};

export const fetchTree = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: Revision,
  prefix: string,
  signal?: AbortSignal
): Promise<Tree> => {
  return api.get<Tree>(`source/tree/${projectUrn}`, {
    query: { peerId, revision: { ...revision, peerId }, prefix },
    signal,
  });
};

export const getLocalState = (path: string): Promise<LocalState> => {
  return api.get<LocalState>(`source/local-state/${path}`);
};

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

export const formatCommitTime = (t: number): string => {
  return format(t * 1000);
};

const formatGroupTime = (t: number): string => {
  return new Date(t).toLocaleDateString("en-US", {
    month: "long",
    weekday: "long",
    day: "numeric",
    year: "numeric",
  });
};

const groupCommits = (history: CommitHeader[]): CommitHistory => {
  const days: CommitHistory = [];
  let groupDate: Date | undefined = undefined;

  history = history.sort((a, b) => {
    if (a.committerTime > b.committerTime) {
      return -1;
    } else if (a.committerTime < b.committerTime) {
      return 1;
    }

    return 0;
  });

  for (const commit of history) {
    const time = commit.committerTime * 1000;
    const date = new Date(time);
    const isNewDay =
      !days.length ||
      !groupDate ||
      date.getDate() < groupDate.getDate() ||
      date.getMonth() < groupDate.getMonth() ||
      date.getFullYear() < groupDate.getFullYear();

    if (isNewDay) {
      days.push({
        time: formatGroupTime(time),
        commits: [],
      });
      groupDate = date;
    }
    days[days.length - 1].commits.push(commit);
  }
  return days;
};
