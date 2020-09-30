import { writable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";
// import * as waitingRoom from "./waitingRoom";
import { ValidationStatus } from "./validation";

interface UntrackedProject {
  urn: string;
}

enum Kind {
  Update = "UPDATE",
}

interface Update extends event.Event<Kind> {
  kind: Kind.Update;
  urn: string;
}

type Msg = Update;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Update:
      requestStore.loading();
      api
        .get<boolean>(`projects/remote/${formatUrn(msg.urn)}`)
        .then(res => {
          console.log(res);
          requestStore.success(res);
        })
        .catch(requestStore.error);
      break;
  }
};

// TODO(sos): Encapsulate this in a urn.ts module or something so it's easy to
// copy/paste URNs app-wide (see https://github.com/radicle-dev/radicle-upstream/issues/840)
const formatUrn = (urn: string) => (urn[0] === "%" ? urn.split("%")[1] : urn);

const requestStore = remote.createStore<boolean>();
export const request = requestStore.readable;

// TODO(sos): actual validation
export const validation = writable({ status: ValidationStatus.Success });

export const updateUrn = event.create<Kind, Msg>(Kind.Update, update);
