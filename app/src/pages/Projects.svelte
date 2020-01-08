<script>
  import { push } from "svelte-spa-router";
  import { Button, Header, Icon, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import NarrowSidebarLayout from "../layouts/NarrowSidebarLayout.svelte";
  import { createProject } from "../path.js";
</script>

<NarrowSidebarLayout dataCy="page">
  <Header style="margin-bottom: 24px">
    <div slot="left">
      <Title.Big>My Projects</Title.Big>
    </div>

    <div slot="right">
      <Button
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
</NarrowSidebarLayout>
