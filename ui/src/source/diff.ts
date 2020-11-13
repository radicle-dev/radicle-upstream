export type Line = string;

export enum LineDiffType {
  Addition = "addition",
  Context = "context",
  Deletion = "deletion",
}

export interface Addition {
  type: LineDiffType.Addition;
  line: Line;
  lineNum: number;
}

export interface Context {
  type: LineDiffType.Context;
  line: Line;
  lineNumNew: number;
  lineNumOld: number;
}

export interface Deletion {
  type: LineDiffType.Deletion;
  line: Line;
  lineNum: number;
}

export type LineDiff = Addition | Deletion | Context;

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

export interface Hunk {
  header: Line;
  lines: LineDiff[];
}

export enum FileDiffType {
  Binary = "binary",
  Plain = "plain",
}

export interface Binary {
  type: FileDiffType.Binary;
}

export interface Plain {
  type: FileDiffType.Plain;
  hunks: Hunk[];
}

export type FileDiff = Binary | Plain;

export interface CopiedFile {
  newPath: string;
  oldPath: string;
}

export type CreatedFile = string;
export type DeletedFile = string;

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
