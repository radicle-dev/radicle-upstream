<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { SvelteComponent } from "svelte";

  import * as modal from "ui/src/modal";
  import type {
    EnsConfiguration,
    EnsMetadataPayload,
    SubmitPayload,
  } from "./ens-flow.types";
  import type { Registration } from "ui/src/org/ensResolver";
  import Modal from "ui/DesignSystem/Modal.svelte";

  import ConfirmEnsName from "./steps/ConfirmEnsName.svelte";
  import EnsSetupFlowIntro from "./steps/EnsSetupFlowIntro.svelte";
  import EnterEnsName from "./steps/EnterEnsName.svelte";
  import WaitingToRegister from "./steps/WaitingToRegister";
  import ConfirmRegistration from "./steps/ConfirmRegistration.svelte";
  import RegistrationSuccess from "./steps/RegistrationSuccess.svelte";
  import PopulateMetadata from "./steps/PopulateMetadata.svelte";
  import UpdateMetadataSuccess from "./steps/UpdateMetadataSuccess.svelte";
  import LinkOrgToName from "./steps/LinkOrgToName.svelte";
  import LinkOrgToNameSuccess from "./steps/LinkOrgToNameSuccess.svelte";

  export let orgAddress: string;
  export let registration: Registration | undefined = undefined;

  interface Step {
    component: typeof SvelteComponent;
    onSubmit?: (payload: SubmitPayload) => void;
  }

  let ensConfiguration: Partial<EnsConfiguration> = {};

  let ensMetadataConfiguration: Partial<EnsMetadataPayload> = {
    ...registration,
    address: orgAddress,
  };

  const createNameFlow: Step[] = [
    {
      component: EnsSetupFlowIntro,
    },
    {
      component: EnterEnsName,
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensNameConfiguration) {
          throw new Error(
            "Expected EnterName step to return ensNameConfiguration"
          );
        }

        ensConfiguration = {
          ...ensConfiguration,
          ...payload.ensNameConfiguration,
        };

        ensMetadataConfiguration = {
          ...payload.ensMetadata,
        };

        if (ensConfiguration.registered) {
          switchFlow(populateNameMetadataFlow);
        } else {
          goForward();
        }
      },
    },
    {
      component: ConfirmEnsName,
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensNameConfiguration?.minAge) {
          throw new Error(
            "Expected ConfirmName step to return ensNameConfiguration with minAge"
          );
        }

        ensConfiguration = {
          ...ensConfiguration,
          ...payload.ensNameConfiguration,
        };

        goForward();
      },
    },
    {
      component: WaitingToRegister,
    },
    {
      component: ConfirmRegistration,
    },
    {
      component: RegistrationSuccess,
      onSubmit: () => {
        switchFlow(populateNameMetadataFlow);
      },
    },
  ];

  const populateNameMetadataFlow: Step[] = [
    {
      component: PopulateMetadata,
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensMetadata) {
          throw new Error(
            "Expected PopulateMetadata step to return ensMetadata"
          );
        }

        ensMetadataConfiguration = {
          ...payload.ensMetadata,
        };

        goForward();
      },
    },
    {
      component: UpdateMetadataSuccess,
      onSubmit: () => {
        switchFlow(linkRegistrationFlowSingleSign);
      },
    },
  ];

  const linkRegistrationFlowSingleSign: Step[] = [
    {
      component: LinkOrgToName,
    },
    {
      component: LinkOrgToNameSuccess,
    },
  ];

  const onSuccess = () => {
    modal.hide();
  };

  let currentStepIndex = 0;

  let currentFlow: Step[] = createNameFlow;

  $: currentStep = currentFlow[currentStepIndex];

  function goForward() {
    if (currentFlow[currentStepIndex + 1]) {
      currentStepIndex += 1;
    } else {
      onSuccess();
    }
  }

  function goToStep(stepNumber: number) {
    currentStepIndex = stepNumber;
  }

  function switchFlow(flow: Step[]) {
    currentFlow = flow;
    goToStep(0);
  }
</script>

<style>
  .content {
    text-align: center;
    position: relative;
  }
</style>

<Modal>
  <div class="content" style="position: relative">
    <svelte:component
      this={currentStep.component}
      {ensConfiguration}
      {ensMetadataConfiguration}
      {registration}
      {orgAddress}
      onSubmit={currentStep.onSubmit || goForward} />
  </div>
</Modal>
