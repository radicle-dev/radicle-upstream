import * as org from "./org";
import * as user from "./user";
import * as validation from "./validation";

// Check if the given id is available
const isAvailable = (id: string): Promise<boolean> =>
  org.getOrg(id).then(org => !org && user.get(id).then(user => !user));

// ID validation
const VALID_ID_MATCH_STR = "^[a-z0-9][a-z0-9]+$";
const VALID_ID_MATCH = new RegExp(VALID_ID_MATCH_STR);
const idConstraints = {
  presence: {
    message: `Id is required`,
    allowEmpty: false,
  },
  format: {
    pattern: VALID_ID_MATCH,
    message: `Id should match ${VALID_ID_MATCH_STR}`,
  },
};

// Id availability validation store.
export const idValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(idConstraints, [
    {
      promise: isAvailable,
      validationMessage: "Sorry, this id is already taken",
    },
  ]);
