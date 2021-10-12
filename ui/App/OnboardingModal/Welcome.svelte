<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, RadicleLogo } from "ui/DesignSystem";

  const dispatch = createEventDispatcher();

  let ran = false;

  const onKeydown = (event: KeyboardEvent) => {
    if (!ran && event.key === "Enter") {
      next();
      ran = true;
    }
  };

  const next = () => {
    dispatch("next");
  };
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }

  p {
    text-align: center;
    color: var(--color-foreground-level-6);
    margin: 2.5rem 0;
    max-width: 20rem;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container" data-cy="welcome-screen">
  <RadicleLogo />

  <p>A peer-to-peer stack for building <br /> software together.</p>

  <Button
    style="flex-shrink: 0; margin-bottom: 1.5rem;"
    on:click={next}
    dataCy="get-started-button">
    Get started
  </Button>
</div>
