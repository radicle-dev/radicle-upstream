import * as notification from './notification'
import * as project from './project'

export enum Kind {
  Notification,
  Project,
}

interface MsgInterface {
  kind: Kind;
}

export interface Notification extends MsgInterface {
  kind: Kind.Notification;
  msg: notification.Msg;
}

export interface Project extends MsgInterface {
  kind: Kind.Project;
  msg: project.Msg;
}

export type Msg = Notification | Project;

type State = {
  notification: notification.State,
  project: project.State,
};

let state: State = {
  notification: notification.init(),
  project: project.init(),
};

function update(state: State, msg: Msg): State {
  console.log("update", msg)

  switch (msg.kind) {
    case Kind.Notification:
      state.notification = notification.update(state.notification, msg.msg)
      break
    case Kind.Project:
      state.project = project.update(state.project, msg.msg)
      break
  }

  return state
}

export function emit(msg: Msg): void {
  console.log(msg);
  state = update(state, msg) 
}
