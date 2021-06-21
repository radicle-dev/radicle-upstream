<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { setContext } from "svelte";

  export let data: unknown;
  export let name: string;

  setContext(name, data);
</script>

<slot />
