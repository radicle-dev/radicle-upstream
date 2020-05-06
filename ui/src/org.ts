import { createValidationStore } from "./validation";
import { EmojiAvatar } from "./avatar";
import { Transaction } from "./transaction";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// TYPES.
export interface Org {
  name: string;
  avatarFallback: EmojiAvatar;
}

enum Kind {
  Fetch = "FETCH"
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

interface RegisterOrgInput {
  id: string;
}

type Msg = Fetch;

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

// STATE
const orgStore = remote.createStore<Org>();
export const org = orgStore.readable;

// EVENTS
const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Fetch:
      orgStore.loading();
      api
        .get<Org>(`orgs/${msg.id}`)
        .then(orgStore.success)
        .catch(orgStore.error);

      break;
  }
};

// Name validation
const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9]+$");
export const nameConstraints = {
  presence: {
    message: `Org name is required`,
    allowEmpty: false
  },
  format: {
    pattern: VALID_NAME_MATCH,
    message: `Org name should match [a-z0-9][a-z0-9_-]+`
  }
};

export const getOrg = (id: string): Promise<Org> => api.get<Org>(`orgs/${id}`);
export const getNameAvailability = (id: string): Promise<boolean> =>
  getOrg(id).then(org => !org);

// Make sure we make a new one every time
export const validationStore = () =>
  createValidationStore(nameConstraints, {
    promise: getNameAvailability,
    validationMessage: "Sorry, this name is already taken"
  });

// MOCK DATA

// TODO(sos): Replace with actual avatar fallback request once the endpoint
// is ready
export const generateAvatar = (_id: string): EmojiAvatar => {
  return {
    background: {
      r: 0,
      g: 200,
      b: 222
    },
    emoji: "😘"
  };
};


export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const register = (id: string): Promise<Transaction> => {
  return api.post<RegisterOrgInput, Transaction>(`orgs`, {
    id
  });
};
