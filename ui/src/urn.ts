import * as validation from "./validation";

// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type Urn = string;

// URN validation.
const VALID_URN_MATCH = /^rad:git:[1-9A-HJ-NP-Za-km-z]{37}/;
const urnConstraints = {
  format: {
    pattern: VALID_URN_MATCH,
    message: `Not a valid project URN`,
  },
};

export const urnValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(urnConstraints);
