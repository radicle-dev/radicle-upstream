// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { format } from "timeago.js";

import * as error from "./error";
import type { PeerId } from "./identity";
import * as proxy from "./proxy";
import type {
  Blob,
  Branch,
  Commit,
  CommitHeader,
  Person,
  RevisionSelector,
  SourceObject,
  Stats,
  Tag,
} from "proxy-client/source";
import { RevisionType } from "proxy-client/source";

export type {
  Blob,
  RevisionSelector,
  Branch,
  Tag,
  Person,
  CommitHeader,
  Commit,
  Stats,
};
export { RevisionType };

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
  projectUrn: string,
  peerId: string,
  path: string,
  revision: RevisionSelector,
  highlight?: "dark" | "light" | "h4x0r",
  signal?: AbortSignal
): Promise<Blob> => {
  if (isMarkdown(path)) {
    highlight = undefined;
  }
  return proxy.client.source.blobGet(
    {
      projectUrn,
      path: encodeURIComponent(path),
      peerId,
      revision,
      highlight,
    },
    { abort: signal }
  );
};

export async function fetchCommits(
  projectUrn: string,
  peerId: PeerId,
  revision: RevisionSelector
): Promise<CommitsHistory> {
  const { headers, stats } = await proxy.client.source.commitsGet({
    projectUrn,
    peerId,
    revision,
  });
  return {
    stats,
    history: headers,
  };
}

export const fetchReadme = async (
  projectUrn: string,
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
      undefined,
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

export async function fetchRevisions(
  projectUrn: string,
  peerId?: PeerId
): Promise<Revisions> {
  const [branchNames, tagNames] = await Promise.all([
    proxy.client.source.branchesGet({ projectUrn, peerId }),
    proxy.client.source.tagsGet({ projectUrn, peerId }),
  ]);

  const branches = branchNames.map(
    (name): Branch => ({
      type: RevisionType.Branch,
      name,
    })
  );

  const tags = tagNames.map(
    (name): Tag => ({
      type: RevisionType.Tag,
      name,
    })
  );
  return { branches, tags };
}

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
