import { writable } from "svelte/store";

import * as event from "./event";
import * as remote from "./remote";

import { ValidationStatus } from "./validation";

interface UntrackedProject {
  uri: string;
}

enum Kind {
  Update = "UPDATE",
}

interface Update extends event.Event<Kind> {
  kind: Kind.Update;
  uri: string;
}

type Msg = Update;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Update:
      validation.set({ status: ValidationStatus.Loading });
      setTimeout(() => {
        validation.set({ status: ValidationStatus.Success });
        project.success({ uri: msg.uri });
      }, 1000);
  }
};

// TODO(sos): actual validation store
export const validation = writable({ status: ValidationStatus.NotStarted });

export const project = remote.createStore<UntrackedProject>();

export const updateUri = event.create<Kind, Msg>(Kind.Update, update);
