// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

if (process.env.NODE_ENV === undefined) {
  process.env.NODE_ENV = "development";
}

/* eslint-disable @typescript-eslint/no-var-requires */
require("ts-node").register({ transpileOnly: true });
require("./index.ts");
