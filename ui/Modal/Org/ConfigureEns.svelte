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
  import UpdateMetadata from "./ConfigureEns/UpdateMetadata.svelte";

  export let orgAddress: string;
  export let registration: ensResolver.Registration | undefined = undefined;
  export let safeAddress: string | undefined = undefined;
  export let fee: ethers.BigNumber;

  type State =
    | { type: "intro" }
    | { type: "registerName"; name: string }
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
      const existingName = registration.domain.replace(
        `.${ensResolver.DOMAIN}`,
        ""
      );
      return { type: "registerName", name: existingName };
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

  function introDone() {
    const existingName = registration?.domain.replace(
      `.${ensResolver.DOMAIN}`,
      ""
    );

    state = {
      type: "registerName",
      name: existingName || "",
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
  <RegisterName
    name={state.name}
    {fee}
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
