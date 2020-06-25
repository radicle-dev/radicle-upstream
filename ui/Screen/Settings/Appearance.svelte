<script>
  import { createEventDispatcher } from "svelte";

  import Text from "../../DesignSystem/Primitive/Text.svelte";
  import Title from "../../DesignSystem/Primitive/Title.svelte";

  import SegmentedControl from "../../DesignSystem/Component/SegmentedControl.svelte";

  export let appearance = null;
  export let options = null;

  const dispatch = createEventDispatcher();
  const update = event => {
    console.log(event);
    dispatch("update", event.detail);
  };
</script>

<section>
  <header>
    <Title variant="large">Appearance</Title>
  </header>
  <div class="item">
    <div class="info">
      <Text variant="medium">Theme</Text>
    </div>
    <div class="action">
      <SegmentedControl
        active={appearance.theme}
        {options}
        on:select={update} />
    </div>
  </div>
</section>
