// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as error from "./error";
import * as ipc from "./ipc";
import * as modal from "./modal";
import * as notification from "./notification";
import * as session from "./session";

import SearchModal from "ui/App/SearchModal.svelte";

export const register = (): void => {
  ipc.listenCustomProtocolInvocation(async message => {
    await session.waitUnsealed();
    handleMessage(message);
  });
};

const handleMessage = (message: ipc.CustomProtocolInvocation): void => {
  const match = message.url.match(
    /^radicle:\/\/(\w+)\/v([\d+])\/?(rad:git:[1-9A-HJ-NP-Za-km-z]{37})?/
  );

  if (!match) {
    notification.showException(
      new error.Error({
        code: error.Code.CustomProtocolParseError,
        message: "Could not parse the provided URL",
        details: { url: message.url },
      })
    );

    return;
  }

  const [namespace, version, urn] = match.slice(1);

  if (namespace !== "link") {
    notification.showException(
      new error.Error({
        code: error.Code.CustomProtocolUnsupportedNamespace,
        message: `The custom protocol namespace "${namespace}" is not supported`,
        details: { url: message.url },
      })
    );

    return;
  }

  if (Number(version) !== 0) {
    notification.showException(
      new error.Error({
        code: error.Code.CustomProtocolUnsupportedVersion,
        message: `The custom protocol version v${version} is not supported`,
        details: { url: message.url },
      })
    );

    return;
  }

  if (!urn) {
    notification.showException(
      new error.Error({
        code: error.Code.CustomProtocolParseError,
        message: "The provided URL does not contain a Radicle ID",
        details: { url: message.url },
      })
    );

    return;
  }

  modal.show(SearchModal, () => {}, { searchQuery: urn });
};
