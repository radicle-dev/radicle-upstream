<script>
  import { SidebarLayout } from "../DesignSystem/Component";
  export let params = null;

  const id = params.id;
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  User profile: {id}
</SidebarLayout>
