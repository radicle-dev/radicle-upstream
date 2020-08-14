<script>
  import { Flex } from "../../DesignSystem/Primitive";

  import AdditionalActionsDropdown from "./AdditionalActionsDropdown.svelte";
  import ProjectCard from "./ProjectCard.svelte";
  import Stats from "./Stats.svelte";

  export let metadata = null;
  export let registration = null;
  export let shareableEntityIdentifier = null;
  export let stats = null;

  export let dataCy = null;
  export let menuItems = null;
</script>

<Flex style="flex: 1; padding: 1.5rem 1rem 1.5rem 1.5rem;" {dataCy}>
  <div slot="left">
    <ProjectCard
      title={metadata.name}
      description={metadata.description}
      showRegisteredBadge={registration} />
  </div>

  <div slot="right" style="display: flex; align-items: center;">
    <Stats {...stats} />
    <AdditionalActionsDropdown
      dataCy="context-menu"
      headerTitle={shareableEntityIdentifier}
      {menuItems} />
  </div>
</Flex>
