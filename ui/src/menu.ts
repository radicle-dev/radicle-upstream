import type { SvelteComponent } from "svelte";
import type { Route } from "ui/src/router";

export interface HorizontalItem {
  icon: typeof SvelteComponent;
  title: string;
  tab: Route;
  counter?: number;
}
