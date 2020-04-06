import { derived, writable, Readable } from 'svelte/store';

// TODO(sos): uppercase enums
export enum RemoteDataStatus {
  NotAsked = 'NOT_ASKED',
  Loading = 'LOADING',
  Failure = 'FAILURE',
  Success = 'SUCCESS'
}

//TODO(sos): generics for data and error state
export interface RemoteData {
  status: RemoteDataStatus,
  data?: any
  error?: any
}

// export type RemoteData =
//   { status: RemoteDataStatus.NotAsked } |
//   { status: RemoteDataStatus.Success, data: {} } |
//   { status: RemoteDataStatus.Failure, error: any }

// A RemoteDataStore is a typesafe svelte readable store that exposes `updateStatus`
// and `update`. It's like a Writable but it can't be externally `set`, and 
// it only accepts data that conforms to the `RemoteData` interface
interface RemoteDataStore<T> extends Readable<T> {
  update: (updater: (value: T) => T) => void,
  updateStatus: (a: any) => any,
  readable: Readable<T>
}

export const createRemoteDataStore = <T extends RemoteData>(
  initialState: T,
  start?: (set: any) => void
): RemoteDataStore<T> => {
  const internalStore = writable(initialState, start)
  const { subscribe, update } = internalStore

  // TODO(sos): status should change every time state changes
  return {
    subscribe,
    update,
    updateStatus: (status: RemoteDataStatus) => update(
      store => {
        return { ...store, status: status }
      }),
    readable: derived(internalStore, $store => $store)
  }
}
