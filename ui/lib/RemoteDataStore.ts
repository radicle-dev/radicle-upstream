import { derived, writable, Readable } from 'svelte/store';

export enum RemoteDataStatus {
  NotAsked = 'NotAsked',
  Loading = 'Loading',
  Failure = 'Failure',
  Success = 'Success'
  // SuccessEmpty = 'SuccessEmpty'
}

export interface RemoteData {
  status: RemoteDataStatus,
  data: any
}

// A RemoteDataStore is a typesafe svelte readable store that exposes `updateStatus`
// and `update`. It's like a Writable but it can't be externally `set`, and 
// it only accepts data that conforms to the `RemoteData` interface
interface RemoteDataStore<T extends RemoteData> extends Readable<T> {
  update: (updater: (value: T) => T) => void,
  updateStatus: (a: any) => any
}

export const createRemoteDataStore = <T extends RemoteData>(
  initialState: T,
  start?: (set: any) => void
): RemoteDataStore<T> => {
  const { subscribe, update } = writable(initialState, start)

  return {
    subscribe,
    update,
    updateStatus: (status: RemoteDataStatus) => update(
      store => {
        return { ...store, status: status }
      })
  }
}

// The derived version is stricly readonly - this is what components should
// subscribe to
export const derivedStore = <T extends RemoteData>(remoteDataStore: RemoteDataStore<T>): Readable<T> =>
  derived(remoteDataStore, $remoteDataStore => $remoteDataStore) as Readable<T>
