// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

export function modifierKey(): "Meta" | "Control" {
  if (process.platform === "linux") {
    return "Control";
  } else {
    return "Meta";
  }
}
