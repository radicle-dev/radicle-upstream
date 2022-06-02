// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as error from "./error";
import * as ipc from "./ipc";
import * as modal from "./modal";
import * as notification from "./notification";
import * as router from "./router";
import * as session from "./session";

import SearchModal from "ui/App/SearchModal.svelte";

export function register(): void {
  ipc.customProtocolInvocation.onValue(async message => {
    await session.waitUnsealed();
    handleMessage(message);
  });
}

function showError(url: string) {
  notification.showException(
    new error.Error({
      code: error.Code.CustomProtocolParseError,
      message: "Could not parse the provided URL",
      details: { url },
    })
  );
}

export const LINK_URI_PREFIX = "radicle://link/v0/";

function handleMessage(message: ipc.CustomProtocolInvocation): void {
  const route = router.uriToRoute(message.url);
  if (route) {
    router.push(route);
    return;
  }

  if (message.url.startsWith(LINK_URI_PREFIX)) {
    const path = message.url.substring(LINK_URI_PREFIX.length);
    const [urn, ...rest] = path.split("/");
    const match = urn.match(/^rad:git:[1-9A-HJ-NP-Za-km-z]{37}$/);
    if (urn && match && rest.length === 0) {
      const urn = match[0];
      modal.show(SearchModal, () => {}, { searchQuery: urn });
      return;
    }
  }

  showError(message.url);
}
