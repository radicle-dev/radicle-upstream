<script>
  import { Text, Button, Placeholder } from "../../DesignSystem";
  import { registerProject } from "../../path.js";
  import { push } from "svelte-spa-router";
  import { getContext } from "svelte";

  const projectId = getContext("projectId");
</script>

<style>
  .wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .layout {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin-top: 72px;
    width: 490px;
  }
</style>

<div class="wrapper">
  <div class="layout">
    <Text
      variant="bigTitle"
      style="color: var(--color-darkgray); margin-bottom: 24px">
      Register your project to receive donations
    </Text>

    <Placeholder style="width: 480px; height: 250px; margin-bottom: 24px" />
    <Text
      style="color: var(--color-darkgray); margin-bottom: 48px; text-align:
      center;">
      Cupidatat velit incididunt cillum cillum voluptate in occaecat ipsum qui
      sunt ad et aliqua consequat. Excepteur pariatur amet sunt duis excepteur
      ullamco exercitation.
    </Text>
    <Button
      variant="primary"
      on:click={() => {
        push(registerProject(projectId));
      }}>
      Register this project
    </Button>
  </div>
</div>
