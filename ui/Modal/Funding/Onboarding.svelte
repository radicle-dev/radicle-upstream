<script lang="typescript">
  import Erc20Allowance from "./Onboarding/Erc20Allowance.svelte";
  import Intro from "./Onboarding/Intro.svelte";
  import SetBudget from "./Onboarding/SetBudget.svelte";
  import TopUp from "./Onboarding/TopUp.svelte";
  import AddReceivers from "./Onboarding/AddReceivers.svelte";
  import Review from "./Onboarding/Review.svelte";

  import * as modal from "../../src/modal";
  import { store } from "../../src/funding/pool";
  import type { Receivers } from "../../src/funding/pool";

  import Big from "big.js";

  enum Step {
    Erc20Allowance = "erc20",
    Intro = "intro",
    SetBudget = "budget",
    AddReceivers = "receivers",
    TopUp = "topup",
    Review = "review",
  }

  function resolveFirstStep(): Step {
    return ($store?.data?.unwrap()?.erc20Allowance || 0) > 0
      ? Step.Intro
      : Step.Erc20Allowance;
  }

  let currentStep = resolveFirstStep();

  function onCancel() {
    modal.hide();
  }

  function onContinue() {
    currentStep = nextStep();
  }

  function nextStep(): Step {
    switch (currentStep) {
      case Step.Erc20Allowance:
        return Step.Intro;
      case Step.Intro:
        return Step.SetBudget;
      case Step.SetBudget:
        return Step.AddReceivers;
      case Step.AddReceivers:
        return Step.TopUp;
      case Step.TopUp:
        return Step.Review;
      default:
        return Step.Review;
    }
  }

  function onBack() {
    currentStep = prevStep();
  }

  function prevStep(): Step {
    switch (currentStep) {
      case Step.AddReceivers:
        return Step.SetBudget;
      case Step.TopUp:
        return Step.AddReceivers;
      case Step.Review:
        return Step.TopUp;
      default:
        return Step.Intro;
    }
  }

  async function approveErc20(): Promise<void> {
    return (
      $store?.approveErc20().then(onContinue) ||
      Promise.reject("The pool is not instantiated")
    );
  }

  async function onConfirmed(): Promise<void> {
    return (
      $store
        ?.onboard(Big(topUp), Big(budget), receivers)
        .then(() => modal.hide()) ||
      Promise.reject("The pool is not instantiated")
    );
  }

  let budget = "";
  let topUp = "";
  let receivers: Receivers = new Map();
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    text-align: center;
  }
</style>

<div class="wrapper">
  {#if currentStep === Step.Erc20Allowance}
    <Erc20Allowance {onCancel} onConfirm={approveErc20} />
  {:else if currentStep === Step.Intro}
    <Intro {onCancel} {onContinue} />
  {:else if currentStep === Step.SetBudget}
    <SetBudget bind:budget {onCancel} {onContinue} />
  {:else if currentStep === Step.AddReceivers}
    <AddReceivers bind:receivers {onBack} {onContinue} />
  {:else if currentStep === Step.TopUp}
    <TopUp bind:amount={topUp} {onBack} {onContinue} />
  {:else}
    <Review {budget} {receivers} {topUp} {onBack} {onConfirmed} />
  {/if}
</div>
