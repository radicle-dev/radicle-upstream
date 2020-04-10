import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// Types.
export interface Transaction {
  id: string;
}

type Transactions = Transaction[];

const transactionsStore = remote.createStore<Transactions>(
  () => fetchList({ ids: [] })
);

export const transactions = transactionsStore.readable;

// Events.
export enum Kind {
  FetchList = "FETCH_LIST",
}

interface FetchList {
  ids: Array<string>;
}

export type Msg = FetchList;

interface ListInput {
  ids: Array<string>;
}

export function update(event: event.Event<Kind, Msg>): void {
  switch (event.kind) {
    case Kind.FetchList:
      transactionsStore.loading();
      api.post<ListInput, Transactions>("transactions", { ids: event.msg!.ids })
        .then(transactionsStore.success)
        .catch(transactionsStore.error);

      break;
  }
}

const fetchList = event.create<Kind, Msg>(Kind.FetchList, update)
