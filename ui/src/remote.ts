import { derived, writable, Readable } from 'svelte/store';

import { Error } from "./error";

export enum Status {
  NotAsked = 'NOT_ASKED',
  Loading = 'LOADING',
  Error = 'ERROR',
  Success = 'SUCCESS'
}

export type Data<T> =
  { status: Status.NotAsked } |
  { status: Status.Loading } |
  { status: Status.Success; data: T } |
  { status: Status.Error; error: Error };


// A Store is a typesafe svelte readable store that exposes `updateStatus`
// and `update`. It's like a Writable but it can't be externally `set`, and 
// it only accepts data that conforms to the `RemoteData` interface
//
// a Readable store of Remote Data based on type T
export interface Store<T> extends Readable<Data<T>> {
  loading: () => void;
  success: (response: T) => void;
  error: (error: Error) => void;
  readable: Readable<Data<T>>;
}

// We should only be updating in this direction: NotAsked => Loading, Loading -> Success | Error
type UpdateableStatus = Status.Loading | Status.Success | Status.Error

interface Update<T> {
  (status: Status.Loading): void;
  (status: Status.Success, payload: T): void;
  (status: Status.Error, payload: Error): void;
}

// TODO(sos): add @param docs here, consider making generic type T required
export const createStore = <T>(
  start?: () => void
): Store<T> => {
  const initialState = { status: Status.NotAsked } as Data<T>
  const internalStore = writable(initialState, start)
  const { subscribe, update } = internalStore

  const updateInternalStore: Update<T> = (status: UpdateableStatus, payload?: T | Error) => {
    let val: Data<T>
    switch (status) {
      case Status.Loading:
        val = { status: Status.Loading }
        break;
      case Status.Success:
        val = { status: Status.Success, data: payload as T }
        break;
      case Status.Error:
        val = { status: Status.Error, error: payload as Error }
        break;
    }

    update(() => { return val })
  }

  return {
    subscribe,
    success: (response: T): void => updateInternalStore(
      Status.Success,
      response
    ),
    loading: (): void => updateInternalStore(Status.Loading),
    error: (error: Error): void => updateInternalStore(
      Status.Error,
      error
    ),
    readable: derived(internalStore, $store => $store)
  }
}
