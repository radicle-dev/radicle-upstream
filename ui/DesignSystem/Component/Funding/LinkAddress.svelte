<script lang="typescript">
  import { Illustration } from "../../Component";
  import { Button } from "../../Primitive";

  import { Variant as IllustrationVariant } from "../../../src/illustration";
  import * as modal from "../../../src/modal";
  import * as path from "../../../src/path";

  function onLink() {
    modal.toggle(path.linkAddress());
  }
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 10vh 0;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 8px;
    width: 100%;
  }

  .inner {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    width: 380px;
    margin: 0 auto;
  }

  p {
    margin-top: 1rem;
    margin-bottom: 1.25rem;
  }
</style>

<div class="wrapper">
  <div class="inner">
    <Illustration variant={IllustrationVariant.Purse} />
    <p class="typo-text">
      In order to give and receive funds, you need to link your Radicle Identity
      to Ethereum.
    </p>
    <Button on:click={onLink}>Link your ID</Button>
  </div>
</div>
