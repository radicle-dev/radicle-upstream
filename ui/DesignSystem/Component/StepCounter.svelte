<script>
  import { Icon, Title } from "../Primitive";

  export let step = null;
  export let of = null;
  export let style = null;
</script>

<style>
  .step-counter {
    display: inline-flex;
    align-items: flex-end;
  }

  .step {
    display: flex;
    flex-direction: column;
    width: 15px;
    color: var(--color-gray);
  }

  .step.active {
    color: var(--color-blue);
  }
</style>

<div class="step-counter" {style}>
  {#each of as stepName, index}
    {#if index > 0}
      <svg height="15" width="126">
        <line
          x1="0"
          y1="7.5"
          x2="126"
          y2="7.5"
          style="stroke: var(--color-gray); stroke-width: 1" />
      </svg>
    {/if}
    <div class="step" class:active={step === index + 1}>
      <Title
        style="color: inherit; margin-bottom: 14px; align-self: center;
        white-space: nowrap;">
        {stepName}
      </Title>
      <Icon.StepCounter active={step === index + 1} />
    </div>
  {/each}
</div>
