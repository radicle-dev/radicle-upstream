import validatejs from "validate.js"
import { writable, Writable, Readable, get } from "svelte/store"

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
  updateInput: (input: string) => void;
}


export const createValidationStore = (constraints: any): ValidationStore => {
  const initialState = { status: ValidationStatus.NotStarted } as ValidationState
  const internalStore = writable(initialState)
  const { subscribe, update } = internalStore
  let inputStore: Writable<string> | undefined = undefined

  const delay = (duration: number): Promise<void> =>
    new Promise(resolve => setTimeout(resolve, duration))

  const validate = async (input: string): Promise<void> => {
    // Always start with Pending
    update(() => { return { status: ValidationStatus.Pending, input: input } })

    // Check for errors
    const errors = validatejs({ input: input }, { input: constraints }, { fullMessages: false })

    if (errors) {
      update(() => { return { status: ValidationStatus.Error, message: errors.input[0], input: input } })
      return
    } else {
      // Check remote
      update(() => { return { status: ValidationStatus.Pending, input: input } })
      await delay(1000).then(() =>
        update((store) => {
          // If the input has changed since this request was fired off, don't update
          if (get(inputStore) !== input) return store
          return { status: ValidationStatus.Success, input: input }
        })
      )
    }
  }

  const updateInput = (input: string): void => {
    if (!inputStore) {
      inputStore = writable(input)
      inputStore.subscribe((input: string) => { const valid = validate(input) })
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



