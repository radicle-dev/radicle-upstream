import validatejs from "validate.js"
import { writable, Writable, Readable, get } from "svelte/store"

export enum ValidationStatus {
  NotStarted = "NOT_STARTED",
  Loading = "LOADING",
  Error = "ERROR",
  Success = "SUCCESS"
}

type ValidationState =
  { status: ValidationStatus.NotStarted } |
  { status: ValidationStatus.Loading } |
  { status: ValidationStatus.Error; message: string } |
  { status: ValidationStatus.Success }

interface ValidationStore extends Readable<ValidationState> {
  validate: (input: string) => void;
  updateInput: (input: string) => void;
}

// TODO(sos): While we're figuring out consistent validations, this method makes
// it easier to derive a ValidationState from an existing validatejs response
export const getValidationState = (entity: string, validationErrors: { [key: string]: string[] }): ValidationState => {
  if (validationErrors && validationErrors[entity]) {
    return {
      status: ValidationStatus.Error,
      message: validationErrors[entity][0]
    };
  }

  return { status: ValidationStatus.Success };
}

export const createValidationStore = (constraints: any): ValidationStore => {
  const initialState = { status: ValidationStatus.NotStarted } as ValidationState
  const internalStore = writable(initialState)
  const { subscribe, update } = internalStore
  let inputStore: Writable<string> | undefined = undefined

  const delay = (duration: number): Promise<void> =>
    new Promise(resolve => setTimeout(resolve, duration))

  const validate = async (input: string): Promise<void> => {
    // Always start with Loading
    update(() => { return { status: ValidationStatus.Loading, input: input } })

    // Check for errors
    const errors = validatejs({ input: input }, { input: constraints }, { fullMessages: false })

    if (errors) {
      update(() => { return { status: ValidationStatus.Error, message: errors.input[0] } })
      return
    } else {
      // Check remote
      await delay(1000).then(() =>
        update((store) => {
          // If the input has changed since this request was fired off, don't update
          if (get(inputStore) !== input) return store
          return { status: ValidationStatus.Success }
        })
      )
    }
  }

  const updateInput = (input: string): void => {
    if (!inputStore) {
      inputStore = writable(input)
      inputStore.subscribe((input: string) => { validate(input) })
      return
    }
    inputStore.set(input)
  }

  return {
    subscribe,
    validate,
    updateInput
  }
}
