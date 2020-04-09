import * as identity from "./identity";
import * as notification from "./notification";
import * as project from "./project"
import * as message from "./message"
import * as transaction from "./transaction"

export enum Kind {
  Notification = "notification",
  Project = "project",
}

interface MsgInterface {
  kind: Kind;
}

export interface NotificationMsg extends MsgInterface {
  kind: Kind.Notification;
  msg: notification.Msg;
}

export interface ProjectMsg extends MsgInterface {
  kind: Kind.Project;
  msg: project.Msg;
}

export type Msg = NotificationMsg | ProjectMsg;

function update(msg: message.Msg): void {
  console.log(msg.kind, msg.msg.kind, msg.msg);

  switch (msg.kind) {
    case message.Kind.Identity:
      identity.update(msg.msg);
      break;
    case message.Kind.Notification:
      // notification.update(state.notification, msg.msg)
      break;
    case message.Kind.Project:
      project.update(msg.msg);
      break;
    case message.Kind.Transaction:
      transaction.update(msg.msg);
      break;
  }
}

export function emit(msg: message.Msg): void {
  update(msg)
}
