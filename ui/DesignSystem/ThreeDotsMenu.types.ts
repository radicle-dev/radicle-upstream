// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { SvelteComponent } from "svelte";

export interface MenuItem {
  title: string;
  icon: typeof SvelteComponent;
  event: () => void;
  tooltip?: string;
  dataCy?: string;
  disabled?: boolean;
}
