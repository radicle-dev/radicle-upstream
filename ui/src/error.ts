export enum Variant {
  EntityExists = "ENTITY_EXISTS",
  GitError = "GIT_ERROR",
  NotFound = "NOT_FOUND",
}

export interface Error {
  message: string;
  variant: Variant;
}
