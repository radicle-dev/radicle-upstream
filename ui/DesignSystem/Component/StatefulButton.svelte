<script lang="ts">
  import { Button, Icon } from "../Primitive";
  import { Spinner } from "../Component";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";

  export let title: string = "";
  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  export let formatError: (error: any) => string;

  enum Status {
    Idle,
    Waiting,
    Succeeded,
    Failed,
  }

  let status = Status.Idle;

  async function userDidClick(): Promise<void> {
    try {
      await setStatus(Status.Waiting);
      await onClick();
      await setStatus(Status.Succeeded);
    } catch (error) {
      notification.error(formatError(error));
      await setStatus(Status.Failed, 2);
    } finally {
      await setStatus(Status.Idle);
    }
  }

  function setStatus(
    newStatus: Status,
    delayInSeconds: number = 1
  ): Promise<void> {
    status = newStatus;
    return continueAfter(delayInSeconds);
  }

  function continueAfter(seconds: number): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve();
      }, seconds * 1000);
    });
  }
</script>

<style>
  .button-wrapper {
    display: inline-flex;
    align-items: center;
    /* Having a min-height helps the UI staying fixed when switching statuses.*/
    min-height: 40px;
  }
</style>

<span class="button-wrapper" data-cy={dataCy}>
  {#if status === Status.Idle}
    <Button {variant} on:click={userDidClick}>{title}</Button>
  {:else if status === Status.Waiting}
    <Spinner />
  {:else if status === Status.Succeeded}
    <Icon.CheckCircle style={`fill: var(--color-positive)`} />
  {:else if status === Status.Failed}
    <Icon.CrossCircle style={`fill: var(--color-negative)}`} />
  {/if}
</span>
