<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let data: unknown;
  export let title: string | undefined = undefined;
</script>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1;
    overflow: scroll;
  }

  .json {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 1rem;
    overflow: scroll;
  }
</style>

<div class="container">
  {#if title}
    <h5>{title}</h5>
  {/if}

  <pre class="json">{JSON.stringify(data, null, 2)}</pre>
</div>
