<script>
  import Text from "../Text.svelte";

  export let style = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;
  export let valid = true;
  export let validationMessage = null;
</script>

<style>
  input {
    border: 1px solid var(--color-lightgray);
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 48px;
    line-height: 48px;
    padding: 0 16px 0 16px;
    background-color: var(--color-white);
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-purple));
    border: 1px solid var(--focus-outline-color, var(--color-purple));
  }

  input.invalid:focus,
  input.invalid {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-red);
    border: 1px solid var(--color-red);
    background: var(--color-white)
      url('data:image/svg+xml;utf8,<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M19.0711 4.92893C17.1957 3.05357 14.6522 2 12 2C9.34784 2 6.8043 3.05357 4.92893 4.92893C3.05357 6.8043 2 9.34784 2 12C2 14.6522 3.05357 17.1957 4.92893 19.0711C6.8043 20.9464 9.34784 22 12 22C14.6522 22 17.1957 20.9464 19.0711 19.0711C20.9464 17.1957 22 14.6522 22 12C22 9.34784 20.9464 6.8043 19.0711 4.92893ZM10.9393 17.5607C11.2207 17.842 11.6022 18 12 18C12.3978 18 12.7794 17.842 13.0607 17.5607C13.342 17.2794 13.5 16.8978 13.5 16.5C13.5 16.1022 13.342 15.7207 13.0607 15.4394C12.7794 15.158 12.3978 15 12 15C11.6022 15 11.2207 15.158 10.9393 15.4394C10.658 15.7207 10.5 16.1022 10.5 16.5C10.5 16.8978 10.658 17.2794 10.9393 17.5607ZM11 12.1C11.12 13.3 12.87 13.3 13 12.1L13.5 7.10001C13.514 6.96054 13.4986 6.81967 13.4546 6.68656C13.4107 6.55344 13.3393 6.43105 13.245 6.32733C13.1507 6.2236 13.0356 6.14086 12.9073 6.08447C12.7789 6.02809 12.6402 5.99931 12.5 6.00001H11.5C11.3598 5.99931 11.2211 6.02809 11.0927 6.08447C10.9644 6.14086 10.8493 6.2236 10.7551 6.32733C10.6608 6.43105 10.5893 6.55344 10.5454 6.68656C10.5015 6.81967 10.486 6.96054 10.5 7.10001L11 12.1Z" fill="%23E75050"/></svg>')
      no-repeat;
    background-position: right 14px top 55%;
    padding-right: 46px;
  }
</style>

<div {style}>
  <input
    data-cy={dataCy}
    class:invalid={!valid}
    {placeholder}
    bind:value
    {disabled}
    on:change />

  {#if !valid && validationMessage}
    <Text
      variant="smallTextBold"
      style="color: var(--color-red); margin-top: 4px">
      {validationMessage}
    </Text>
  {/if}
</div>
