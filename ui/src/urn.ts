// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type Urn = string;

export const shorten = (stringWithUrn: string): string => {
  return stringWithUrn.replace(/(rad:git:[\w]{3})[\w]{53}([\w]{3})/, "$1…$2");
};
