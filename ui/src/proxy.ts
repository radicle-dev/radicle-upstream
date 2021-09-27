// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { config } from "ui/src/config";

export * from "proxy-client/index";
import { ProxyClient } from "proxy-client";

export const client = new ProxyClient(`http://${config.proxyAddress}`);
