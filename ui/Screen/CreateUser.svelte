<script>
  import { Button, Input, Text, Title } from "../DesignSystem/Primitive";
  import { ModalLayout } from "../DesignSystem/Component";
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
      <Input.Text placeholder="Enter a handle*" />
      <Input.Text placeholder="Add a display name" style="margin-top: 16px;" />
      <Input.Text placeholder="Avatar url" style="margin: 16px 0 32px 0;" />

      <div class="buttons">
        <Button variant="transparent" size="big">Cancel</Button>
        <Button disabled size="big">Create</Button>
      </div>
    </div>

  </div>
</ModalLayout>
