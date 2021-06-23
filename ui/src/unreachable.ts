// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as error from "ui/src/error";

export const unreachable = (value: never): never => {
  throw new error.Error({
    code: error.Code.Unreachable,
    message: "Unreachable code",
    details: { value },
  });
};
