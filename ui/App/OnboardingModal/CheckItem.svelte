<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Button from "design-system/Button.svelte";
  import CheckCircle from "design-system/icons/CheckCircle.svelte";
  import ChevronDown from "design-system/icons/ChevronDown.svelte";
  import Circle from "design-system/icons/Circle.svelte";
  import Ellipsis from "design-system/icons/Ellipsis.svelte";
  import { slide } from "svelte/transition";

  export let title: string;
  export let expanded: boolean;
  export let onClick: (() => unknown) | null = null;
  export let onSkip: (() => unknown) | null = null;
  export let done = false;
  export let waitingFor: string | null = null;
  export let badge: string | null = null;
</script>

<style>
  .check-item {
    padding: 24px;
    border-radius: 16px;
    background-color: var(--color-foreground-level-1);
    margin-bottom: 24px;
    transition: background-color 0.3s ease;
  }

  .done {
    background-color: var(--color-positive-level-1);
  }

  .done * {
    color: var(--color-positive-level-6);
    transition: color 0.3s ease;
  }

  .opener {
    display: flex;
    justify-content: space-between;
  }

  .icon-and-title {
    display: flex;
    align-items: center;
    gap: 16px;
    height: 40px;
  }

  .icon {
    height: 24px;
    width: 24px;
    transform-origin: center;
    transition: transform 0.3s ease;
  }

  .badge-and-chevron {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .badge {
    padding: 8px;
    border-radius: 8px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .badge.positive {
    background-color: var(--color-positive-level-6);
  }

  .badge.negative {
    background-color: var(--color-negative-level-2);
  }

  .badge.positive > h4 {
    color: var(--color-positive-level-1);
  }

  .badge.negative > h4 {
    color: var(--color-negative-level-6);
  }

  .content {
    margin-top: 32px;
    display: flex;
    flex-direction: column;
  }

  .card-end {
    display: flex;
    margin-top: 32px;
    width: 100%;
    align-items: center;
    justify-content: space-between;
  }

  .waiting-for {
    color: var(--color-foreground-level-5);
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .skip-button:only-child {
    margin-left: auto;
  }
</style>

<div class="check-item" class:done on:click={onClick}>
  <div class="opener">
    <div class="icon-and-title">
      {#if done}
        <CheckCircle
          style={`fill: var(--${
            done ? "color-positive-level-6" : "color-foreground-level-4"
          }); transition: fill .3s ease`} />
      {:else}
        <Circle
          style={`fill: var(--${
            done ? "color-positive-level-6" : "color-foreground-level-4"
          }); transition: fill .3s ease`} />
      {/if}
      <h3>{title}</h3>
    </div>
    <div class="badge-and-chevron">
      {#if badge}
        <div class="badge" class:positive={done} class:negative={!done}>
          <h4>{badge}</h4>
        </div>
      {/if}
      {#if !done}
        <div class="icon" style:transform={`rotate(${expanded ? -180 : 0}deg)`}>
          <ChevronDown />
        </div>
      {/if}
    </div>
  </div>
  {#if expanded}
    <div class="content" transition:slide>
      <slot name="content" />
      <div class="card-end">
        {#if waitingFor}
          <div class="waiting-for">
            <Ellipsis animate />
            <p class="typo-text-small-bold">Waiting for {waitingFor}</p>
          </div>
        {/if}
        {#if onSkip}
          <div class="skip-button">
            <Button on:click={onSkip} variant="outline">Mark as done</Button>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
