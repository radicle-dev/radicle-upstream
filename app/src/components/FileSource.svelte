<script>
  import { Icon } from "../DesignSystem";

  export let path = null;
  export let code = null;
</script>

<style>
  .file-source {
    border: 1px solid var(--color-lightgray);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header {
    display: flex;
    background-color: var(--color-almostwhite);
    font-family: "GT America Mono Regular";
    font-size: 14px;
    height: 48px;
    align-items: center;
    padding-left: 13px;
    border-bottom: 1px solid var(--color-lightgray);
  }

  .line-numbers {
    font-family: "GT America Mono Regular";
    font-size: 14px;
    background-color: var(--color-almostwhite);
    color: var(--color-gray);
    text-align: center;
    flex: 0 0 49px;
    border-right: 1px solid var(--color-lightgray);
    user-select: none;
  }

  .code {
    font-family: "GT America Mono Regular";
    font-size: 14px;
    padding-left: 8px;
    overflow-x: scroll;
  }

  .container {
    display: flex;
  }
</style>

<div class="file-source">
  <header>
    <Icon.File />
    {path}
  </header>
  <div class="container">
    <pre class="line-numbers">
      {@html code
        .split('\n')
        .slice(0, -1)
        .map((_, index) => {
          return `${index + 1}`;
        })
        .join('\n')}
    </pre>
    <pre class="code">{code}</pre>
  </div>
</div>
