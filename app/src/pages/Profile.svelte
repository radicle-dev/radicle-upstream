<script>
  import { Text } from "../DesignSystem";
  import Layout from "../layouts/SidebarLayout.svelte";
  import RowLayout from "../layouts/helpers/Row.svelte";
</script>

<Layout dataCy="page">
  <RowLayout>
    <div slot="left">
      <Text variant="bigTitle">Profile</Text>
    </div>
  </RowLayout>
</Layout>
