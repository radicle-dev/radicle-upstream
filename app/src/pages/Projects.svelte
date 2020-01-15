<script>
  import { push } from "svelte-spa-router";
  import { Button, Header, Icon, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import SidebarLayout from "../layouts/SidebarLayout.svelte";
  import { createProject } from "../path.js";
</script>

<SidebarLayout dataCy="page">
  <Header style="margin-bottom: 24px">
    <div slot="left">
      <Title.Big>My Projects</Title.Big>
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
  </Header>

  <Projects />
</SidebarLayout>
