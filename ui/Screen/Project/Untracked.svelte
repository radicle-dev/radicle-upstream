<script>
  import twemoji from "twemoji";

  import {
    Copyable,
    SidebarLayout,
    TrackToggle,
  } from "../../DesignSystem/Component";
  import { Code, Text } from "../../DesignSystem/Primitive";

  export let params = null;
  const uri = params.uri;

  console.log(uri);
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    flex-direction: column;
  }

  .emoji {
    height: 44px;
    width: 44px;
    margin-bottom: 24px;
  }

  .uri {
    background: var(--color-foreground-level-2);
    padding: 4px;
    border-radius: 4px;
    margin-bottom: 24px;
    max-width: 90%;
    overflow: hidden;
  }
</style>

<SidebarLayout>
  <div class="container">
    <div class="emoji">
      {@html twemoji.parse('🎷', { base: '', folder: 'twemoji/', ext: '.svg' })}
    </div>

    <div class="uri">
      <Copyable>
        <Code
          variant="medium"
          style="font-size: 14px; color: var(--color-foreground-level-6);
          text-overflow: ellipsis; white-space: nowrap; overflow: hidden;">
          {uri}
        </Code>
      </Copyable>
    </div>

    <Text
      style="max-width: 384px; color: var(--color-foreground-level-6);
      margin-bottom: 24px;"
      align="center">
      You’re not tracking this project yet, so there’s nothing to show here.
      Track it and you’ll be notified as soon as it’s available.
    </Text>

    <TrackToggle />
  </div>
</SidebarLayout>
