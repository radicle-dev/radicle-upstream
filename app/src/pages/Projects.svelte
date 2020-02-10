<script>
  import { push } from "svelte-spa-router";
  import { Button, Icon, Title } from "../DesignSystem";
  import ProjectList from "../components/ProjectList.svelte";
  import SidebarLayout from "../layouts/SidebarLayout.svelte";
  import RowLayout from "../layouts/helpers/Row.svelte";
  import { createProject } from "../path.js";
</script>

<SidebarLayout dataCy="page">
  <RowLayout style="margin-bottom: 24px">
    <div slot="left">
      <Title size="big">My Projects</Title>
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
  </RowLayout>

  <ProjectList />
</SidebarLayout>
