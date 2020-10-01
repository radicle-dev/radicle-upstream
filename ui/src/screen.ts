export const loading = (): void => {
  document.documentElement.classList.add("loading");
};

export const loaded = (): void => {
  document.documentElement.classList.remove("loading");
};

export const lock = (): void => {
  document.documentElement.classList.add("lock-screen");
};

export const unlock = (): void => {
  document.documentElement.classList.remove("lock-screen");
};

export const isLocked = (): boolean => {
  return document.documentElement.classList.contains("lock-screen");
};
