import * as error from "ui/src/error";

export const unreachable = (value: never): never => {
  throw new error.Error({
    code: error.Code.Unreachable,
    message: "Unreachable code",
    details: { value },
  });
};
