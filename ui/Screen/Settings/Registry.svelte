<script>
  import { createEventDispatcher } from "svelte";

  import Text from "../../DesignSystem/Primitive/Text.svelte";
  import Title from "../../DesignSystem/Primitive/Title.svelte";

  import SegmentedControl from "../../DesignSystem/Component/SegmentedControl.svelte";

  export let network = null;
  export let options = null;

  const dispatch = createEventDispatcher();
  const update = event => dispatch("update", event.detail);
</script>

<section>
  <header>
    <Title variant="large">Registry</Title>
  </header>
  <div class="item">
    <div class="info">
      <Text variant="medium">Network</Text>
    </div>
    <div class="action">
      <SegmentedControl active={network} {options} on:select={update} />
    </div>
  </div>
</section>
