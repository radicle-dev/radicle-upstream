// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import {
  CreatedFile,
  DeletedFile,
  Diff,
  LineDiff,
  LineDiffType,
  ModifiedFile,
} from "proxy-client/commit";

export type { CreatedFile, DeletedFile, ModifiedFile, Diff };

export const lineNumberR = (line: LineDiff): string | number => {
  switch (line.type) {
    case LineDiffType.Addition: {
      return line.lineNum;
    }
    case LineDiffType.Context: {
      return line.lineNumNew;
    }
    case LineDiffType.Deletion: {
      return " ";
    }
  }
};

export const lineNumberL = (line: LineDiff): string | number => {
  switch (line.type) {
    case LineDiffType.Addition: {
      return " ";
    }
    case LineDiffType.Context: {
      return line.lineNumOld;
    }
    case LineDiffType.Deletion: {
      return line.lineNum;
    }
  }
};

export const lineSign = (line: LineDiff): string => {
  switch (line.type) {
    case LineDiffType.Addition: {
      return "+";
    }
    case LineDiffType.Context: {
      return " ";
    }
    case LineDiffType.Deletion: {
      return "-";
    }
  }
};
