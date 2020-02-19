import validatelib from "validate.js";

export const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
export const validatejs = validatelib;

export const isEmpty = v => {
  return validatejs.isEmpty(v);
};

// Returns `false` if the item is valid or the validation error message
// if it is invalid.
export const invalid = (v, item) => {
  if (isEmpty(v)) {
    return false;
  }
  return !!v[item] && v[item][0];
};
