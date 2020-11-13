<script lang="typescript">
  import { Button, Input } from "../../../../Primitive";
  import { Dai, Illustration } from "../../../../Component";

  import { Variant as IllustrationVariant } from "../../../../../src/illustration";

  export let onCancel: () => void;
  export let onContinue: (budget: number) => void;

  let budget = 0;
</script>

<style>
  h1,
  p {
    padding: 0 var(--content-padding);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<Illustration variant={IllustrationVariant.Money} />
<h1>Set a budget</h1>
<p>
  Set your monthly budget for outgoing support. This amount will flow to your
  receivers in real time.
</p>
<Input.Text
  dataCy="modal-amount-input"
  bind:value={budget}
  showLeftItem
  autofocus
  style={'width: 125px'}>
  <div slot="left" style="position: absolute; top: 1px; left: 12px;">
    <Dai />
  </div>
</Input.Text>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onCancel}
    style="margin-right: 1rem">
    Cancel
  </Button>

  <Button dataCy="confirm-button" on:click={() => onContinue(budget)}>
    Continue
  </Button>
</div>
