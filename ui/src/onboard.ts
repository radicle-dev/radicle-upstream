import { writable } from "svelte/store";

export enum State {
  Welcome = "WELCOME",
  EnterName = "ENTER_NAME",
  EnterPassphrase = "ENTER_PASSPHRASE",
  SuccessView = "SUCCESS_VIEW",
  Complete = "COMPLETE",
}

export const store = writable(State.Welcome);
