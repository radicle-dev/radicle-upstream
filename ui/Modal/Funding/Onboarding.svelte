<script lang="typescript">
  import Intro from "../../DesignSystem/Component/Funding/Pool/Onboarding/Intro.svelte";
  import SetBudget from "../../DesignSystem/Component/Funding/Pool/Onboarding/SetBudget.svelte";
  import TopUpz from "../../DesignSystem/Component/Funding/Pool/Onboarding/TopUpz.svelte";
  import AddReceivers from "../../DesignSystem/Component/Funding/Pool/Onboarding/AddReceivers.svelte";

  import type { Changeset } from "../../src/funding/pool";

  import * as modal from "../../src/modal";

  enum Step {
    Intro = "intro",
    SetBudget = "budget",
    AddReceivers = "receivers",
    TopUp = "topup",
  }

  let currentStep = Step.Intro;

  function onCancel() {
    modal.hide();
  }

  function onBudgetSet(amount: number) {
    budget = amount;
    currentStep = Step.AddReceivers;
  }

  function onAddReceivers(receivers: Changeset) {
    receivers = receivers;
    currentStep = Step.TopUp;
  }

  function onTopUp(amount: number) {
    balance = amount;
    currentStep = Step.Intro;
  }

  /* Themz values */
  let budget = 0;
  let balance = 0;
  let receivers = {};
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 600px;
    min-height: 400px;
    background: var(--color-background);
    border-radius: 0.5rem;

    text-align: center;
  }
</style>

<div class="wrapper">
  {#if currentStep === Step.Intro}
    <Intro
      onSkip={onCancel}
      onContinue={() => (currentStep = Step.SetBudget)} />
  {:else if currentStep === Step.SetBudget}
    <SetBudget {onCancel} onContinue={onBudgetSet} />
  {:else if currentStep === Step.AddReceivers}
    <AddReceivers
      onBack={() => (currentStep = Step.SetBudget)}
      onSave={onAddReceivers} />
  {:else if currentStep === Step.TopUp}
    <TopUpz
      onBack={() => (currentStep = Step.AddReceivers)}
      onContinue={amount => console.log('TODO')} />
  {/if}
</div>
