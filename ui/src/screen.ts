export const lock = (): void => {
  document.documentElement.classList.add("lock-screen");
};

export const unLock = (): void => {
  document.documentElement.classList.remove("lock-screen");
};
