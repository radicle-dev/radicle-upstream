<script>
  import { Button, Header, Icon, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import NarrowSidebarLayout from "../layouts/NarrowSidebarLayout.svelte";
</script>

<NarrowSidebarLayout dataCy="page">
  <Header style="margin-bottom: 24px">
    <div slot="left">
      <Title.Big>Projects</Title.Big>
    </div>

    <div slot="right">
      <Button icon={Icon.Plus} variant="outline">New Project</Button>
    </div>
  </Header>

  <Projects />
</NarrowSidebarLayout>
