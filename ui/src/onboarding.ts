// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

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
    message: "You must provide a display name",
    allowEmpty: false,
  },
  firstHandleChar: {
    valueName: "display name",
  },
  length: {
    minimum: 2,
    maximum: 32,
    tooShort: "Your display name should be at least 2 characters long.",
    tooLong: "Your display name should not be longer than 32 characters.",
  },
  format: {
    pattern: new RegExp(HANDLE_MATCH, "i"),
    message:
      "Your display name has unsupported characters in it. You can only use basic letters, numbers, and the _ and - characters.",
  },
};

export const createHandleValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(handleConstraints);

export const formatHandleInput = (input: string): string =>
  input.replace(" ", "-");
