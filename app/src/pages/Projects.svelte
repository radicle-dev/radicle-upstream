<script>
  import { Button, Header, Icon, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import Layout from "./Layout.svelte";
</script>

<Layout>
  <Header style="margin-bottom: 24px">
    <div slot="left">
      <Title.Big>Projects</Title.Big>
    </div>

    <div slot="right">
      <Button icon={Icon.Plus} variant="outline">New Project</Button>
    </div>
  </Header>

  <Projects />
</Layout>
