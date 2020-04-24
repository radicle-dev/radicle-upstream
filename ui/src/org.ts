import validatejs from "validate.js"
import { derived, writable, Readable, get } from "svelte/store"

import { MessageType } from './transaction';
import { store } from './identity';

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

export enum ValidationStatus {
  NotStarted = "NOT_STARTED",
  FormatValidation = "FORMAT_VALIDATION",
  RemoteValidation = "REMOTE_VALIDATION",
  Error = "ERROR",
  Success = "SUCCESS"
}

type ValidationState =
  { status: ValidationStatus.NotStarted } |
  { status: ValidationStatus.FormatValidation } |
  { status: ValidationStatus.RemoteValidation } |
  { status: ValidationStatus.Error; message: string } |
  { status: ValidationStatus.Success }

interface ValidationStore extends Readable<ValidationState> {
  validate: (input: string) => void;
}


const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
const constraints = {
  name: {
    presence: {
      message: `Org name is required`,
      allowEmpty: false
    },
    format: {
      pattern: VALID_NAME_MATCH,
      message: `Org name should match [a-z0-9][a-z0-9_-]+`
    }
  }
};

export const createValidationStore = (): ValidationStore => {
  const initialState = { status: ValidationStatus.NotStarted } as ValidationState
  const internalStore = writable(initialState)
  const { subscribe, update } = internalStore

  const validateFormat = (input: string): ValidationState => {
    const validations = validatejs({ name: input }, constraints, { fullMessages: false })

    if (validations) {
      return { status: ValidationStatus.Error, message: validations.name[0] }
    } else {
      return { status: ValidationStatus.Success }
    }
  }

  const validate = (input: string) => {
    const currentStatus = get(internalStore).status
    const val: ValidationState = validateFormat(input)

    // switch (currentStatus) {
    //   case ValidationStatus.NotStarted:
    //     val = validateFormat(input)
    //   case ValidationStatus.RemoteValidation:
    //   case ValidationStatus.Success:
    //   case ValidationStatus.Error:

    // }

    update(() => { return val })
  }

  return {
    subscribe,
    validate
  }
}


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
