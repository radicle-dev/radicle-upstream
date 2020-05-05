import * as api from "./api"
import { createValidationStore } from "./validation"
import { Transaction } from './transaction';
import { EmojiAvatar } from "./avatar"

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

interface Org {
  name: string;
  avatarFallback: EmojiAvatar;
}

// Api 
export const getOrg = (id: string): Promise<Org> => api.get<Org>(`orgs/${id}`)
export const getNameAvailability = (id: string): Promise<boolean> =>
  getOrg(id).then(org => !org)

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
export const validationStore = () => createValidationStore(nameConstraints, {
  promise: getNameAvailability,
  validationMessage: "Sorry, this name is already taken"
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
