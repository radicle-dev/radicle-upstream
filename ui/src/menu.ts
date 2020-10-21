import { SvelteComponent } from "svelte";

export interface HorizontalItem {
  icon: typeof SvelteComponent;
  title: string;
  href: string;
  looseActiveStateMatching: boolean;
  counter?: number;
}
