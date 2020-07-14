import * as api from "./api";
import * as validation from "./validation";

// The possible availability statuses of an Id
enum Status {
  // Available
  Available = "available",
  // Currently taken by an Org or a User
  Taken = "taken",
  // The id was unregistered by an Org or a User and is no longer claimable
  Retired = "retired",
}

const getStatus = (id: string): Promise<Status> =>
  api.get<Status>(`ids/${id}/status`);

// Check if the given id is available
const isAvailable = (id: string): Promise<boolean> =>
  getStatus(id).then(status => status == Status.Available);

// ID validation
const VALID_ID_MATCH_STR = "^[a-z0-9][a-z0-9]+$";
const VALID_ID_MATCH = new RegExp(VALID_ID_MATCH_STR);
const idConstraints = {
  presence: {
    message: `This field is required`,
    allowEmpty: false,
  },
  format: {
    pattern: VALID_ID_MATCH,
    message: `It should match ${VALID_ID_MATCH_STR}`,
  },
};

// Id validation store.
export const idValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(idConstraints, [
    {
      promise: isAvailable,
      validationMessage: "Sorry, this one is already taken",
    },
  ]);
