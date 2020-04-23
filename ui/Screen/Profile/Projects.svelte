<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { projects } from "../../src/project.ts";

  import { Flex, Text, Button } from "../../DesignSystem/Primitive";
  import { Placeholder, ProjectList, Remote } from "../../DesignSystem/Component";
</script>

<style>
  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
  }

  .create-project {
    text-align: center;
    width: 480px;
  }
</style>

<Remote store={projects}>
  <div slot="success" let:data>
    {#if data.lenght > 0}
      <ProjectList projects={data} />
    {:else}
      <div class="wrapper">
        <div class="create-project">
          <Placeholder style="width: 420px; height: 217px;" />
          <Flex style="margin-top: 27px;">
            <div slot="left" style="align-items: center; justify-content: center">
              <Text
                style="margin-bottom: 24px; text-align: left; color:
                var(--color-foreground-level-6);">
                Create a new project because that's why you're here.
              </Text>
              <Button
                variant="vanilla"
                on:click={() => {
                  push(path.createProject());
                }}>
                Start a new project
              </Button>
            </div>
            <div
              slot="right"
              style="margin-left: 24px; display: flex; flex-direction: column;
              align-items: center; justify-content: center">
              <Text
                style="margin-bottom: 24px; text-align: left; color:
                var(--color-foreground-level-6);">
                Register so your friends can find you!
              </Text>
              <Button
                variant="vanilla"
                on:click={() => {
                  push(path.registerUser());
                }}>
                Register handle
              </Button>
            </div>
          </Flex>
        </div>
      </div>
    {/if}
  </div>

  <div slot="error" let:error>
    <Text>{error}</Text>
  </div>
</Remote>
