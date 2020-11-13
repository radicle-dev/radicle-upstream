<script lang="typescript">
  import { Illustration } from "../../../Component";
  import { Button } from "../../../Primitive";

  import { Variant as IllustrationVariant } from "../../../../src/illustration";
  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";

  function getStarted() {
    modal.toggle(path.poolOnboarding());
  }
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 15vh 0;

    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;
  }

  p {
    margin-top: 1rem;
    margin-bottom: 1.25rem;
  }
</style>

<div class="wrapper">
  <Illustration variant={IllustrationVariant.Money} />
  <p class="typo-text">Stream digital money your favorite people.</p>
  <Button on:click={getStarted}>Get started</Button>
</div>
