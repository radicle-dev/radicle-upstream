<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { uiFont, uiFontOptions } from "ui/src/appearance";
  import SegmentedControl from "design-system/SegmentedControl.svelte";
</script>

<SegmentedControl
  active={$uiFont}
  options={uiFontOptions}
  on:select={ev => uiFont.set(ev.detail)} />
