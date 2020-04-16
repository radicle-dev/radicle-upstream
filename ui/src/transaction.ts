import * as api from "./api";
import { Event, createEvent } from "./event";
import * as remote from "./remote";

// Types.
export interface Transaction {
  id: string;
}

type Transactions = Transaction[];

const transactionsStore = remote.createStore<Transactions>();

export const transactions = transactionsStore.readable;

// Events.
export enum Kind {
  FetchList = "FETCH_LIST",
}

interface FetchList extends Event<Kind> {
  ids: Array<string>;
}

export type Msg = FetchList;

interface ListInput {
  ids: Array<string>;
}

export function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.FetchList:
      transactionsStore.loading();
      api.post<ListInput, Transactions>("transactions", { ids: msg.ids })
        .then(transactionsStore.success)
        .catch(transactionsStore.error);

      break;
  }
}

const fetchList = createEvent<Kind, Msg>(Kind.FetchList, update)

// Fetch initial list when the store has been subcribed to for the first time.
transactionsStore.start(() => { fetchList({ ids: [] }) });
