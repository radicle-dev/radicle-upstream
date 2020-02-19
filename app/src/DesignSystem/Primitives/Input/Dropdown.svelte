<script>
  export let items = null;
  export let value = null;
  export let disabled = null;

  export let style = null;
  export let dataCy = null;
</script>

<style>
  select {
    -moz-appearance: none;
    -webkit-appearance: none;
    appearance: none;
    height: 36px;
    border: 1px solid var(--color-lightgray);
    border-radius: 2px;
    color: var(--color-darkgray);
    font-family: var(--typeface-medium);
    font-size: 16px;
    background: var(--color-white)
      url('data:image/svg+xml;utf8,<svg width="10" height="6" viewBox="0 0 10 6" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M0.292893 0.292893C0.683417 -0.0976311 1.31658 -0.0976311 1.70711 0.292893L5 3.58579L8.29289 0.292893C8.68342 -0.0976311 9.31658 -0.0976311 9.70711 0.292893C10.0976 0.683417 10.0976 1.31658 9.70711 1.70711L5.70711 5.70711C5.31658 6.09763 4.68342 6.09763 4.29289 5.70711L0.292893 1.70711C-0.0976311 1.31658 -0.0976311 0.683417 0.292893 0.292893Z" fill="%23546474"/></svg>')
      no-repeat;
    background-position: right 14px top 55%;
    padding: 0 34px 4px 12px;
  }

  select:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-purple));
    border: 1px solid var(--focus-outline-color, var(--color-purple));
  }

  select:disabled {
    color: var(--color-gray);
    background: var(--color-white)
      url('data:image/svg+xml;utf8,<svg width="10" height="6" viewBox="0 0 10 6" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M0.292893 0.292893C0.683417 -0.0976311 1.31658 -0.0976311 1.70711 0.292893L5 3.58579L8.29289 0.292893C8.68342 -0.0976311 9.31658 -0.0976311 9.70711 0.292893C10.0976 0.683417 10.0976 1.31658 9.70711 1.70711L5.70711 5.70711C5.31658 6.09763 4.68342 6.09763 4.29289 5.70711L0.292893 1.70711C-0.0976311 1.31658 -0.0976311 0.683417 0.292893 0.292893Z" fill="%2390a0af"/></svg>')
      no-repeat;
    background-position: right 14px top 55%;
  }
</style>

<select data-cy={dataCy} {style} bind:value {disabled}>
  {#each items as item}
    <option value={item}>{item}</option>
  {/each}
</select>
