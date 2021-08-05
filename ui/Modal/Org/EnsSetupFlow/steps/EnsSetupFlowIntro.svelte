<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

  export let onSubmit: () => void = () => {};
</script>

<div>
  <HeadlineAndDescription
    headline="Register your radicle.eth name"
    description="Your radicle.eth ENS name allows linking your organization with a name, logo, URL and social media profiles." />
  <ButtonRow {onSubmit} />
</div>
