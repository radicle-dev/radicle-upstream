// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import type { Fetcher, RequestOptions } from "./fetcher";
import {
  Commit,
  CommitHeader,
  Person,
  commitHeaderSchema,
  commitSchema,
} from "./commit";

export type { Commit, CommitHeader, Person };

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

const statsSchema: zod.Schema<Stats> = zod.object({
  branches: zod.number(),
  commits: zod.number(),
  contributors: zod.number(),
});

export interface CommitSummary {
  headers: CommitHeader[];
  stats: Stats;
}

const commitSummarySchema: zod.Schema<CommitSummary> = zod.object({
  headers: zod.array(commitHeaderSchema),
  stats: statsSchema,
});

export enum ObjectType {
  Blob = "BLOB",
  Tree = "TREE",
}

export interface SourceObject {
  path: string;
  info: {
    name: string;
    objectType: ObjectType;
    lastCommit: CommitHeader | null;
  };
}

const sourceObjectSchema = zod.object({
  path: zod.string(),
  info: zod.object({
    name: zod.string(),
    objectType: zod.enum([ObjectType.Blob, ObjectType.Tree]),
    lastCommit: commitHeaderSchema.nullable(),
  }),
});

// See
// https://github.com/radicle-dev/radicle-surf/blob/605e6f40840310c14bfe21d7d8a97ac4204f0ec0/source/src/object/blob.rs#L67-L80
// for the serialization.
export type Blob = SourceObject & BlobContent;

type BlobContent =
  | { binary: false; html: boolean; content: string }
  | { binary: true };

const blobContentSchema: zod.Schema<BlobContent> = zod.union([
  zod.object({
    binary: zod.literal(false),
    html: zod.boolean(),
    content: zod.string(),
  }),
  zod.object({ binary: zod.literal(true) }),
]);

// We can’t explicitly annotate this with the schema type.
//
// See https://github.com/colinhacks/zod/issues/541
const blobSchema = zod.intersection(sourceObjectSchema, blobContentSchema);

export interface LocalState {
  branches: string[];
}

const localStateSchema: zod.Schema<LocalState> = zod.object({
  branches: zod.array(zod.string()),
});

export interface Tree extends SourceObject {
  entries: SourceObject[];
}

const treeSchema: zod.Schema<Tree> = sourceObjectSchema.extend({
  entries: zod.array(sourceObjectSchema),
});

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

export type RevisionSelector = (Branch | Tag | Sha) & { peerId?: string };

interface BlobGetParams {
  projectUrn: string;
  peerId?: string;
  path: string;
  revision: RevisionSelector;
  highlight?: "dark" | "light" | "h4x0r";
}

interface TreeGetParams {
  projectUrn: string;
  peerId: string;
  revision: RevisionSelector;
  prefix: string;
}

interface RefsGetParams {
  projectUrn: string;
  peerId?: string;
}

interface CommitsGetParams {
  projectUrn: string;
  peerId?: string;
  revision: RevisionSelector;
}

interface CommitGetParams {
  projectUrn: string;
  sha1?: string;
}

export class Client {
  private fetcher: Fetcher;

  constructor(fetcher: Fetcher) {
    this.fetcher = fetcher;
  }

  async blobGet(
    params: BlobGetParams,
    options?: RequestOptions
  ): Promise<Blob> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/blob/${params.projectUrn}`,
        query: {
          path: params.path,
          peerId: params.peerId,
          revision: { peerId: params.peerId, ...params.revision },
          highlight: params.highlight,
        },
        options,
      },
      blobSchema
    );
  }

  async branchesGet(
    params: RefsGetParams,
    options?: RequestOptions
  ): Promise<string[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/branches/${params.projectUrn}`,
        query: {
          peerId: params.peerId,
        },
        options,
      },
      zod.array(zod.string())
    );
  }

  async treeGet(
    params: TreeGetParams,
    options?: RequestOptions
  ): Promise<Tree> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/tree/${params.projectUrn}`,
        query: {
          peerId: params.peerId,
          revision: { ...params.revision, peerId: params.peerId },
          prefix: params.prefix,
        },
        options,
      },
      treeSchema
    );
  }

  async tagsGet(
    params: RefsGetParams,
    options?: RequestOptions
  ): Promise<string[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/tags/${params.projectUrn}`,
        query: {
          peerId: params.peerId,
        },
        options,
      },
      zod.array(zod.string())
    );
  }

  async commitsGet(
    params: CommitsGetParams,
    options?: RequestOptions
  ): Promise<CommitSummary> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/commits/${params.projectUrn}`,
        query: {
          revision: {
            ...params.revision,
            peerId: params.peerId,
          },
        },
        options,
      },
      commitSummarySchema
    );
  }

  async commitGet(
    params: CommitGetParams,
    options?: RequestOptions
  ): Promise<Commit> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/commit/${params.projectUrn}/${params.sha1}`,
        options,
      },
      commitSchema
    );
  }

  async localStateGet(
    path: string,
    options?: RequestOptions
  ): Promise<LocalState> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `source/local-state`,
        query: {
          path,
        },
        options,
      },
      localStateSchema
    );
  }
}
