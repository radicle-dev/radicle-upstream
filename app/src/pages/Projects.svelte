<script>
  import { Button, Header, Title } from "../DesignSystem";
  import Projects from "../components/Projects.svelte";
  import Layout from "../components/Layout.svelte";
</script>

<Layout>
  <Header>
    <div slot="left">
      <Title.Big>Projects</Title.Big>
    </div>
    <div slot="right">
      <Button.Outline>New Project</Button.Outline>
    </div>
  </Header>

  <Projects />
</Layout>
