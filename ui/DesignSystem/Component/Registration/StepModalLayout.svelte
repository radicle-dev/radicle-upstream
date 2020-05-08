<script>
  import { Title } from "../../Primitive";
  import ModalLayout from "../ModalLayout.svelte";
  import StepCounter from "../StepCounter.svelte";

  export let dataCy = null;
  export let selectedStep = null;
  export let steps = null;
</script>

<style>
  .container {
    margin: 86px 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<ModalLayout {dataCy}>
  <div class="container">
    <StepCounter {selectedStep} {steps} style="margin-bottom: 50px;" />
    <Title variant="big" style="margin-bottom: 16px;">
      <slot name="title" />
    </Title>
    <slot />
  </div>
</ModalLayout>
