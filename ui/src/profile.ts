import { SvelteComponent } from "svelte";
import * as view from "./view";

import Projects from "../Screen/Profile/Projects.svelte";
import Wallet from "../Screen/Profile/Wallet.svelte";

export enum Fragment {
  Projects = "Projects",
  Wallet = "Wallet",
}

export const fragments: view.Map<Fragment, typeof SvelteComponent> = {
  [Fragment.Projects]: Projects,
  [Fragment.Wallet]: Wallet,
};

export interface MenuItem {
  click(): void,
  icon: typeof SvelteComponent,
  title: string,
}
