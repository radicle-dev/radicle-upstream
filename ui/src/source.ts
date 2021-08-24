// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { format } from "timeago.js";

import * as api from "./api";
import * as error from "./error";
import type { PeerId } from "./identity";
import type { Urn } from "./urn";
import * as proxy from "./proxy";
import type {
  Blob,
  RevisionSelector,
  Branch,
  Tag,
  SourceObject,
  Person,
  CommitHeader,
} from "./proxy/source";
import { RevisionType } from "./proxy/source";
import type * as diff from "./source/diff";

export type { Blob, RevisionSelector, Branch, Tag, Person, CommitHeader };
export { RevisionType };

// TYPES
export type Sha1 = string;
export interface CommitStats {
  additions: number;
  deletions: number;
}

export interface Commit {
  branches: string[];
  diff: diff.Diff;
  header: CommitHeader;
  stats: CommitStats;
  changeset: Record<string, unknown>;
}

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

interface Commits {
  headers: CommitHeader[];
  stats: Stats;
}

export interface CommitsHistory {
  history: CommitHeader[];
  stats: Stats;
}

interface CommitGroup {
  time: string;
  commits: CommitHeader[];
}

export enum ObjectType {
  Blob = "BLOB",
  Tree = "TREE",
}

export interface LocalState {
  branches: string[];
  managed: boolean;
}

export interface Tree extends SourceObject {
  entries: SourceObject[];
}

export interface Readme {
  content: string;
  path: string;
}

export interface Revisions {
  branches: Branch[];
  tags: Tag[];
}

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
  revision: RevisionSelector,
  highlight?: boolean,
  signal?: AbortSignal
): Promise<Blob> => {
  return proxy.client.source.blobGet(
    {
      projectUrn,
      path: encodeURIComponent(path),
      peerId,
      revision,
      highlight: highlight && !isMarkdown(path),
    },
    { abort: signal }
  );
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
  revision: RevisionSelector
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
        history: response.headers,
      };
    });
};

export const fetchReadme = async (
  projectUrn: Urn,
  peerId: PeerId,
  revision: RevisionSelector,
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
    if (blob && !blob.binary && blob.content) {
      return {
        path: blob.path,
        content: blob.content,
      };
    } else {
      return null;
    }
  } catch (err: unknown) {
    error.log(error.fromUnknown(err));
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

export const fetchTree = (
  projectUrn: Urn,
  peerId: PeerId,
  revision: RevisionSelector,
  prefix: string,
  signal?: AbortSignal
): Promise<Tree> => {
  return api.get<Tree>(`source/tree/${projectUrn}`, {
    query: { peerId, revision: { ...revision, peerId }, prefix },
    signal,
  });
};

export const getLocalState = (path: string): Promise<LocalState> => {
  return api.get<LocalState>(`source/local-state`, {
    query: { path },
  });
};

const findReadme = (tree: Tree): string | null => {
  for (const entry of tree.entries) {
    if (entry.info.objectType !== ObjectType.Blob) {
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

export interface GroupedCommitsHistory {
  history: CommitGroup[];
  stats: Stats;
}

// A set of commits grouped by time.
interface CommitGroup {
  time: string;
  commits: CommitHeader[];
}

export const groupCommitHistory = (
  history: CommitsHistory
): GroupedCommitsHistory => {
  return { ...history, history: groupCommits(history.history) };
};

const groupCommits = (commits: CommitHeader[]): CommitGroup[] => {
  const groupedCommits: CommitGroup[] = [];
  let groupDate: Date | undefined = undefined;

  commits = commits.sort((a, b) => {
    if (a.committerTime > b.committerTime) {
      return -1;
    } else if (a.committerTime < b.committerTime) {
      return 1;
    }

    return 0;
  });

  for (const commit of commits) {
    const time = commit.committerTime * 1000;
    const date = new Date(time);
    const isNewDay =
      !groupedCommits.length ||
      !groupDate ||
      date.getDate() < groupDate.getDate() ||
      date.getMonth() < groupDate.getMonth() ||
      date.getFullYear() < groupDate.getFullYear();

    if (isNewDay) {
      groupedCommits.push({
        time: formatGroupTime(time),
        commits: [],
      });
      groupDate = date;
    }
    groupedCommits[groupedCommits.length - 1].commits.push(commit);
  }
  return groupedCommits;
};
