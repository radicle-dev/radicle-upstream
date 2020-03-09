<script>
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";
  import { pop, push } from "svelte-spa-router";

  import { setCurrentIdentity } from "../store/identity.js";
  import { showNotification } from "../store/notification.js";

  import { Button, Input, Text, Title } from "../DesignSystem/Primitive";
  import { ModalLayout } from "../DesignSystem/Component";

  let handle, displayName, avatarUrl;

  const client = getClient();

  const CREATE_IDENTITY = gql`
    mutation($handle: String!, $displayName: String, $avatarUrl: String) {
      createIdentity(
        handle: $handle
        displayName: $displayName
        avatarUrl: $avatarUrl
      ) {
        id
        shareableEntityIdentifier
        metadata {
          handle
          displayName
          avatarUrl
        }
      }
    }
  `;

  const createIdentity = async () => {
    let response;

    try {
      response = await mutate(client, {
        mutation: CREATE_IDENTITY,
        variables: {
          handle: handle,
          displayName: displayName,
          avatarUrl: avatarUrl
        }
      });

      const responseData = response.data.createIdentity;
      setCurrentIdentity({
        handle: responseData.metadata.handle,
        displayName: responseData.metadata.displayName,
        avatarUrl: responseData.metadata.avatarUrl,
        shareableEntityIdentifier: responseData.shareableEntityIdentifier
      });
      push("/identity/success");
    } catch (error) {
      pop();
      showNotification({
        text: `Could not create identity: ${error}`,
        level: "error"
      });
    }
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
  }
</style>

<ModalLayout>
  <div class="container">
    <div>
      <Title variant="big" style="text-align: center;">
        Create an identity
      </Title>
      <Text style="margin: 20px 0; color: var(--color-gray);">
        An identity is required to interact on the radicle network. Multiple
        devices can be linked to a single identity.
      </Text>
      <Input.Text placeholder="Enter a handle*" bind:value={handle} />
      <Input.Text
        placeholder="Add a display name"
        bind:value={displayName}
        style="margin-top: 16px;" />
      <Input.Text
        placeholder="Avatar url"
        bind:value={avatarUrl}
        style="margin: 16px 0 32px 0;" />

      <div class="buttons">
        <Button variant="transparent" size="big" style="margin-right: 16px;">
          Cancel
        </Button>
        <Button disabled={!handle} size="big" on:click={createIdentity}>
          Create
        </Button>
      </div>
    </div>
  </div>
</ModalLayout>
