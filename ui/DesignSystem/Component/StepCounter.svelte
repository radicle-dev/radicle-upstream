<script>
  import { Icon, Line, Title } from "../Primitive";

  export let selectedStep = null;
  export let steps = null;
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
    color: var(--color-foreground-level-5);
  }

  .step.active {
    color: var(--color-secondary);
  }
</style>

<div class="step-counter" {style}>
  {#each steps as stepName, index}
    {#if index > 0}
      <Line height="15" width="126" />
    {/if}
    <div class="step" class:active={selectedStep === index + 1}>
      <Title
        style="color: inherit; margin-bottom: 14px; align-self: center;
        white-space: nowrap;">
        {stepName}
      </Title>
      <Icon.StepCounter active={selectedStep === index + 1} />
    </div>
  {/each}
</div>
