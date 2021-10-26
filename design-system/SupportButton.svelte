<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let style: string | undefined = undefined;
  export let dataCy: string | undefined = undefined;
</script>

<style>
  .support {
    white-space: nowrap;
    background: var(--color-background);
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    align-items: center;
    height: 40px;
    outline-style: none;
    padding: 0 12px;
    user-select: none;
    position: relative;
    border-radius: 0.5rem;
    border: 1px solid transparent;
    background-clip: padding-box;
    transform-style: preserve-3d;
  }

  .support:after {
    content: "";
    position: absolute;
    top: 0px;
    right: 0px;
    bottom: 0px;
    left: 0px;
    transform: translateZ(-1px);
    margin: -1px;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      var(--color-primary),
      var(--color-positive),
      var(--color-caution),
      var(--color-negative),
      var(--color-primary)
    );
    background-size: 200% 200%;
  }

  .support:hover {
    border: solid 1px transparent;
    color: var(--color-foreground);
  }

  .support:hover:after {
    content: "";
    position: absolute;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      var(--color-primary),
      var(--color-positive),
      var(--color-caution),
      var(--color-negative),
      var(--color-primary)
    );
    background-size: 200% auto;
    animation: shine 2s linear infinite;
  }

  /* motion */
  @keyframes shine {
    to {
      background-position: 200% center;
    }
  }
  .support:active {
    color: var(--color-foreground-level-5);
  }
</style>

<button data-cy={dataCy} class="support" on:click {style}>
  <p class="typo-text-bold">Support</p>
</button>
