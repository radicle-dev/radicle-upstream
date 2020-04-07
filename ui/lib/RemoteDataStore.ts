import { derived, writable, Readable, Writable } from 'svelte/store';

export enum RemoteDataStatus {
  NotAsked = 'NOT_ASKED',
  Loading = 'LOADING',
  Error = 'ERROR',
  Success = 'SUCCESS'
}

//TODO(sos): flesh out what errors should look like
type Error = string

export type RemoteData<T> =
  { status: RemoteDataStatus.NotAsked } |
  { status: RemoteDataStatus.Loading } |
  { status: RemoteDataStatus.Success, data: T } |
  { status: RemoteDataStatus.Error, error: Error }


// We should only be updating in this direction: NotAsked => Loading, Loading -> Success | Error
type UpdateableStatus = RemoteDataStatus.Loading | RemoteDataStatus.Success | RemoteDataStatus.Error

interface Update<T> {
  (status: RemoteDataStatus.Loading): void
  (status: RemoteDataStatus.Success, payload: T): void
  (status: RemoteDataStatus.Error, payload: Error): void
}

// A RemoteDataStore is a typesafe svelte readable store that exposes `updateStatus`
// and `update`. It's like a Writable but it can't be externally `set`, and 
// it only accepts data that conforms to the `RemoteData` interface
interface RemoteDataStore<T> extends Readable<RemoteData<T>> { // a Readable store of Remote Data based on type T
  update: Update<T>,
  readable: Readable<RemoteData<T>>
}

export const createRemoteDataStore = <T>(value: T, start?: (set: any) => void): RemoteDataStore<T> => {
  const initialState = { status: RemoteDataStatus.NotAsked } as RemoteData<T>
  const internalStore = writable(initialState, start)
  const { subscribe, update } = internalStore

  const modifiedUpdate: Update<T> = (status: UpdateableStatus, payload?: T | Error) => {
    let val: RemoteData<T>
    switch (status) {
      case RemoteDataStatus.Loading:
        val = { status: RemoteDataStatus.Loading }
        break
      case RemoteDataStatus.Success:
        val = { status: RemoteDataStatus.Success, data: payload as T }
        break
      case RemoteDataStatus.Error:
        val = { status: RemoteDataStatus.Error, error: payload as Error }
        break
    }

    update(() => { return val })
  }

  return {
    subscribe,
    update: modifiedUpdate,
    readable: derived(internalStore, $store => $store)
  }
}
