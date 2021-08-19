<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";
  import * as org from "ui/src/org";
  import * as transaction from "ui/src/transaction";

  import { Modal, TextInput } from "ui/DesignSystem";

  import ButtonRow from "./ButtonRow.svelte";

  export let onSubmit: () => void;
  export let domain: string;
  export let orgAddress: string;
  export let safeAddress: string | undefined = undefined;

  let buttonsDisabled = false;

  let linked = false;

  async function link() {
    buttonsDisabled = true;

    if (safeAddress) {
      const signNotification = notification.info({
        message:
          "Waiting for you to sign the proposal in your connected wallet",
        showIcon: true,
        persist: true,
      });

      try {
        await org.proposeSetNameChange(domain, orgAddress, safeAddress);
        linked = true;
      } catch (err) {
        buttonsDisabled = false;

        error.show(
          new error.Error({
            message: err.message,
            source: err,
          })
        );
      } finally {
        signNotification.remove();
      }
    } else {
      const setNameNotification = notification.info({
        message:
          "Waiting for you to confirm the set name transaction in your connected wallet",
        showIcon: true,
        persist: true,
      });

      let tx: transaction.ContractTransaction | undefined = undefined;

      try {
        tx = await org.setNameSingleSig(domain, orgAddress);
        transaction.add(transaction.linkEnsNameToOrg(tx));
        linked = true;
      } catch (err) {
        error.show(
          new error.Error({
            message: err.message,
            source: err,
          })
        );
        return;
      } finally {
        setNameNotification.remove();
      }

      await tx.wait(1);
      // TODO: yank the cache for this org and let the user know.
    }
  }
</script>

<style>
  .label {
    padding-left: 12px;
    margin-bottom: 12px;
    color: var(--color-foreground-level-6);
  }
</style>

{#if !linked}
  <Modal
    emoji="🔗"
    title="Let’s link your name"
    desc={`In this last step, we’re updating your organization to point towards your newly created name. Once that’s done, your organization will appear with your new name across Radicle!`}>
    <div class="label typo-text-bold">Organization address</div>
    <TextInput disabled style="margin-bottom: 24px" value={orgAddress} />

    <div class="label typo-text-bold">Name</div>
    <TextInput disabled style="margin-bottom: 24px" value={domain} />

    <p
      style="color: var(--color-foreground-level-5; margin: 16px 0;"
      class="typo-text-small">
      You can also do this later by selecting "Register ENS Name" and entering
      your existing name.
    </p>

    <ButtonRow
      disableButtons={buttonsDisabled}
      onSubmit={link}
      confirmCopy="Link name to org" />
  </Modal>
{:else if safeAddress}
  <Modal
    emoji="🔗"
    title="Approve on Gnosis"
    desc={"As a final step your organisation will have to confirm the transaction on Gnosis. After it's been approved and executed your newly registered name will start appearing across Radicle in place of your organization address!"}>
    <ButtonRow
      onSubmit={() => {
        safeAddress && org.openOnGnosisSafe(safeAddress, "transactions");
        onSubmit();
      }}
      canCancel={false}
      confirmCopy="View proposal on Gnosis" />
  </Modal>
{:else}
  <Modal
    emoji="🎉"
    title="That's it!"
    desc={`Great, your organization now points to your new name ${domain}. Shortly, your name will start appearing across Radicle in place of your organization address!`}>
    <ButtonRow {onSubmit} canCancel={false} confirmCopy="Amazing, thanks!" />
  </Modal>
{/if}
