import { Readable } from "svelte/store";

import * as history from "./history";

export enum Screen {
  Blank,
  Help,
}

interface View {
  readonly component: any;
}

interface Navigation {
  readonly current: Readable<View>;
  back(): void;
  set(view: View): void;
}

export const create = (initial: View): Navigation => {
  const hist = history.create<View>(initial);

  return {
    current: hist.current,
    back: (): void => {
      hist.pop();
    },
    set: (view: View): void => {
      hist.push(view);
    },
  };
};
