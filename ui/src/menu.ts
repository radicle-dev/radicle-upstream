import type { SvelteComponent } from "svelte";

interface Item {
  icon: typeof SvelteComponent;
  title: string;
  href: string;
  active?: boolean;
  counter?: string | number;
}
