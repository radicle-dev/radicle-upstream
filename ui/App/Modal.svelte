<script>
  import qs from "qs";
  import regexparam from "regexparam";
  import { querystring } from "svelte-spa-router";

  import Help from "../Screen/Help.svelte";
  import IdentityCreation from "../Screen/IdentityCreation.svelte";
  import MemberRegistration from "../Screen/Org/MemberRegistration.svelte";
  import OrgRegistration from "../Screen/OrgRegistration.svelte";
  import ProjectCreation from "../Screen/ProjectCreation.svelte";
  import ProjectRegistration from "../Screen/ProjectRegistration.svelte";
  import UserRegistration from "../Screen/UserRegistration.svelte";
  import TransactionDetails from "../Screen/TransactionDetails.svelte";

  const modals = {
    "/help": Help,
    "/identity/new": IdentityCreation,
    "/orgs/register": OrgRegistration,
    "/orgs/:id/members/register": MemberRegistration,
    "/projects/new": ProjectCreation,
    "/projects/register/:domainId": ProjectRegistration,
    "/projects/:projectId/register/:domainId": ProjectRegistration,
    "/user-registration": UserRegistration,
    "/transactions/:id": TransactionDetails,
  };

  $: component = null;
  $: modal = qs.parse($querystring).modal;
  $: if ($modal) {
    console.log(regexparam(qs.parse($querystring)));
    component = modals[$modal];
  }
</script>

{#if component}
  <svelte:component this={component} />
{/if}
