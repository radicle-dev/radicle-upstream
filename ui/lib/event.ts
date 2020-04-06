import * as notification from './notification'
import * as project from './project'
import { GlobalMessage, GlobalMessageKind } from './messages'
import { Readable, Writable, writable } from 'svelte/store'

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

function update(msg: GlobalMessage) {
  console.log(msg)
  switch (msg.kind) {
    case GlobalMessageKind.Notification:
      // notification.update(state.notification, msg.msg)
      break
    case GlobalMessageKind.Project:
      project.update(msg.msg)
      break
  }
}

export function emit(msg: GlobalMessage): void {
  update(msg)
}

// single routing table 
