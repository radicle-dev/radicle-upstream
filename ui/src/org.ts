import * as api from "./api"
import * as user from "./user"
import { createValidationStore } from "./validation"
import { Transaction, MessageType } from './transaction';
import { EmojiAvatar } from "./avatar"
import { Avatar } from '../DesignSystem/Primitive';

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

export interface Org {
  name: string;
  avatarFallback: EmojiAvatar;
}

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

// MOCK DATA
// TODO(sos): Replace with actual avatar fallback request once the endpoint is ready
export const generateAvatar = (id: string): EmojiAvatar => {
  return {
    background: {
      r: 0,
      g: 200,
      b: 222
    },
    emoji: "😘"
  }
}

interface RegisterOrgInput {
  id: string;
}

export const register = (id: string): Promise<Transaction> => {
  return api.post<RegisterOrgInput, Transaction>(`orgs`, {
    id
  });
}

export const registerMemberTransaction = (orgId: string, userId: string) => ({
  messages: [
    {
      type: MessageType.OrgMemberRegistration,
      orgId,
      userId
    }
  ]
})
