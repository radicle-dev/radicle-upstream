// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";

export enum LineDiffType {
  Addition = "addition",
  Context = "context",
  Deletion = "deletion",
}

export interface Addition {
  type: LineDiffType.Addition;
  line: string;
  lineNum: number;
}

export interface Context {
  type: LineDiffType.Context;
  line: string;
  lineNumNew: number;
  lineNumOld: number;
}

export interface Deletion {
  type: LineDiffType.Deletion;
  line: string;
  lineNum: number;
}

export type LineDiff = Addition | Deletion | Context;

const lineDiffSchema: zod.Schema<LineDiff> = zod.union([
  zod.object({
    type: zod.literal(LineDiffType.Addition),
    line: zod.string(),
    lineNum: zod.number(),
  }),
  zod.object({
    type: zod.literal(LineDiffType.Context),
    line: zod.string(),
    lineNumNew: zod.number(),
    lineNumOld: zod.number(),
  }),
  zod.object({
    type: zod.literal(LineDiffType.Deletion),
    line: zod.string(),
    lineNum: zod.number(),
  }),
]);

export interface Binary {
  type: "binary";
}

export interface Plain {
  type: "plain";
  hunks: Array<{
    header: string;
    lines: LineDiff[];
  }>;
}

export type FileDiff = Binary | Plain;

const fileDiffSchema: zod.Schema<FileDiff> = zod.union([
  zod.object({ type: zod.literal("binary") }),
  zod.object({
    type: zod.literal("plain"),
    hunks: zod.array(
      zod.object({ header: zod.string(), lines: zod.array(lineDiffSchema) })
    ),
  }),
]);

export interface CopiedFile {
  newPath: string;
  oldPath: string;
}

export interface CreatedFile {
  diff: FileDiff;
  path: string;
}

export interface DeletedFile {
  diff: FileDiff;
  path: string;
}

export interface ModifiedFile {
  diff: FileDiff;
  path: string;
}

export interface MovedFile {
  newPath: string;
  oldPath: string;
}

export interface Diff {
  copied: CopiedFile[];
  created: CreatedFile[];
  deleted: DeletedFile[];
  modified: ModifiedFile[];
  moved: MovedFile[];
}

const diffSchema: zod.Schema<Diff> = zod.object({
  copied: zod.array(
    zod.object({
      newPath: zod.string(),
      oldPath: zod.string(),
    })
  ),
  created: zod.array(zod.object({ path: zod.string(), diff: fileDiffSchema })),
  deleted: zod.array(zod.object({ path: zod.string(), diff: fileDiffSchema })),
  modified: zod.array(
    zod.object({
      path: zod.string(),
      diff: fileDiffSchema,
    })
  ),
  moved: zod.array(
    zod.object({
      newPath: zod.string(),
      oldPath: zod.string(),
    })
  ),
});

export interface CommitHeader {
  author: Person;
  committer: Person;
  committerTime: number;
  description: string;
  sha1: string;
  summary: string;
}

export interface Person {
  email: string;
  name: string;
}
const personSchema: zod.Schema<Person> = zod.object({
  email: zod.string(),
  name: zod.string(),
});

export const commitHeaderSchema: zod.Schema<CommitHeader> = zod.object({
  author: personSchema,
  committer: personSchema,
  committerTime: zod.number(),
  description: zod.string(),
  sha1: zod.string(),
  summary: zod.string(),
});

export interface CommitStats {
  additions: number;
  deletions: number;
}

export interface Commit {
  branches: string[];
  diff: Diff;
  header: CommitHeader;
  stats: CommitStats;
}

export const commitSchema: zod.Schema<Commit> = zod.object({
  branches: zod.array(zod.string()),
  diff: diffSchema,
  header: commitHeaderSchema,
  stats: zod.object({
    additions: zod.number(),
    deletions: zod.number(),
  }),
});
