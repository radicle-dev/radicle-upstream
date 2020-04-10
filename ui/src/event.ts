export interface Event<K, M> {
  kind: K;
  msg?: M;
}

export function create<K, M>(kind: K, cb: (event: Event<K, M>) => void): (msg?: M) => void {
  return (msg?: M): void => { cb({ kind: kind, msg: msg }) }
}
