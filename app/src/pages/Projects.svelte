<script>
  import { push } from "svelte-spa-router";
  import { Button, Flex, Icon, Text } from "../DesignSystem";
  import ProjectList from "../components/ProjectList.svelte";
  import SidebarLayout from "../layouts/SidebarLayout.svelte";
  import { createProject } from "../path.js";
</script>

<SidebarLayout dataCy="page">
  <Flex style="margin-bottom: 24px">
    <div slot="left">
      <Text variant="bigTitle">My Projects</Text>
    </div>

    <div slot="right">
      <Button
        dataCy="new-project-button"
        icon={Icon.Plus}
        variant="outline"
        size="small"
        on:click={() => {
          push(createProject());
        }}>
        New Project
      </Button>
    </div>
  </Flex>

  <ProjectList />
</SidebarLayout>
