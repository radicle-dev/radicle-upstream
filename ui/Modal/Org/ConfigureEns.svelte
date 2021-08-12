<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as ethers from "ethers";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as modal from "ui/src/modal";
  import * as error from "ui/src/error";
  import { unreachable } from "ui/src/unreachable";

  import Modal from "ui/DesignSystem/Modal.svelte";

  import ConfigureEnsIntro from "./ConfigureEns/ConfigureEnsIntro.svelte";
  import EnterEnsName from "./ConfigureEns/EnterEnsName.svelte";
  import type * as enterEnsName from "./ConfigureEns/EnterEnsName.svelte";
  import LinkOrgToName from "./ConfigureEns/LinkOrgToName.svelte";
  import UpdateMetadata from "./ConfigureEns/UpdateMetadata.svelte";
  import Register from "./ConfigureEns/Register.svelte";

  export let orgAddress: string;
  export let registration: ensResolver.Registration | undefined = undefined;
  export let safeAddress: string | undefined = undefined;

  type State =
    | { type: "intro" }
    | { type: "enterEnsName"; currentName: string | undefined }
    | { type: "register"; name: string; fee: ethers.BigNumber }
    | {
        type: "updateMetadata";
        registration: ensResolver.Registration;
      }
    | {
        type: "linkOrgToName";
        domain: string;
      };

  let state: State = ((): State => {
    if (registration) {
      const currentName = registration.domain.replace(
        `.${ensResolver.DOMAIN}`,
        ""
      );
      return { type: "enterEnsName", currentName };
    } else {
      return { type: "intro" };
    }
  })();

  function bindRegistrationDone(name: string) {
    const domain = `${name}.${ensResolver.DOMAIN}`;
    return async function () {
      // TODO handle exception
      const registration = await ensResolver.getRegistration(domain);
      if (!registration) {
        throw new error.Error({
          message: "Domain not registered",
          details: { domain },
        });
      }
      state = {
        type: "updateMetadata",
        registration,
      };
    };
  }

  function configureEnsIntroDone() {
    const currentName = registration?.domain.replace(
      `.${ensResolver.DOMAIN}`,
      ""
    );

    state = {
      type: "enterEnsName",
      currentName,
    };
  }

  function enterEnsNameDone(result: enterEnsName.Result) {
    if (result.registration) {
      state = {
        type: "updateMetadata",
        registration: result.registration,
      };
    } else {
      state = {
        type: "register",
        name: result.name,
        fee: result.fee,
      };
    }
  }

  function bindPopulateMetadataDone(domain: string) {
    return () => {
      state = {
        type: "linkOrgToName",
        domain,
      };
    };
  }

  function linkOrgToNameDone() {
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
    {#if state.type === "intro"}
      <ConfigureEnsIntro onSubmit={configureEnsIntroDone} />
    {:else if state.type === "enterEnsName"}
      <EnterEnsName
        currentName={state.currentName}
        onSubmit={enterEnsNameDone} />
    {:else if state.type === "register"}
      <Register
        name={state.name}
        fee={state.fee}
        done={bindRegistrationDone(state.name)} />
    {:else if state.type === "updateMetadata"}
      <UpdateMetadata
        {orgAddress}
        registration={state.registration}
        onSubmit={bindPopulateMetadataDone(state.registration.domain)} />
    {:else if state.type === "linkOrgToName"}
      <LinkOrgToName
        domain={state.domain}
        {orgAddress}
        {safeAddress}
        onSubmit={linkOrgToNameDone} />
    {:else}
      {unreachable(state)}
    {/if}
  </div>
</Modal>
