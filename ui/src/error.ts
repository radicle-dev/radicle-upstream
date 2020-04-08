export enum Variant {
  NotFound = "NOT_FOUND";
}

export interface Error {
  message: string;
  variant: Variant;
}
