<script>
  import NavigationButtons from "./NavigationButtons.svelte";
  import Transaction from "../Transaction.svelte";

  export let transaction = null;
  export let onCancel = null;
  export let onNextStep = null;
</script>

<div style="width: 100%;">
  <Transaction {transaction} style="margin-bottom: 32px;" />
  <NavigationButtons
    style="margin-top: 32px;"
    nextStepTitle="Submit transaction"
    {onCancel}
    {onNextStep} />
</div>
