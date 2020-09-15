declare module "svelte-spa-router" {
  import type { SvelteComponent } from "svelte";

  export default class Router extends SvelteComponent {
    $$prop_def: {
      routes: {
        [path: string]: typeof SvelteComponent;
      };
    };
  }
}
