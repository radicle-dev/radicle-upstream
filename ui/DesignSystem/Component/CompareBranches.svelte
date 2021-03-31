<script lang="typescript">
  import { Icon } from "../Primitive";
  import BranchBox from "./BranchBox.svelte";

  export let baseBranch: string;
  export let compareBranch: string;
</script>

<style>
  .branches {
    display: flex;
    align-self: center;
    align-items: center;
    gap: 0.5rem;
  }
</style>

<div class="branches">
  <BranchBox name={baseBranch} />
  <Icon.ArrowLeft />
  <BranchBox name={compareBranch} />
</div>
