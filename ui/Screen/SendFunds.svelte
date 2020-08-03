<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import * as currency from "../src/currency.ts";
  import * as notification from "../src/notification.ts";
  import {
    TransferState,
    payerStore,
    recipientStore,
    amountStore,
    amountValidationStore,
    recipientValidationStore,
    transfer,
  } from "../src/transfer.ts";
  import { costSummary, MessageType } from "../src/transaction.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import { Dropdown, ModalLayout, Rad } from "../DesignSystem/Component";
  import Row from "../DesignSystem/Component/Transaction/Row.svelte";
  import {
    Avatar,
    Button,
    Icon,
    Input,
    Text,
    Title,
  } from "../DesignSystem/Primitive";

  const { identity } = getContext("session");
  const { orgs } = getContext("session");

  const payerAccountIds = {};
  payerAccountIds[identity.metadata.handle] = identity.accountId;
  orgs.forEach(org => (payerAccountIds[org.id] = org.accountId));

  $: recipientValidation = recipientValidationStore(
    payerAccountIds[$payerStore]
  );

  let validatingAmount = false;
  let validatingRecipient = false;

  // TODO(nuno): receive this from the user once possible
  const transactionFee = 1;

  // Filled on `goToConfirmation`
  let isTransferringFromUser = null;
  let tx = null;
  let summary = null;

  const dropdownOptions = [
    {
      variant: "avatar",
      value: identity.metadata.handle,
      avatarProps: {
        variant: "circle",
        title: identity.metadata.handle,
        avatarFallback: identity.avatarFallback,
        imageUrl: identity.imageUrl,
      },
    },
    ...orgs.map(org => ({
      variant: "avatar",
      value: org.id,
      avatarProps: {
        variant: "square",
        title: org.id,
        avatarFallback: org.avatarFallback,
        imageUrl: null,
      },
    })),
  ];

  let state = TransferState.Preparation;

  const goToConfirmation = () => {
    isTransferringFromUser = $payerStore === identity.metadata.handle;
    const amountInMicroRad = currency.radToMicroRad($amountStore);

    tx = {
      fee: transactionFee,
      messages: [
        isTransferringFromUser
          ? {
              type: MessageType.Transfer,
              amount: amountInMicroRad,
              recipient: $recipientStore,
            }
          : {
              type: MessageType.TransferFromOrg,
              amount: amountInMicroRad,
              recipient: $recipientStore,
              orgId: $payerStore,
            },
      ],
    };
    summary = costSummary(tx);
    state = TransferState.Confirmation;
  };

  const backToPreperation = () => {
    state = TransferState.Preparation;
  };

  const onConfirmed = async () => {
    try {
      await transfer(
        isTransferringFromUser,
        $payerStore,
        parseInt($amountStore),
        $recipientStore,
        transactionFee
      );
    } catch (error) {
      notification.error(`Could not transfer funds: ${error.message}`);
    } finally {
      pop();
    }
  };

  $: amountValidation = amountValidationStore(
    transactionFee,
    payerAccountIds[$payerStore]
  );
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }
  $: {
    if ($recipientStore && $recipientStore.length > 0)
      validatingRecipient = true;
    if (validatingRecipient) recipientValidation.validate($recipientStore);
  }
  $: disableSubmit =
    $amountValidation.status !== ValidationStatus.Success ||
    $recipientValidation.status !== ValidationStatus.Success ||
    $payerStore.length === 0;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    margin: 92px 0 32px 0;
    width: 540px;
  }
  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    padding: 2rem;
    margin-bottom: 1.5rem;
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }
  .icon {
    height: 2.5rem;
    width: 2.5rem;
    border-radius: 1.25rem;
    background-color: var(--color-primary-level-1);
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    padding-top: 1.5rem;
  }

  .from-to {
    display: grid;
    grid-template-columns: 13rem 1.5rem 13rem;
    grid-column-gap: 1rem;
    margin-top: 1rem;
  }

  .from {
    display: flex;
    justify-content: flex-end;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper" data-cy="send-funds-modal">
    {#if state === TransferState.Preparation}
      <div data-cy="preperation-step">
        <header>
          <div class="icon">
            <Icon.ArrowUp style="fill: var(--color-primary)" />
          </div>
          <Title variant="big">Outgoing transfer</Title>
        </header>
        <Title
          style="color: var(--color-foreground-level-6); padding: 0 0.5rem
          0.5rem 0.5rem;">
          To
        </Title>
        <Input.Text
          dataCy="modal-recipient-input"
          bind:value={$recipientStore}
          placeholder="Enter recipient address"
          style="flex: 1; padding-bottom: 0.5rem;"
          validation={$recipientValidation} />
        <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
          Amount
        </Title>
        <Input.Text
          dataCy="modal-amount-input"
          placeholder="Enter the amount"
          bind:value={$amountStore}
          showLeftItem
          style="flex: 1; padding-bottom: 0.5rem;"
          validation={$amountValidation}>
          <div slot="left" style="display: flex;">
            <Icon.Currency
              size="big"
              style="fill: var(--color-foreground-level-6)" />
          </div>
        </Input.Text>
        <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
          From
        </Title>
        <!-- TODO(julien): shouldn't identity.accountId be the same as fromAddress -->
        <Dropdown
          dataCy="modal-sender-dropdown"
          placeholder="Select wallet you want to use"
          bind:value={$payerStore}
          options={dropdownOptions}
          valid={$payerStore && $payerStore.length > 0}
          validationMessage="From entity is required" />
        <div class="submit">
          <Button
            dataCy="review-transfer-button"
            disabled={disableSubmit}
            on:click={goToConfirmation}>
            Review transfer
          </Button>
        </div>
      </div>
    {/if}
    {#if state === TransferState.Confirmation}
      <div data-cy="review-step">
        <header>
          <div class="icon">
            <Icon.ArrowUp style="fill: var(--color-primary)" />
          </div>
          <Title variant="big">Outgoing transfer</Title>
          <div class="from-to">
            <div class="from">
              {#each dropdownOptions as option}
                {#if option.value === $payerStore}
                  <Avatar
                    size="small"
                    title={option.avatarProps.title}
                    variant={option.avatarProps.variant}
                    avatarFallback={option.avatarProps.avatarFallback} />
                {/if}
              {/each}
            </div>
            <Icon.ArrowRight />
            <div class="to">
              <Title truncate style="color: var(--color-foreground-level-6);">
                {$recipientStore}
              </Title>
            </div>
          </div>
        </header>

        <Row dataCy="transfer-amount" variant="top" style="">
          <div slot="left">
            <Text
              variant="regular"
              style="color:var(--color-foreground-level-6);">
              Amount
            </Text>
          </div>

          <div slot="right">
            <Rad
              rad={summary.transferAmount.rad}
              usd={summary.transferAmount.usd} />
          </div>
        </Row>
        <Row dataCy="transaction-fee" variant="middle" style="">
          <div slot="left">
            <Text
              variant="regular"
              style="color:var(--color-foreground-level-6);">
              Transaction Fee
            </Text>
          </div>
          <div slot="right">
            <Rad rad={summary.txFee.rad} usd={summary.txFee.usd} />
          </div>
        </Row>
        <Row
          dataCy="total"
          variant="bottom"
          style="border: 1px solid var(--color-foreground-level-2);">
          <div slot="left">
            <Title
              variant="regular"
              style="color:var(--color-foreground-level-6);">
              Total
            </Title>
          </div>
          <div slot="right">
            <Rad rad={summary.total.rad} usd={summary.total.usd} />
          </div>
        </Row>
        <Row dataCy="funding-source" style="margin-top: 1.5rem;">
          <div slot="left">
            <Text
              variant="regular"
              style="color:var(--color-foreground-level-6);">
              Funding source
            </Text>
          </div>

          <div slot="right">
            {#each dropdownOptions as option}
              {#if option.value === $payerStore}
                <Avatar
                  size="small"
                  title={option.avatarProps.title}
                  variant={option.avatarProps.variant}
                  avatarFallback={option.avatarProps.avatarFallback} />
              {/if}
            {/each}
          </div>
        </Row>
        <div class="submit">
          <Button
            dataCy="back-to-review-button"
            variant="transparent"
            on:click={backToPreperation}>
            back
          </Button>
          <Button
            dataCy="submit-tranfer-button"
            style="margin-left: 1rem;"
            on:click={onConfirmed}>
            Confirm and send
          </Button>
        </div>
      </div>
    {/if}
  </div>
</ModalLayout>
