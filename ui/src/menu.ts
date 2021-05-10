import type { SvelteComponent } from "svelte";
import type { State } from "ui/src/router";

export interface HorizontalItem {
  icon: typeof SvelteComponent;
  title: string;
  tab: State;
  counter?: number;
}
