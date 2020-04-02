import * as notification from './notification'
import * as project from './project'

import { Writable, derived, readable } from 'svelte/store'

export enum Kind {
  Notification = 'notification',
  Project = 'project',
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

function update(msg: Msg) {
  switch (msg.kind) {
    case Kind.Notification:
      // notification.update(state.notification, msg.msg)
      break
    case Kind.Project:
      project.update(msg.msg)
      break
  }
}

export function emit(msg: Msg): void {
  update(msg)
}
