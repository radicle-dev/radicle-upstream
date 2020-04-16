<script>
  import { ModalLayout } from "../DesignSystem/Component";

  export let params = null;
  const projectId = params.projectId;
  const entityId = params.entityId;
</script>

<ModalLayout>Register projectId: {projectId} entityId: {entityId}</ModalLayout>
