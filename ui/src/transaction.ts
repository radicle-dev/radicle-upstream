import { Readable } from "svelte/store";

import * as api from "./api";
import { Avatar, getAvatar, Usage, EmojiAvatar } from "./avatar"
import * as event from "./event";
import * as remote from "./remote";
import { Identity } from "./identity";
import { Domain } from "./project"

// Types.
export enum MessageType {
  OrgRegistration = "orgRegistration",
  OrgUnregistration = "orgUnregistration",
  OrgMemberRegistration = "orgMemberRegistration",
  OrgMemberUnregistration = "orgMemberUnregistration",
  ProjectRegistration = "projectRegistration",
  UserRegistration = "userRegistration",
}

interface OrgRegistration {
  type: MessageType.OrgRegistration;
  id: string;
}

interface OrgUnregistration {
  type: MessageType.OrgUnregistration;
  id: string;
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

// TODO(sos): coordinate message format for project registration with proxy
interface ProjectRegistration {
  type: MessageType.ProjectRegistration;
  domain: Domain;
  org_id: string;
  project_name: string;
  // domainId: string; // domain under which project falls, e.g. User or Org
  cocoId: string;
  projectName: string;
  projectDescription: string;
}

interface UserRegistration {
  type: MessageType.UserRegistration;
  handle: string;
  id: string;
}

type Message
  = OrgRegistration
  | OrgUnregistration
  | OrgMemberRegistration
  | OrgMemberUnregistration
  | ProjectRegistration
  | UserRegistration;

export enum StateType {
  Applied = "applied",
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

export const fetchList = (ids?: Array<string>): void =>
  event.create<Kind, Msg>(Kind.FetchList, update)({ ids: ids || [] });

export const fetch = (id: string): Readable<remote.Data<Transaction | null>> => {
  const store = remote.createStore<Transaction | null>();

  api.post<ListInput, Transactions>("transactions", { ids: [id] })
    .then(txs => store.success(txs.length === 1 ? txs[0] : null))
    .catch(store.error);

  return store;
}

// Fetch initial list when the store has been subcribed to for the first time.
transactionsStore.start(fetchList);

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


// Having both enums & interfaces here is somewhat verbose; the reason we do this 
// is so we have compatibility with non-TS svelte components while still enjoying 
// some type strictness
export enum PayerType {
  Org = "org",
  User = "user"
}

// TODO(sos): update to support orgs
export const formatPayer = (identity: Identity): object => {
  return {
    name: identity.metadata.displayName || identity.metadata.handle,
    type: PayerType.User,
    avatarFallback: identity.avatarFallback,
    imageUrl: identity.metadata.avatarUrl
  };
};

export enum SubjectType {
  User = "user",
  OrgProject = "org_project",
  UserProject = "user_project",
  Org = "org",
  Member = "member"
}

interface Subject {
  name: string;
  type: SubjectType;
  avatarSource?: Promise<Avatar>;
}

export const formatSubject = (msg: Message): Subject => {
  let avatarSource, name, type

  switch (msg.type) {
    case MessageType.OrgRegistration:
      name = msg.id;
      type = SubjectType.Org
      avatarSource = getAvatar(Usage.Org, msg.id)
      break;

    case MessageType.OrgUnregistration:
      name = msg.id;
      type = SubjectType.Org
      avatarSource = getAvatar(Usage.Org, msg.id)
      break;

    // TODO(sos): replace with actual avatar lookup for the identity associated with
    // the member, should it exist
    case MessageType.OrgMemberRegistration:
      name = msg.userId;
      type = SubjectType.Member
      // avatarSource = getAvatar(Usage.Identity, msg.userId)
      break;

    case MessageType.OrgMemberUnregistration:
      name = msg.userId;
      type = SubjectType.Member
      // avatarSource = getAvatar(Usage.Identity, msg.userId)
      break;

    // TODO(sos): replace with actual avatar lookup for the identity associated with
    // the user, should it exist
    case MessageType.UserRegistration:
      name = msg.handle;
      type = SubjectType.User
      avatarSource = getAvatar(Usage.Identity, msg.id)
      break;

    // TODO(sos): replace with associated identity handle for user, should it exist
    // TODO(sos): once we can register projects to users, accommodate circle avatars
    case MessageType.ProjectRegistration:
      name = `${msg.org_id} / ${msg.project_name}`
      type = SubjectType.OrgProject
      avatarSource = getAvatar(msg.domain === Domain.User ? Usage.Identity : Usage.Org, msg.org_id)
      break;
  }

  return {
    name,
    type,
    avatarSource
  }
}
