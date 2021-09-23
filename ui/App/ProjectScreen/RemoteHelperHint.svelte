<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript" context="module">
  import * as zod from "zod";

  import * as browserStore from "ui/src/browserStore";

  const remoteHelperHintShown = browserStore.create<boolean>(
    "radicle.remoteHelperHintShown",
    false,
    zod.boolean()
  );
</script>

<script lang="typescript">
  import { Copyable, Hoverable, Icon } from "ui/DesignSystem";

  let hover = false;
</script>

<style>
  .info {
    margin-top: 1rem;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 0.5rem;
    align-items: left;
    text-align: left;
  }

  .description {
    margin-bottom: 0.75rem;
    color: var(--color-foreground-level-6);
  }

  .close-hint-button {
    float: right;
    cursor: pointer;
  }
</style>

{#if !$remoteHelperHintShown}
  <div class="info" data-cy="remote-helper-hint">
    <div
      data-cy="close-hint-button"
      class="close-hint-button"
      on:click={() => {
        remoteHelperHintShown.set(true);
      }}>
      <Icon.CrossSmall />
    </div>
    <p class="description">
      To publish code to Radicle, you need to add this to your shell
      configuration file. Not sure how?
      <a
        class="typo-link"
        href="https://docs.radicle.xyz/docs/getting-started#configuring-your-system">
        Read more
      </a>
    </p>
    <Hoverable bind:hovering={hover}>
      <Copyable name="shell configuration" tooltipStyle="width: fit-content;">
        <p
          class="typo-text-small-mono"
          style="color: var(--color-foreground-level-6)">
          export PATH="$HOME/.radicle/bin:$PATH"
        </p>
      </Copyable>
    </Hoverable>
  </div>
{/if}
