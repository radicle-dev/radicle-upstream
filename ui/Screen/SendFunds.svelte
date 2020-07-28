<script>
  import { getContext } from "svelte";
  import { fromStore, toStore, amountStore } from "../src/transfer.ts";

  import { Dropdown, ModalLayout } from "../DesignSystem/Component";
  import { Button, Icon, Input, Title } from "../DesignSystem/Primitive";

  const { identity } = getContext("session");
  const { orgs } = getContext("session");

  let fromAddress;
  fromStore.subscribe(value => {
    fromAddress = value;
  });
  let toAddress;
  toStore.subscribe(value => {
    toAddress = value;
  });
  let amount;
  amountStore.subscribe(value => {
    amount = value;
  });

  console.log(fromAddress, " - ", toAddress, " - ", amount);

  const dropdownOptions = [];
  dropdownOptions.push({
    variant: "avatar",
    value: identity.accountId,
    avatarProps: {
      variant: "circle",
      title: identity.metadata.handle,
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.imageUrl,
    },
  });
  for (let i = 0; i < orgs.length; i++) {
    dropdownOptions.push({
      variant: "avatar",
      value: orgs[i].accountId,
      avatarProps: {
        variant: "square",
        title: orgs[i].id,
        avatarFallback: orgs[i].avatarFallback,
        imageUrl: null,
      },
    });
  }

  // compare who the transfer is from and set the dropdown value

  let step = 1;

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };
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
    margin-bottom: 1rem;
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
    padding-top: 1rem;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    {#if step === 1}
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <Title variant="big">Outgoing transfer</Title>
      </header>
      <Title style="padding: 0.5rem;">To</Title>
      <Input.Text
        bind:value={toAddress}
        placeholder="Enter recipient address"
        style="flex: 1; padding-bottom: 0.5rem;" />
      <Title style="padding: 0.5rem;">Amount</Title>
      <Input.Text
        placeholder="Enter the amount"
        bind:value={amount}
        showLeftItem
        style="flex: 1; padding-bottom: 0.5rem;">
        <div slot="left" style="display: flex;">
          <Icon.Currency
            size="big"
            style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>
      <Title style="padding: 0.5rem;">From</Title>
      <!-- TODO(julien): shouldn't identity.accountId be the same as fromAddress -->
      <Dropdown
        placeholder="Select wallet you want to use"
        bind:value={fromAddress}
        options={dropdownOptions} />
      <div class="submit">
        <Button on:click={() => nextStep()}>Review transfer</Button>
      </div>
    {/if}
    {#if step === 2}
      <header>step 2</header>
      <p>amount: {amount}</p>
      <p>to: {toAddress}</p>
      <p>from: {fromAddress}</p>
      <Button on:click={() => previousStep()}>back</Button>
    {/if}
  </div>
</ModalLayout>
