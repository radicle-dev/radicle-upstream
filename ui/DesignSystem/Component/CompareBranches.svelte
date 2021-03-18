<script lang="typescript">
  import { Icon } from "../Primitive";
  import BranchBox from "./BranchBox.svelte";

  import type { Branch } from "../../src/source";

  export let baseBranch: Branch;
  export let compareBranch: Branch;
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
