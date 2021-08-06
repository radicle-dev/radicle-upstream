<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import type {
    EnsConfiguration,
    EnsMetadataPayload,
    SubmitPayload,
  } from "./ens-flow.types";
  import type { Registration } from "ui/src/org/ensResolver";

  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";

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
  export let safeAddress: string | undefined = undefined;
  export let currentStepIndex: number = 0;

  interface Step {
    component: typeof SvelteComponent;
    props: () => { [propName: string]: unknown };
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
      props: () => {
        return {};
      },
    },
    {
      component: EnterEnsName,
      props: () => {
        return { ensMetadataConfiguration };
      },
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensNameConfiguration) {
          throw new error.Error({
            message: "Expected EnterName step to return ensNameConfiguration",
            details: { payload },
          });
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
      props: () => {
        return {
          ensConfiguration,
        };
      },
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensNameConfiguration?.minAge) {
          throw new error.Error({
            message:
              "Expected ConfirmName step to return ensNameConfiguration with minAge",
            details: { payload },
          });
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
      props: () => {
        return {
          ensConfiguration,
        };
      },
    },
    {
      component: ConfirmRegistration,
      props: () => {
        return {
          ensConfiguration,
        };
      },
    },
    {
      component: RegistrationSuccess,
      props: () => {
        return {
          ensConfiguration,
        };
      },
      onSubmit: () => {
        switchFlow(populateNameMetadataFlow);
      },
    },
  ];

  const populateNameMetadataFlow: Step[] = [
    {
      component: PopulateMetadata,
      props: () => {
        return {
          ensMetadataConfiguration,
          ensConfiguration,
          orgAddress,
        };
      },
      onSubmit: (payload: SubmitPayload) => {
        if (!payload.ensMetadata) {
          throw new error.Error({
            message: "Expected PopulateMetadata step to return ensMetadata",
            details: { payload },
          });
        }

        ensMetadataConfiguration = {
          ...payload.ensMetadata,
        };

        goForward();
      },
    },
    {
      component: UpdateMetadataSuccess,
      props: () => {
        return { ensConfiguration };
      },
      onSubmit: () => {
        switchFlow(linkRegistrationFlow);
      },
    },
  ];

  const linkRegistrationFlow: Step[] = [
    {
      component: LinkOrgToName,
      props: () => {
        return {
          registration,
          ensMetadataConfiguration,
          ensConfiguration,
          safeAddress,
        };
      },
    },
    {
      component: LinkOrgToNameSuccess,
      props: () => {
        return { registration, safeAddress, ensConfiguration };
      },
    },
  ];

  const onSuccess = () => {
    modal.hide();
  };

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
      {...currentStep.props()}
      onSubmit={currentStep.onSubmit || goForward} />
  </div>
</Modal>
