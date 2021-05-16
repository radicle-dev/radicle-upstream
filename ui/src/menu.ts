import type { SvelteComponent } from "svelte";

export interface HorizontalItem {
  icon: typeof SvelteComponent;
  title: string;
  tab: "files" | "commits" | "commit";
  counter?: number;
}
