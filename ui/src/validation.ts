import validatejs from "validate.js"
import { derived, writable, Writable, Readable, get } from "svelte/store"

export enum ValidationStatus {
  NotStarted = "NOT_STARTED",
  FormatValidation = "FORMAT_VALIDATION",
  Pending = "PENDING",
  Error = "ERROR",
  Success = "SUCCESS"
}

type ValidationState =
  { status: ValidationStatus.NotStarted } |
  { status: ValidationStatus.FormatValidation } |
  { status: ValidationStatus.Pending } |
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

export const inputStore = writable("")


export const createValidationStore = (input?: string): ValidationStore => {
  const initialState = { status: ValidationStatus.NotStarted } as ValidationState
  const internalStore = writable(initialState)
  const { subscribe, update } = internalStore

  const delay = (duration: number) =>
    new Promise(resolve => setTimeout(resolve, duration))



  const validate = async (input: string) => {
    update(store => { return { status: ValidationStatus.Pending, input: input } })
    const errors = validatejs({ name: input }, constraints, { fullMessages: false })

    if (errors) {
      update(store => { return { status: ValidationStatus.Error, message: errors.name[0], input: input } })
      return
    } else {
      update(store => { return { status: ValidationStatus.Pending, input: input } })
      await delay(1000)
      update((store) => {
        return { status: ValidationStatus.Success, input: input }
      })
    }
  }

  inputStore.subscribe((input: string) => {
    console.log("validating ", input)
    validate(input)
  })

  return {
    subscribe,
    validate
  }
}



