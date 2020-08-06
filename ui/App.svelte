<script lang="ts">
  import { setContext } from "svelte";

  import * as notification from "./src/notification";
  import * as remote from "./src/remote";
  import * as screen from "./src/screen";
  import { clear, fetch, session as store } from "./src/session";
  import * as view from "./src/view";

  import { NotificationFaucet, Remote } from "./DesignSystem/Component";
  import { Button } from "./DesignSystem/Primitive";

  import Hotkeys from "./Hotkeys.svelte";
  import Theme from "./Theme.svelte";
  import ViewRouter from "./View/Router.svelte";

  const screens = view.create(screen.map, screen.Screen.ProfileProjects);
  setContext("screens", screens);

  $: switch ($store.status) {
    case remote.Status.NotAsked:
      fetch();
      break;

    case remote.Status.Success:
      if ($store.data.identity === null) {
        screens.set(screen.Screen.IdentityCreation);
      }
      break;

    case remote.Status.Error:
      console.error($store.error);
      notification.error("Session could not be fetched");
      break;
  }
</script>

<style>
  .error {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  h2 {
    margin-bottom: 32px;
  }
</style>

<Hotkeys />
<NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />
<Theme />
<Remote {store} context="session">
  <ViewRouter nav={screens} />

  <div slot="error" class="error">
    <h2>We're having trouble logging you into radicle. 😪</h2>
    <Button on:click={clear}>Clear Session</Button>
  </div>
</Remote>
