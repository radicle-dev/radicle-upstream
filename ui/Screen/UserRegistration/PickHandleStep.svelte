<script>
  import validatejs from "validate.js";
  import { Button, Flex, Input } from "../../DesignSystem/Primitive";

  import { pop } from "svelte-spa-router";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  export let onNextStep = null;

  const nextStep = () => {
    if (!validatejs.isEmpty(validations)) {
      return;
    }
    onNextStep();
  };

  export let handle = "";
  export let imageUrl = null;
  export let avatarFallback = null;

  let validating = false;
  let validations = false;
  let timeout = null;
  let delay = 0;
  const client = getClient();

  const GET_USER = gql`
    query Query($handle: ID!) {
      user(handle: $handle)
    }
  `;

  const GET_AVATAR = gql`
    query Query($handle: ID!) {
      avatar(handle: $handle) {
        emoji
        background {
          r
          g
          b
        }
      }
    }
  `;

  const validateHandleAvailability = async () => {
    try {
      const response = await query(client, {
        query: GET_USER,
        variables: { handle: handle }
      });
      const result = await response.result();
      if (await result.data.user) {
        validations = { handle: ["Handle already taken"] };
      }
    } catch (error) {
      validations = { handle: [error] };
    }
  };

  const updateAvatarFallback = async () => {
    const response = await query(client, {
      query: GET_AVATAR,
      variables: { handle: handle }
    });
    const result = await response.result();
    avatarFallback = await result.data.avatar;
  };

  validatejs.options = {
    fullMessages: false
  };

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  const constraints = {
    handle: {
      presence: {
        message: "Handle is required",
        allowEmpty: false
      },
      format: {
        pattern: VALID_NAME_MATCH,
        message: "Handle should match [a-z0-9][a-z0-9_-]+"
      }
    }
  };

  const validate = async () => {
    validating = true;
    if (!handle) {
      avatarFallback = null;
    }
    validations = validatejs({ handle: handle }, constraints);
    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      clearTimeout(timeout);
      timeout = setTimeout(async () => {
        updateAvatarFallback();
        await validateHandleAvailability();
        validating = false;
      }, delay);
      // set the delay after the first validation on load
      delay = 1000;
    }
  };

  $: validate(handle);
</script>

<Input.Text
  dataCy="handle"
  {avatarFallback}
  {imageUrl}
  style="--focus-outline-color: var(--color-pink)"
  placeholder="User handle"
  bind:value={handle}
  valid={!(validations && validations.handle)}
  validationMessage={validations && validations.handle && validations.handle[0]}
  variant="handle"
  validationPending={validating} />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>
  <Button
    dataCy="next-button"
    disabled={!handle || validating || validations}
    on:click={nextStep}
    variant="primary">
    Next
  </Button>
</Flex>
