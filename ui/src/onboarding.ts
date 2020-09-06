import * as validation from "./validation";

export enum State {
  Welcome = "WELCOME",
  EnterName = "ENTER_NAME",
  EnterPassphrase = "ENTER_PASSPHRASE",
  SuccessView = "SUCCESS_VIEW",
}

const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";

const handleConstraints = {
  presence: {
    message: "You must provide a handle",
    allowEmpty: false,
  },
  format: {
    pattern: new RegExp(HANDLE_MATCH, "i"),
    message: `Handle should match ${HANDLE_MATCH}`,
  },
};

export const createHandleValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(handleConstraints);
