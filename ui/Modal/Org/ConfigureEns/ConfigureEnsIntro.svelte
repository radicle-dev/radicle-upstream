<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ensResolver from "ui/src/org/ensResolver";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let onSubmit: () => void;
</script>

<div>
  <Header
    title={`Register your ${ensResolver.DOMAIN} name`}
    description={`Your ${ensResolver.DOMAIN} ENS name allows linking your ` +
      `organization with a name, logo, URL and social media profiles.`} />
  <ButtonRow {onSubmit} />
</div>
