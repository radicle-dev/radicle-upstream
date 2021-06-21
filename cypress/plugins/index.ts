// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { nodeManagerPlugin } from "./nodeManager/plugin";

export default (
  on: Cypress.PluginEvents,
  _config: Cypress.PluginConfigOptions
): void => {
  on("task", { ...nodeManagerPlugin });
};
