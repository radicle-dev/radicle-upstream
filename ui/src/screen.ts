export const lock = (): void => {
  document.documentElement.classList.add("lock-screen");
};

export const unlock = (): void => {
  document.documentElement.classList.remove("lock-screen");
};

export const isLocked = (): boolean => {
  return document.documentElement.classList.contains("lock-screen");
};

export function withLock<T>(f: () => Promise<T>): Promise<T> {
  lock();
  return f().finally(() => unlock());
}
