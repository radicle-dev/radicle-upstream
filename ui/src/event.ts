export type Event<K> = {
  kind: K;
}

type Msg = object | null;

declare type callback<K, M extends Event<K>> = (event: M) => void;
declare type call = (msg?: Msg) => void;

export function createEvent<K, M extends Event<K>>(kind: K, cb: callback<K, M>): call {
  return (msg?: Msg): void => { cb({ kind: kind, ...msg } as M) }
}
