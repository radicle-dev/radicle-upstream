import validatejs from "validate.js"
import { derived, writable, Readable, get } from "svelte/store"

export enum ValidationStatus {
  NotStarted = "NOT_STARTED",
  FormatValidation = "FORMAT_VALIDATION",
  Pending = "PENDING",
  Error = "ERROR",
  Success = "SUCCESS"
}

type ValidationState =
  { status: ValidationStatus.NotStarted } |
  { status: ValidationStatus.FormatValidation; input: string } |
  { status: ValidationStatus.Pending; input: string } |
  { status: ValidationStatus.Error; input: string; message: string } |
  { status: ValidationStatus.Success; input: string }

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

export const createValidationStore = (input?: string): ValidationStore => {
  const initialState = { status: ValidationStatus.NotStarted } as ValidationState
  const internalStore = writable(initialState)
  const { subscribe, update } = internalStore

  const delay = (duration: number) =>
    new Promise(resolve => setTimeout(resolve, duration))

  const validate = async (input: string) => {
    const errors = validatejs({ name: input }, constraints, { fullMessages: false })

    if (errors) {
      update(store => { return { status: ValidationStatus.Error, message: errors.name[0], input: input } })
      return
    } else {
      update(store => { return { status: ValidationStatus.Pending, input: input } })
      await delay(1000)
      update(store => { return { status: ValidationStatus.Success, input: input } })
    }
  }

  return {
    subscribe,
    validate
  }
}
