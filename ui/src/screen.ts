import { SvelteComponent } from "svelte";
import * as view from "./view";

import Blank from "../Screen/Blank.svelte";
import Help from "../Screen/Help.svelte";
import IdentityCreation from "../Screen/IdentityCreation.svelte";
import Profile from "../Screen/Profile.svelte";

export enum Screen {
  Blank = "Blank",
  Help = "Help",
  IdentityCreation = "IdentityCreation",
  ProfileProjects = "ProfileProjects",
}

export const map: view.Map<Screen, typeof SvelteComponent> = {
  [Screen.Blank]: Blank,
  [Screen.Help]: Help,
  [Screen.IdentityCreation]: IdentityCreation,
  [Screen.ProfileProjects]: Profile,
};
