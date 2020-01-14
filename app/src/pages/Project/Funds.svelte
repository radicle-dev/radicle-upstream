<script>
  import { Header, Title, Button } from "../../DesignSystem";
  import { registerProject } from "../../path.js";
  import { push } from "svelte-spa-router";
  import { getContext } from "svelte";

  const projectId = getContext("projectId");
</script>

<Title.Big style="color: var(--color-darkgray)">
  Register your project to receive donations
</Title.Big>

<Button
  variant="primary"
  on:click={() => {
    push(registerProject(projectId));
  }}>
  Register this project
</Button>
