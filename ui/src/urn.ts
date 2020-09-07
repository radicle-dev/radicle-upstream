export const shorten = (stringWithUrn: string): string => {
  return stringWithUrn.replace(/(rad:git:[\w]{3})[\w]{53}([\w]{3})/, "$1…$2");
};
