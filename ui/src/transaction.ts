import * as api from "./api";
import { Error } from "./error";
import { emit } from "./event";
import * as message from "./message";
import { createStore } from "./remote";

export interface Transaction {
  id: string;
}

type Transactions = Transaction[];

const transactionsStore = createStore<Transactions>(
  () => emit({
      kind: message.Kind.Transaction,
      msg: { kind: Kind.FetchList, ids: [] }
    })
);

export const transactions = transactionsStore.readable;

export enum Kind {
  FetchList = "FETCH_LIST",
  ListFetched = "LIST_FETCHED",
}

interface MsgInterface {
  kind: Kind,
}

interface FetchList extends MsgInterface {
  kind: Kind.FetchList;
  ids: Array<String>;
}

interface ListFetched extends MsgInterface {
  kind: Kind.ListFetched;
  transactions: Transactions;
}

export type Msg = FetchList | ListFetched;

export function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.FetchList:
      Api.list({ ids: msg.ids });
      transactionsStore.loading();
      break;
    case Kind.ListFetched:
      transactionsStore.success(msg.transactions);
      break;
  }
}

interface ListInput {
  ids: Array<String>;
}

namespace Api {
  export function list(input: ListInput): void {
      api.post<Transactions>("transactions", input)
        .then((transactions: Transactions) => {
          transactionsStore.success(transactions);
        })
        .catch((err: Error) => {
          transactionsStore.error(err);
        })
  }
}
