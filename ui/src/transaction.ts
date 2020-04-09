import * as api from "./api";
import { emit } from "./event";
import * as message from "./message";
import { createStore } from "./remote";

export interface Transaction {
  id: string;
}

type Transactions = Transaction[];

export enum Kind {
  FetchList = "FETCH_LIST",
  ListFetched = "LIST_FETCHED",
}

interface MsgInterface {
  kind: Kind;
}

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
  ids: Array<string>;
}

interface ListFetched extends MsgInterface {
  kind: Kind.ListFetched;
  transactions: Transactions;
}

export type Msg = FetchList | ListFetched;

const transactionsStore = createStore<Transactions>(
  () => emit({
      kind: message.Kind.Transaction,
      msg: { kind: Kind.FetchList, ids: [] }
    })
);

export const transactions = transactionsStore.readable;

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
