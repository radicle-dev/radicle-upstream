import * as history from "./history";

export enum Screen {
  Blank,
}

const hist = history.create<Screen>(Screen.Blank);
export const store = hist.current;
