// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { TextInputValidationState } from "design-system/TextInput";

import validatejs from "validate.js";
import { writable, Writable, Readable, get } from "svelte/store";

export enum ValidationStatus {
  NotStarted = "NOT_STARTED",
  Loading = "LOADING",
  Error = "ERROR",
  Success = "SUCCESS",
}

export interface ValidationStore extends Readable<TextInputValidationState> {
  reset: () => void;
  validate: (input: string) => void;
}

interface RemoteValidation {
  promise: (input: string) => Promise<boolean>;
  validationMessage: string;
}

interface FirstHandleCharOptions {
  valueName: string;
}

interface FormatConstraints {
  presence?: {
    message: string;
    allowEmpty?: boolean;
  };
  format?: {
    pattern: RegExp;
    message: string;
  };
  firstHandleChar?: FirstHandleCharOptions;
  length?: {
    minimum?: number;
    maximum?: number;
    tooShort?: string;
    tooLong?: string;
  };
}

validatejs.validators.firstHandleChar = (
  value: string,
  options: FirstHandleCharOptions,
  _key: unknown,
  _attributes: unknown
) => {
  if (!value.match(new RegExp("^[a-z0-9]", "i"))) {
    return `Your ${options.valueName} should start with a letter or a number.`;
  }
};

export const createValidationStore = (
  constraints: FormatConstraints,
  remoteValidations?: RemoteValidation[]
): ValidationStore => {
  const initialState = {
    type: "unvalidated",
  } as TextInputValidationState;
  const internalStore = writable(initialState);
  // eslint-disable-next-line @typescript-eslint/unbound-method
  const { subscribe, update } = internalStore;
  let inputStore: Writable<string> | undefined = undefined;

  const reset = (): void => {
    inputStore = undefined;
    internalStore.set(initialState);
  };

  const getInput = (): string | undefined => {
    if (inputStore) {
      return get(inputStore);
    } else {
      return undefined;
    }
  };

  const runValidations = async (input: string): Promise<void> => {
    // Always start with Loading
    update(() => {
      return { type: "pending", input: input };
    });

    // Check for errors
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    const errors = validatejs(
      { input: input },
      { input: constraints },
      { fullMessages: false }
    );

    if (errors) {
      update(() => {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
        return { type: "invalid", message: errors.input[0] };
      });
      return;
    }

    let remoteSuccess = true;

    if (remoteValidations) {
      for (const remoteValidation of remoteValidations) {
        try {
          const valid = await remoteValidation.promise(input);

          if (!valid) {
            remoteSuccess = false;

            update(store => {
              // If the input has changed since this request was fired off, don't update
              if (getInput() !== input) {
                return store;
              }
              return {
                type: "invalid",
                message: remoteValidation.validationMessage,
              };
            });

            break;
          }
        } catch (error: unknown) {
          remoteSuccess = false;

          update(store => {
            // If the input has changed since this request was fired off, don't update
            if (getInput() !== input) {
              return store;
            }
            return {
              type: "invalid",
              message: `Cannot validate "${input}": ${
                (error as Error).message
              }`,
            };
          });

          break;
        }
      }

      if (!remoteSuccess) {
        return;
      }
    }

    // If we made it here, it's valid
    update(store => {
      // If the input has changed since this request was fired off, don't update
      if (getInput() !== input) {
        return store;
      }
      return { type: "valid" };
    });
  };

  const validate = (input: string): void => {
    if (!inputStore) {
      inputStore = writable(input);
      inputStore.subscribe((input: string) => {
        runValidations(input);
      });
      return;
    }

    inputStore.set(input);
  };

  return {
    reset,
    subscribe,
    validate,
  };
};
