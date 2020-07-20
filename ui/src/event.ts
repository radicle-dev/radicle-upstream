export type Event<K> = {
  kind: K;
};

type Msg = Record<string, unknown> | null;

declare type callback<K, M extends Event<K>> = (event: M) => void;
declare type call = (msg?: Msg) => void;

export function create<K, M extends Event<K>>(
  kind: K,
  cb: callback<K, M>
): call {
  return (msg?: Msg): void => {
    cb({ kind: kind, ...msg } as M);
  };
}
