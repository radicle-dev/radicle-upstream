<script>
  import * as navigation from "../src/navigation.ts";

  const click = () => {
    console.log("click");

    navigation.navigateToScreen(navigation.Screen.Help);
  };
</script>

<button on:click={click}>Help</button>
