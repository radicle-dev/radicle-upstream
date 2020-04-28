
import { MessageType } from './transaction';

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

const imageUrl =
  "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg";


interface Org {
  name: string;
  avatar: {
    imageUrl: string;
  };
}

// Name validation
const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
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

// DUMMY DATA

export const org: Org = {
  name: "",
  avatar: {
    imageUrl: imageUrl
  }
}

export const transaction = {
  messages: [
    {
      type: MessageType.OrgRegistration,
      orgId: "1234"
    }
  ]
};

export const payer = {
  name: "someone",
  kind: "org",
  avatarFallback: null,
  imageUrl: imageUrl
};

export const subject = {
  name: "",
  kind: "org",
  avatarFallback: null,
  imageUrl: imageUrl
};
