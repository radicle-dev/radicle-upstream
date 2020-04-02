// String enums are nice for debugging
export enum GlobalMessageKind {
  Notification = 'notification',
  Project = 'project',
}

export type GlobalMessage =
  | { kind: GlobalMessageKind.Notification, msg: any }
  | { kind: GlobalMessageKind.Project, msg: any }


