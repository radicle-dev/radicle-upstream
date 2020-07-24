import { SvelteComponentDev } from "svelte";
import { Readable } from "svelte/store";

import * as history from "./history";

import Blank from "../Screen/Blank.svelte";
import Help from "../Screen/Help.svelte";

console.log(typeof Blank)
console.log(Blank)

export enum Screen {
  Blank = "Blank",
  Help = "Help",
}

export interface View {
  readonly component: SvelteComponent;
  readonly props?: Props;
}

type Props = Record<string, 0 | string>;
type ComponentMap<Key extends string> = Record<Key, SvelteComponent>;

const screenMap: ComponentMap<Screen> = {
  [Screen.Blank]: Blank,
  [Screen.Help]: Help,
}

interface Navigation {
  readonly current: Readable<View>;
  back(): void;
  set(key: Screen, props?: Props): void;
}

export const create = <Key extends string>(componentMap: ComponentMap<Key>): Navigation => {
  const hist = history.create<View>({ component: componentMap[0] });

  return {
    current: hist.current,
    back: (): void => {
      hist.pop();
    },
    set: (key: Screen, props?: Props): void => {
      hist.push({
        component: componentMap[key],
        props,
      });
    },
  };
};
