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
  } from "./ConfigureEns/ens-flow.types";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";

  import Modal from "ui/DesignSystem/Modal.svelte";

  import ConfigureEnsIntro from "./ConfigureEns/ConfigureEnsIntro.svelte";
  import ConfirmEnsName from "./ConfigureEns/ConfirmEnsName.svelte";
  import ConfirmRegistration from "./ConfigureEns/ConfirmRegistration.svelte";
  import EnterEnsName from "./ConfigureEns/EnterEnsName.svelte";
  import LinkOrgToName from "./ConfigureEns/LinkOrgToName.svelte";
  import LinkOrgToNameSuccess from "./ConfigureEns/LinkOrgToNameSuccess.svelte";
  import PopulateMetadata from "./ConfigureEns/PopulateMetadata.svelte";
  import RegistrationSuccess from "./ConfigureEns/RegistrationSuccess.svelte";
  import UpdateMetadataSuccess from "./ConfigureEns/UpdateMetadataSuccess.svelte";
  import WaitingToRegister from "./ConfigureEns/WaitingToRegister.svelte";

  export let orgAddress: string;
  export let registration: ensResolver.Registration | undefined = undefined;
  export let safeAddress: string | undefined = undefined;
  export let currentStepIndex: number = 0;

  // TODO: make sure onSubmit is required on a type level.
  interface Step {
    component: typeof SvelteComponent;
    props: () => { [propName: string]: unknown };
  }

  let ensConfiguration: Partial<EnsConfiguration> = {};

  let ensMetadataConfiguration: Partial<EnsMetadataPayload> = {
    ...registration,
    address: orgAddress,
  };

  const createNameFlow: Step[] = [
    {
      component: ConfigureEnsIntro,
      props: () => {
        return { onSubmit: nextStep };
      },
    },
    {
      component: EnterEnsName,
      props: () => {
        return {
          ensMetadataConfiguration,
          onSubmit: (payload: SubmitPayload) => {
            if (!payload.ensNameConfiguration) {
              throw new error.Error({
                message:
                  "Expected EnterName step to return ensNameConfiguration",
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
              nextStep();
            }
          },
        };
      },
    },
    {
      component: ConfirmEnsName,
      props: () => {
        return {
          ensConfiguration,
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

            nextStep();
          },
        };
      },
    },
    {
      component: WaitingToRegister,
      props: () => {
        return {
          ensConfiguration,
          onSubmit: nextStep,
        };
      },
    },
    {
      component: ConfirmRegistration,
      props: () => {
        return {
          ensConfiguration,
          onSubmit: nextStep,
        };
      },
    },
    {
      component: RegistrationSuccess,
      props: () => {
        return {
          ensConfiguration,
          onSubmit: () => {
            switchFlow(populateNameMetadataFlow);
          },
        };
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

            nextStep();
          },
        };
      },
    },
    {
      component: UpdateMetadataSuccess,
      props: () => {
        return {
          ensConfiguration,
          onSubmit: () => {
            // There's already a registration for the org, and that
            // registration has the same name as that entered in the name entry
            // step, so we can skip linking.
            if (
              registration &&
              registration.name ===
                `${ensConfiguration.name}.${ensResolver.DOMAIN}`
            ) {
              onFlowFinished();
            } else {
              switchFlow(linkRegistrationFlow);
            }
          },
        };
      },
    },
  ];

  const linkRegistrationFlow: Step[] = [
    {
      component: LinkOrgToName,
      props: () => {
        return {
          ensMetadataConfiguration,
          ensConfiguration,
          safeAddress,
          onSubmit: nextStep,
        };
      },
    },
    {
      component: LinkOrgToNameSuccess,
      props: () => {
        return { safeAddress, ensConfiguration, onSubmit: nextStep };
      },
    },
  ];

  let currentFlow: Step[] = createNameFlow;
  $: currentStep = currentFlow[currentStepIndex];

  function nextStep() {
    if (currentFlow[currentStepIndex + 1]) {
      currentStepIndex += 1;
    } else {
      onFlowFinished();
    }
  }

  function switchFlow(flow: Step[]) {
    currentFlow = flow;
    currentStepIndex = 0;
  }

  function onFlowFinished(): void {
    modal.hide();
  }
</script>

<style>
  .content {
    text-align: center;
    position: relative;
  }
</style>

<Modal>
  <div class="content">
    <svelte:component this={currentStep.component} {...currentStep.props()} />
  </div>
</Modal>
