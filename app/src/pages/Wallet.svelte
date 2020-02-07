<script>
  import { Title } from "../DesignSystem";
  import Layout from "../layouts/SidebarLayout.svelte";
  import RowLayout from "../layouts/helpers/Row.svelte";
</script>

<Layout dataCy="page">
  <RowLayout>
    <div slot="left">
      <Title size="big">Wallet</Title>
    </div>
  </RowLayout>
</Layout>
