<script>
  import { Avatar, Icon } from "../../../Primitive";

  export let ahead = 3;
  export let behind = 5;
  export let branchName = "main";
</script>

<style>
  .container {
    display: flex;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
    align-items: center;
    padding: 0 0.5rem;
    white-space: nowrap;
    gap: 0.5rem;
    color: var(--color-foreground-level-6);
  }
</style>

<div class="container">
  <p class="typo-text-bold">{ahead} ahead, {behind} behind</p>
  <p class="typo-text">of</p>
  <Avatar size="small" />
  <p class="typo-text-bold">{branchName}</p>
  <Icon.ChevronUpDown />
</div>
