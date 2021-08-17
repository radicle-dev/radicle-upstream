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

  import Intro from "./ConfigureEns/Intro.svelte";
  import LinkOrgToName from "./ConfigureEns/LinkOrgToName.svelte";
  import RegisterName from "./ConfigureEns/RegisterName.svelte";
  import type * as registerName from "./ConfigureEns/RegisterName.svelte";
  import UpdateMetadata from "./ConfigureEns/UpdateMetadata.svelte";

  export let orgAddress: string;
  export let registration: ensResolver.Registration | undefined = undefined;
  export let safeAddress: string | undefined = undefined;
  export let fee: ethers.BigNumber;

  type State =
    | { type: "intro" }
    | { type: "registerName"; currentName: string | undefined }
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
      return { type: "registerName", currentName };
    } else {
      return { type: "intro" };
    }
  })();

  async function registrationDone(result: registerName.Result) {
    let registration: ensResolver.Registration | null;

    if (result.registration) {
      registration = result.registration;
    } else {
      const domain = `${result.name}.${ensResolver.DOMAIN}`;
      registration = await ensResolver.getRegistration(domain);

      // TODO(thomas): handle exception
      if (!registration) {
        throw new error.Error({
          message: "Domain not registered",
          details: { domain },
        });
      }
    }

    state = {
      type: "updateMetadata",
      registration,
    };
  }

  function introDone() {
    const currentName = registration?.domain.replace(
      `.${ensResolver.DOMAIN}`,
      ""
    );

    state = {
      type: "registerName",
      currentName,
    };
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

{#if state.type === "intro"}
  <Intro onSubmit={introDone} {fee} />
{:else if state.type === "registerName"}
  <RegisterName currentName={state.currentName} {fee} {registrationDone} />
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
