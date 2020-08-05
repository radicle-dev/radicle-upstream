import { SvelteComponent } from "svelte";
import * as view from "./view";

import Blank from "../Screen/Blank.svelte";
import Help from "../Screen/Help.svelte";

export enum Screen {
  Blank = "Blank",
  Help = "Help",
}

export const map: view.Map<Screen, typeof SvelteComponent> = {
  [Screen.Blank]: Blank,
  [Screen.Help]: Help,
};
