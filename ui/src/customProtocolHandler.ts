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
  ipc.listenCustomProtocolInvocation(async message => {
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

function handleMessage(message: ipc.CustomProtocolInvocation): void {
  const match = message.url.match(/^radicle:\/\/(\w+)\//);

  if (!match) {
    showError(message.url);
    return;
  }

  const namespace = match.slice(1)[0];

  if (namespace === "link") {
    const match = message.url.match(
      /^radicle:\/\/link\/v0\/(rad:git:[1-9A-HJ-NP-Za-km-z]{37})/
    );
    if (match) {
      const urn = match.slice(1)[0];
      if (urn) {
        modal.show(SearchModal, () => {}, { searchQuery: urn });
        return;
      }
    }
  }

  if (namespace === "upstream") {
    const match = message.url.match(/^radicle:\/\/upstream\/v0\/(.*)/);

    if (match) {
      const path = match.slice(1)[0];
      if (path) {
        const route = router.pathToRoute(path);
        if (route) {
          router.push(route);
          return;
        }
      }
    }
  }

  showError(message.url);
}
