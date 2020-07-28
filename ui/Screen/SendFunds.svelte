<script>
  import { getContext } from "svelte";
  import { fromStore, toStore, amountStore } from "../src/transfer.ts";

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
  <div class="wrapper">
    {#if step === 1}
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <Title variant="big">Outgoing transfer</Title>
      </header>
      <Title
        style="color: var(--color-foreground-level-6); padding: 0 0.5rem 0.5rem
        0.5rem;">
        To
      </Title>
      <Input.Text
        bind:value={toAddress}
        placeholder="Enter recipient address"
        style="flex: 1; padding-bottom: 0.5rem;" />
      <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
        Amount
      </Title>
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
      <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
        From
      </Title>
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
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <Title variant="big">Outgoing transfer</Title>
        <div class="from-to">
          <div class="from">
            {#each dropdownOptions as option}
              {#if option.value === fromAddress}
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
              {toAddress}
            </Title>
          </div>
        </div>
      </header>
      <Row variant="top" style="">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Amount
          </Text>
        </div>

        <div slot="right">
          <Rad rad={amount} />
        </div>
      </Row>
      <Row variant="middle" style="">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Transaction Fee
          </Text>
        </div>

        <div slot="right">
          <Rad rad={amount} />
        </div>
      </Row>
      <Row
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
          <Rad rad={amount} />
        </div>
      </Row>
      <Row style="margin-top: 1.5rem;">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Funding source
          </Text>
        </div>

        <div slot="right">
          {#each dropdownOptions as option}
            {#if option.value === fromAddress}
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
        <Button variant="transparent" on:click={() => previousStep()}>
          back
        </Button>
        <Button style="margin-left: 1rem;" on:click>Confirm and send</Button>
      </div>
    {/if}
  </div>
</ModalLayout>
