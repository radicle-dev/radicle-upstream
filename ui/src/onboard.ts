import { writable } from "svelte/store"

export enum State {
  Welcome = "WELCOME",
  Form = "FORM",
  SuccessView = "SUCCESS_VIEW",
  Complete = "COMPLETE"
}

export const store = writable(State.Welcome)
