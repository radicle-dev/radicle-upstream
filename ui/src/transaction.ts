import { Readable } from "svelte/store";

import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";
import { Identity } from "./identity";

// Types.
export enum MessageType {
  OrgRegistration = "ORG_REGISTRATION",
  OrgUnregistration = "ORG_UNREGISTRATION",
  OrgMemberRegistration = "ORG_MEMBER_REGISTRATION",
  OrgMemberUnregistration = "ORG_MEMBER_UNREGISTRATION",
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

interface OrgMemberRegistration {
  type: MessageType.OrgMemberRegistration;
  orgId: string;
  userId: string;
}

interface OrgMemberUnregistration {
  type: MessageType.OrgMemberUnregistration;
  orgId: string;
  userId: string;
}

interface ProjectRegistration {
  type: MessageType.ProjectRegistration;
  orgId: string;
  cocoId: string;
  projectName: string;
  projectDescription: string;
}

interface UserRegistration {
  type: MessageType.UserRegistration;
  handle: string;
  id: string;
}

type Message = OrgRegistration | OrgUnregistration | OrgMemberRegistration | OrgMemberUnregistration | ProjectRegistration | UserRegistration;

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

  api.post<ListInput, Transactions>("transactions", { ids: [id] })
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

    case MessageType.OrgMemberRegistration:
      return "Org member registration"

    case MessageType.OrgMemberUnregistration:
      return "Org member unregistration"

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

export enum Variant {
  Org = "ORG",
  User = "USER",
  Project = "PROJECT"
}

export const formatPayer = (identity: Identity): object => {
  return {
    name: identity.metadata.displayName || identity.metadata.handle,
    variant: Variant.User,
    avatarFallback: identity.avatarFallback,
    imageUrl: identity.metadata.avatarUrl
  };
};

// TODO(sos): add avatarFallback for org registration once endpoint is ready
export const formatSubject = (identity: Identity, msg: Message): object => {
  let name, variant;

  switch (msg.type) {
    case MessageType.OrgRegistration:
      name = msg.orgId;
      variant = Variant.Org
      break;

    case MessageType.OrgUnregistration:
      name = msg.orgId;
      variant = Variant.Org
      break;

    case MessageType.UserRegistration:
      name = msg.handle;
      variant = Variant.User
      break;

    case MessageType.ProjectRegistration:
      name = `${identity.metadata.handle} / ${msg.projectName}`;
      variant = Variant.Project
      break;
  }

  return {
    name,
    variant,
    avatarFallback: identity.avatarFallback,
    imageUrl: identity.metadata.avatarUrl
  };
};
