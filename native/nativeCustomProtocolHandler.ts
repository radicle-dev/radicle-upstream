// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import lodash from "lodash";
import * as zod from "zod";

export const throttled: (callback: () => void) => void = lodash.throttle(
  callback => {
    callback();
  },
  1000, // 1 second
  { trailing: false }
);

export const radicleUrlSchema = zod
  .string()
  .min(1)
  .max(1024)
  .refine(
    value => {
      let url;
      try {
        url = new URL(value);
      } catch {
        return false;
      }
      return url.protocol === "radicle:";
    },
    { message: "Invalid URL or protocol" }
  );
