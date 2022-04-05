<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { config } from "ui/src/config";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import Emoji from "design-system/Emoji.svelte";
</script>

<style>
  section {
    margin-bottom: 1.5rem;
    padding: 0 0.75rem;
  }
  .title {
    margin: 2rem 0;
    padding: 0 0.75rem;
  }

  .seeds {
    display: flex;
    gap: 1.5rem;
  }

  .seed {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 3rem;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 1rem;
  }

  .seed:hover {
    background-color: var(--color-foreground-level-1);
  }
</style>

<ScreenLayout dataCy="network-page">
  <h1 class="title">Network</h1>
  <section>
    <p class="typo-text">Seeds help you find more projects on the network.</p>
    <div class="seeds">
      {#if config.isDev}
        <a
          href="https://app.radicle.network/seeds/seed.upstream.radicle.xyz"
          class="seed">
          <Emoji emoji="🌱" size="huge" />
          <p class="typo-text-bold">seed.upstream.radicle.xyz</p>
        </a>
      {:else}
        <a
          href="https://app.radicle.network/seeds/willow.radicle.garden"
          class="seed">
          <Emoji emoji="🪵" size="huge" />
          <p class="typo-text-bold">willow.radicle.garden</p>
        </a>
        <a
          href="https://app.radicle.network/seeds/pine.radicle.garden"
          class="seed">
          <Emoji emoji="🌲" size="huge" />
          <p class="typo-text-bold">pine.radicle.garden</p>
        </a>
        <a
          href="https://app.radicle.network/seeds/maple.radicle.garden"
          class="seed">
          <Emoji emoji="🍁" size="huge" />
          <p class="typo-text-bold">maple.radicle.garden</p>
        </a>
      {/if}
    </div>
  </section>
</ScreenLayout>
