<script lang="ts">
  import { Variant as IllustrationVariant } from "../src/illustration";

  import { Illustration } from "../DesignSystem/Component";
</script>

<style>
  .container {
    width: 38.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  .container:focus {
    outline: none;
  }
</style>

<div data-cy="remotes-modal" class="container">
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Keyboard} />
  <h1>Manage remotes</h1>
  <div>MANAAAAAAGE</div>
</div>
