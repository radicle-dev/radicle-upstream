let lockRequests = 0;

export const isLocked = (): boolean => {
  return lockRequests > 0;
};

// Shows a spinning cursor and ignore user clicks while `f` is running.
export function withLock<T>(f: () => Promise<T>): Promise<T> {
  if (lockRequests === 0) {
    document.documentElement.classList.add("lock-screen");
  }
  lockRequests += 1;
  return f().finally(() => {
    lockRequests -= 1;
    if (lockRequests === 0) {
      document.documentElement.classList.remove("lock-screen");
    }
  });
}
