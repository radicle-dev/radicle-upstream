export const hash = string =>
  string.split("").reduce((a, b) => ((a << 5) - a + b.charCodeAt(0)) | 0, 0);
