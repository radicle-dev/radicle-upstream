<script>
  import { Button, Header, Icon, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import Layout from "../components/Layout.svelte";
</script>

<Layout>
  <Header>
    <div slot="left">
      <Title.Big>Projects</Title.Big>
    </div>
    <div slot="right">
      <Button icon={Icon.Plus} variant="outline">New Project</Button>
    </div>
  </Header>

  <Projects />
</Layout>
