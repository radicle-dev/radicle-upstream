<script>
  import { Header, Remote, SidebarLayout } from "../DesignSystem/Component";
  import { fetch, identity as store } from "../src/identity.ts";
  export let params = null;

  $: fetch({ urn: params.urn });
</script>

<SidebarLayout>
  <Remote {store} let:data={identity}>
    <Header.Large variant="profile" entity={identity} />
  </Remote>
</SidebarLayout>
