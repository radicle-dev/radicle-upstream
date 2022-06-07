// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { SvelteComponent } from "svelte";

export interface Tab {
  title: string;
  active: boolean;
  icon: typeof SvelteComponent;
  counter?: number;
  onClick: () => void;
}
