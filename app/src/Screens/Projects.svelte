<script>
  import { push } from "svelte-spa-router";
  import { Button, Flex, Icon, Title } from "../DesignSystem/Primitives";
  import { ProjectList, SidebarLayout } from "../DesignSystem/Components";
  import { createProject } from "../lib/path.js";
</script>

<SidebarLayout dataCy="page">
  <Flex style="margin-bottom: 24px">
    <div slot="left">
      <Title variant="big">My Projects</Title>
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
