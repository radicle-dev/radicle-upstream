<script lang="typescript">
  import { Icon } from "../Primitive";
  import BranchBox from "./BranchBox.svelte";

  import type { Branch } from "../../src/source";

  export let baseBranch: Branch;
  export let compareBranch: Branch;
  export let bg: string = "";
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
  <BranchBox style={bg} name={baseBranch} />
  <Icon.ArrowLeft />
  <BranchBox style={bg} name={compareBranch} />
</div>
