import { Readable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// Types.
export enum MessageType {
  OrgRegistration = "ORG_REGISTRATION",
  OrgUnregistration = "ORG_UNREGISTRATION",
  ProjectRegistration = "PROJECT_REGISTRATION",
  UserRegistration = "USER_REGISTRATION",
}

interface OrgRegistration {
  type: MessageType.OrgRegistration;
  orgId: string;
}

interface OrgUnregistration {
  type: MessageType.OrgUnregistration;
  orgId: string;
}

interface ProjectRegistration {
  type: MessageType.ProjectRegistration;
  orgId: string;
  projectName: string;
}

interface UserRegistration {
  type: MessageType.UserRegistration;
  handle: string;
  id: string;
}

type Message = OrgRegistration | OrgUnregistration | ProjectRegistration | UserRegistration;

enum StateType {
  Applied = "APPLIED",
}

interface Applied {
  type: StateType.Applied;
  blockHash: string;
}

type State = Applied;

export interface Transaction {
  id: string;
  messages: Message[];
  state: State;
}

type Transactions = Transaction[];

const transactionsStore = remote.createStore<Transactions>();

export const transactions = transactionsStore.readable;

// Events.
enum Kind {
  FetchList = "FETCH_LIST",
}

interface FetchList extends event.Event<Kind> {
  ids: Array<string>;
}

type Msg = FetchList;

interface ListInput {
  ids: Array<string>;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.FetchList:
      transactionsStore.loading();
      api.post<ListInput, Transactions>("transactions", { ids: msg.ids })
        .then(transactionsStore.success)
        .catch(transactionsStore.error);

      break;
  }
}

const fetchList = event.create<Kind, Msg>(Kind.FetchList, update)

export const fetch = (id: string): Readable<remote.Data<Transaction | null>> => {
  const store = remote.createStore<Transaction | null>();

  api.post<ListInput, Transactions>("transactions", { ids: [ id ] })
    .then(txs => store.success(txs.length === 1 ? txs[0] : null))
    .catch(store.error);

  return store;
}

// Fetch initial list when the store has been subcribed to for the first time.
transactionsStore.start(() => { fetchList({ ids: [] }) });

// FORMATTING
export const formatMessage = (msg: Message): string => {
  switch (msg.type) {
    case MessageType.OrgRegistration:
      return "Org registration";

    case MessageType.OrgUnregistration:
      return "Org unregistration";

    case MessageType.ProjectRegistration:
      return "Project registration";

    case MessageType.UserRegistration:
      return "User registration";
  }
};

// TODO(merle): Use actual data.
export const format = (tx: Transaction): object => {
  return {
    id: tx.id,
    message: formatMessage(tx.messages[0]),
    state: "pending",
    progress: 0
  }
}

export const formatStake = (msg: Message): string => `${formatMessage(msg)} deposit`;

export const formatPayer = (identity: identity.Identity): object => {
  return {
    name: identity.metadata.displayName || identity.metadata.handle,
    kind: "user",
    avatarFallback: identity.avatarFallback,
    imageUrl: identity.metadata.avatarUrl
  };
};

export const formatSubject = (identity: identity.Identity, msg: Message): object => {
  let name;

  switch (msg.type) {
    case MessageType.OrgRegistration:
      name = msg.orgId;
      break;

    case MessageType.OrgUnregistration:
      name = msg.orgId;
      break;

    case MessageType.UserRegistration:
      name = msg.handle;
      break;

    case MessageType.ProjectRegistration:
      name = `${identity.metadata.handle} / ${msg.projectName}`;
      break;
  }

  return {
    name,
    kind: "user",
    avatarFallback: identity.avatarFallback,
    imageUrl: identity.metadata.avatarUrl
  };
};
