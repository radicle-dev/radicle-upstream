declare module "svelte-json-tree" {
  import type { SvelteComponentTyped } from "svelte";
  interface Props {
    key?: string;
    value: any; // eslint-disable-line @typescript-eslint/no-explicit-any
  }
  export default class JSONTree extends SvelteComponentTyped<Props> {}
}
