import { derived, writable, Readable } from 'svelte/store';

export enum RemoteDataStatus {
  NotAsked = 'NOT_ASKED',
  Loading = 'LOADING',
  Error = 'ERROR',
  Success = 'SUCCESS'
}

//TODO(sos): flesh out what errors should look like; consumers should define them
type Error = string

export type RemoteData<T> =
  { status: RemoteDataStatus.NotAsked } |
  { status: RemoteDataStatus.Loading } |
  { status: RemoteDataStatus.Success, data: T } |
  { status: RemoteDataStatus.Error, error: Error }


// A RemoteDataStore is a typesafe svelte readable store that exposes `updateStatus`
// and `update`. It's like a Writable but it can't be externally `set`, and 
// it only accepts data that conforms to the `RemoteData` interface
interface RemoteDataStore<T> extends Readable<RemoteData<T>> { // a Readable store of Remote Data based on type T
  loading: () => void,
  success: (response: T) => void,
  error: (error: Error) => void,
  readable: Readable<RemoteData<T>>
}

// We should only be updating in this direction: NotAsked => Loading, Loading -> Success | Error
type UpdateableStatus = RemoteDataStatus.Loading | RemoteDataStatus.Success | RemoteDataStatus.Error

interface Update<T> {
  (status: RemoteDataStatus.Loading): void;
  (status: RemoteDataStatus.Success, payload: T): void;
  (status: RemoteDataStatus.Error, payload: Error): void;
}

// TODO(sos): add @param docs here, consider making generic type T required
export const createRemoteDataStore = <T>(
  start?: () => void
): RemoteDataStore<T> => {
  const initialState = { status: RemoteDataStatus.NotAsked } as RemoteData<T>
  const internalStore = writable(initialState, start)
  const { subscribe, update } = internalStore

  const updateInternalStore: Update<T> = (status: UpdateableStatus, payload?: T | Error) => {
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
    success: (response: T) => updateInternalStore(
      RemoteDataStatus.Success,
      response
    ),
    loading: () => updateInternalStore(RemoteDataStatus.Loading),
    error: (error: Error) => updateInternalStore(
      RemoteDataStatus.Error,
      error
    ),
    readable: derived(internalStore, $store => $store)
  }
}
