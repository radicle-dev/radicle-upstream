// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import lodash from "lodash";

export const throttled: (callback: () => void) => void = lodash.throttle(
  callback => {
    callback();
  },
  1000, // 1 second
  { trailing: false }
);

export const parseRadicleUrl = (url: string): undefined | string => {
  if (
    typeof url !== "string" ||
    url.length === 0 ||
    Buffer.byteLength(url, "utf8") > 1024 ||
    !url.toLowerCase().match(/^radicle:\/\//)
  ) {
    return;
  }

  return url;
};
