// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";

import * as error from "ui/src/error";
import * as screen from "ui/src/screen";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as notification from "ui/src/notification";
import * as bacon from "ui/src/bacon";

import { Route, LoadedRoute, loadRoute, routeToPath } from "./definition";
export * from "./definition";

// This is only respected by Safari.
const DOCUMENT_TITLE = "Radicle Upstream";

const BOOT_ROUTE: Route & LoadedRoute = { type: "boot" };

const historyStore: svelteStore.Writable<Route[]> = svelteStore.writable([
  BOOT_ROUTE,
]);

const historyExecutor = mutexExecutor.create();

// Sets the history to the given value and navigates to the last item
// in the history.
const setHistory = async (history: Route[]) => {
  if (history.length === 0) {
    throw new error.Error({
      code: error.Code.EmptyHistory,
      message: "Cannot set empty history",
    });
  }
  const targetRoute = history.slice(-1)[0];

  const loadedRoute = await historyExecutor.run(async () => {
    let notificationHandle: notification.Handle | undefined;
    let notificationTimeout;

    const abort = new bacon.Bus<void>();

    function scheduleNotification() {
      notificationTimeout = setTimeout(() => {
        notificationHandle = notification.info({
          persist: true,
          bypassLockedScreen: true,
          message: "This seems to be taking a while",
          actions: [
            {
              label: "Keep waiting",
              handler: () => {
                scheduleNotification();
              },
            },
            {
              label: "Stop loading",
              handler: () => {
                abort.push();
              },
            },
          ],
        });
      }, 10_000);
    }

    scheduleNotification();

    try {
      return await screen.withLock(() => {
        return Promise.race([abort.firstToPromise(), loadRoute(targetRoute)]);
      });
    } finally {
      clearTimeout(notificationTimeout);
      if (notificationHandle) {
        notificationHandle.remove();
      }
    }
  });
  if (loadedRoute === undefined) {
    return;
  }

  activeRouteStore.set(loadedRoute);
  historyStore.set(history);
  window.history.replaceState(
    history,
    DOCUMENT_TITLE,
    // This sets `window.location.href`. At the moment it's not used by
    // Upstream, but when we switch to a browser environment it'll be
    // displayed in the URL bar.
    routeToPath(targetRoute)
  );
};

export const push = async (newRoute: Route): Promise<void> => {
  const history = svelteStore.get(historyStore);
  // Limit history to a maximum of 10 steps. We shouldn't be doing more than
  // one subsequent pop() anyway.
  await setHistory([...history, newRoute].slice(-10));
};

export const pop = async (): Promise<void> => {
  const history = svelteStore.get(historyStore);
  if (history.length > 1) {
    await setHistory(history.slice(0, -1));
  }
};

export const activeRouteStore = svelteStore.writable<LoadedRoute>(BOOT_ROUTE);

export const initialize = async (): Promise<void> => {
  let history: Route[];
  if (window.history.state === null) {
    history = [BOOT_ROUTE];
  } else {
    history = window.history.state;
  }
  await setHistory(history);
};
