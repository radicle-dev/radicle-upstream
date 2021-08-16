// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import lodash, { Dictionary } from "lodash";
import { nodeManagerPlugin } from "./nodeManager/plugin";
import { ethereumDevNodePlugin } from "./ethereumDevNode";

export default (
  on: Cypress.PluginEvents,
  _config: Cypress.PluginConfigOptions
): void => {
  on("task", {
    ...addNamespace("nodeManager", nodeManagerPlugin),
    ...addNamespace("ethereumDevNode", ethereumDevNodePlugin),
  });
};

// eslint-disable-next-line @typescript-eslint/ban-types
function addNamespace<T extends object>(
  namespace: string,
  api: T
): Dictionary<T[keyof T]> {
  return lodash.mapKeys(api, (_value, key) => `${namespace}:${key}`);
}
