import * as api from "./api"
import * as user from "./user"
import * as event from "./event";
import * as remote from "./remote";

import { createValidationStore } from "./validation"
import { Transaction, MessageType } from './transaction';
import { EmojiAvatar } from "./avatar"
import { Avatar } from '../DesignSystem/Primitive';


// Types
export interface Org {
  name: string;
  avatarFallback: EmojiAvatar;
}

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

// State
const orgStore = remote.createStore<Org>();
export const org = orgStore.readable;


// Api 
export const getOrg = (id: string): Promise<Org> => api.get<Org>(`orgs/${id}`)
export const getNameAvailability = (id: string): Promise<boolean> =>
  getOrg(id).then(org => !org)
const validateUserExistence = (handle: string): Promise<boolean> =>
  user.get(handle).then(user => !!user)

// TODO(sos): replace with actual user avatar once api is ready, probably as remote store
export const mockAvatarUrl = "https://www.washingtonian.com/wp-content/uploads/2017/06/6-30-17-goat-yoga-congressional-cemetery-1.jpg"
export const getUserAvatar = (handle: string): Promise<Avatar | undefined> => new Promise<Avatar>(() => ({ url: mockAvatarUrl }))
// user.get(handle).then((user: user.User | null) => user ? user.avatar : undefined)


// Events
enum Kind {
  Fetch = "FETCH"
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

type Msg = Fetch;

interface RegisterInput {
  id: string;
}

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

export const registerMemberTransaction = (orgId: string, userId: string) => ({
  messages: [
    {
      type: MessageType.OrgMemberRegistration,
      orgId,
      userId
    }
  ]
})

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const register = (id: string): Promise<Transaction> =>
 api.post<RegisterInput, Transaction>(`orgs`, { id });

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

// Make sure we make a new one every time
export const orgNameValidationStore = () => createValidationStore(nameConstraints, {
  promise: getNameAvailability,
  validationMessage: "Sorry, this name is already taken"
})

const memberNameConstraints = {
  presence: {
    message: "Member name is required",
    allowEmpty: false
  }
}

export const memberNameValidationStore = () => createValidationStore(memberNameConstraints, {
  promise: validateUserExistence,
  validationMessage: "Cannot find this user"
})
