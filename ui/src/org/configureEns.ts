// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ensResolver from "ui/src/org/ensResolver";
import * as notification from "ui/src/notification";
import * as router from "ui/src/router";
import * as svelteStore from "ui/src/svelteStore";

// If the user is on the screen of `orgAddress`, populate the screen with the
// updated registration metadata and show a notification.
//
// If the user has navigated away to another screen, show a notification about
// the updated registration metadata and a button to navigate back to the org
// screen of `orgAddress`.
export async function updateScreenAndNotifyUser(
  orgAddress: string,
  message: string
): Promise<void> {
  const updatedRegistration = await ensResolver.getCachedRegistrationByAddress(
    orgAddress,
    true
  );
  const activeRoute = svelteStore.get(router.activeRouteStore);

  let actions;

  if (
    (activeRoute.type === "singleSigOrg" ||
      activeRoute.type === "multiSigOrg") &&
    activeRoute.address === orgAddress
  ) {
    router.activeRouteStore.set({
      ...activeRoute,
      registration: updatedRegistration,
    });
  } else {
    actions = [
      {
        label: "Go to org",
        handler: () => {
          router.push({
            type: "org",
            params: {
              address: orgAddress,
              view: "projects",
            },
          });
        },
      },
    ];
  }

  notification.info({
    message,
    actions,
    showIcon: true,
  });
}
