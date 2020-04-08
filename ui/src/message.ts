import * as identity from "./identity";
import * as notification from "./notification";
import * as project from "./project";

// String enums are nice for debugging
export enum Kind {
  Identity = "IDENTITY",
  Notification = "NOTIFICATION",
  Project = "PROJECT",
}

export type Msg =
  | { kind: Kind.Identity, msg: identity.Msg }
  | { kind: Kind.Notification, msg: notification.Msg }
  | { kind: Kind.Project, msg: project.Msg };
