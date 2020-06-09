import { Readable, Writable, get, derived, writable } from "svelte/store";
import * as timeago from "timeago.js";

import * as api from "./api";
import { Avatar, getAvatar, Usage, EmojiAvatar } from "./avatar"
import * as currency from "./currency";
import * as event from "./event";
import { Identity } from "./identity";
import { Domain } from "./project"
import * as remote from "./remote";

const POLL_INTERVAL = 10000;

// Types.
type Height = number;

interface Timestamp {
  secs: number;
  nanos: number;
}

export interface Costs {
  minimumFee: currency.MicroRad;
  userRegistrationDeposit: currency.MicroRad;
  orgRegistrationDeposit: currency.MicroRad;
  projectRegistrationDepoist: currency.MicroRad;
  memberRegistrationDeposit: currency.MicroRad;
}

export enum MessageType {
  OrgRegistration = "orgRegistration",
  OrgUnregistration = "orgUnregistration",
  MemberRegistration = "memberRegistration",
  MemberUnregistration = "memberUnregistration",
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

interface MemberRegistration {
  type: MessageType.MemberRegistration;
  orgId: string;
  handle: string;
}

interface MemberUnregistration {
  type: MessageType.MemberUnregistration;
  orgId: string;
  handle: string;
}

// TODO(sos): coordinate message format for project registration with proxy
interface ProjectRegistration {
  type: MessageType.ProjectRegistration;
  domain: Domain;
  orgId: string;
  // domainId: string; // domain under which project falls, e.g. User or Org
  cocoId: string;
  projectName: string;
}

interface UserRegistration {
  type: MessageType.UserRegistration;
  handle: string;
  id: string;
}

type Message
  = OrgRegistration
  | OrgUnregistration
  | MemberRegistration
  | MemberUnregistration
  | ProjectRegistration
  | UserRegistration;

export enum StateType {
  Confirmed = "confirmed",
  Failed = "failed",
  Pending = "pending",
  Settled = "settled",
}

interface Confirmed {
  type: StateType.Confirmed;
  block: Height;
  minConfirmations: number;
  confirmations: number;
  timestamp: Timestamp;
}

interface Failed {
  type: StateType.Failed;
  error: string;
  timestamp: Timestamp;
}

interface Pending {
  type: StateType.Pending;
  timestamp: Timestamp;
}

interface Settled {
  type: StateType.Settled;
  minConfirmations: number;
  timestamp: Timestamp;
}

type State = Confirmed | Failed | Pending | Settled;

export interface Transaction {
  id: string;
  messages: Message[];
  state: State;
  timestamp: Timestamp;
  fee: number;
}

type Transactions = Transaction[];

interface SummaryCounts {
  confirmed: number;
  failed: number;
  pending: number;
  settled: number;
  sum: number;
}

interface Summary {
  confirmations: number;
  minConfirmations: number;
  counts: SummaryCounts;
}

export const summarizeTransactions = (txs: Transactions): Summary =>
  txs.reduce((acc, tx): Summary => {
    acc.counts.sum += 1;
    acc.counts[tx.state.type] += 1;

    if (tx.state.type === StateType.Confirmed) {
      acc.confirmations += tx.state.confirmations;
      acc.minConfirmations = tx.state.minConfirmations;
    } else if (tx.state.type === StateType.Settled) {
      acc.confirmations += tx.state.minConfirmations;
      acc.minConfirmations = tx.state.minConfirmations;
    }

    return acc;
  }, {
    confirmations: 0,
    minConfirmations: 0,
    counts: {
      confirmed: 0,
      failed: 0,
      pending: 0,
      settled: 0,
      sum: 0,
    },
  });

const transactionsStore = remote.createStore<Transactions>();
export const transactions = transactionsStore.readable;

const summaryStore: Writable<Summary | null> = writable(null);
export const summary = derived(transactionsStore, (store) => {
  if (store.status === remote.Status.Success) {
    const updated = summarizeTransactions(store.data)

    summaryStore.set(updated);

    return updated;
  }

  return get(summaryStore);
});

// Events.
enum Kind {
  FetchList = "FETCH_LIST",
  RefetchList = "REFETCH_LIST",
}

interface FetchList extends event.Event<Kind> {
  kind: Kind.FetchList;
  ids: Array<string>;
}

interface RefetchList extends event.Event<Kind> {
  kind: Kind.RefetchList;
}

type Msg = FetchList | RefetchList;

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

    case Kind.RefetchList:
      api.post<ListInput, Transactions>("transactions", { ids: [] })
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
export const refetchList = event.create<Kind, Msg>(Kind.RefetchList, update);

// Fetch initial list when the store has been subcribed to for the first time.
transactionsStore.start(() => {
  const poll = setInterval(() => refetchList(), POLL_INTERVAL);
  fetchList();

  return (): void => clearInterval(poll);
});

// FORMATTING
export const formatMessage = (msg: Message): string => {
  switch (msg.type) {
    case MessageType.OrgRegistration:
      return "Org registration";

    case MessageType.OrgUnregistration:
      return "Org unregistration";

    case MessageType.MemberRegistration:
      return "Org member registration"

    case MessageType.MemberUnregistration:
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

// TODO(sos): coordinate payer shape with proxy/registry
interface Payer {
  type: PayerType;
  name: string;
  avatarFallback: EmojiAvatar;
  imageUrl?: string;
}

export const formatPayer = (identity: Identity): Payer => (identity && {
  name: identity.metadata.displayName || identity.metadata.handle,
  type: PayerType.User,
  avatarFallback: identity.avatarFallback,
  imageUrl: identity.metadata.avatarUrl
});

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
    case MessageType.MemberRegistration:
      name = msg.handle;
      type = SubjectType.Member
      // avatarSource = getAvatar(Usage.Identity, msg.handle)
      break;

    case MessageType.MemberUnregistration:
      name = msg.handle;
      type = SubjectType.Member
      // avatarSource = getAvatar(Usage.Identity, msg.handle)
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
      name = `${msg.orgId} / ${msg.projectName}`
      type = SubjectType.OrgProject
      avatarSource = getAvatar(msg.domain === Domain.User ? Usage.Identity : Usage.Org, msg.orgId)
      break;
  }

  return {
    name,
    type,
    avatarSource
  }
}

export const iconProgress = (state: State): number => {
  switch (state.type) {
    case StateType.Confirmed:
      return state.confirmations / state.minConfirmations * 100;
    case StateType.Settled:
      return 100;
    default:
      return 0;
  }
}

export enum IconState {
  Caution = "caution",
  Negative = "negative",
  Positive = "positive",
}

export const iconState = (state: State): IconState => {
  switch (state.type) {
    case StateType.Failed:
      return IconState.Negative;
    case StateType.Settled:
      return IconState.Positive;
    default:
      return IconState.Caution;
  }
}

export const statusText = (state: State): string => {
  const timestamp = timeago.format(state.timestamp.secs * 1000);

  switch (state.type) {
    case StateType.Confirmed:
      return `Waiting for transaction to settle`;

    case StateType.Failed:
      return `Transaction failed ${timestamp}`;

    case StateType.Pending:
      return `Waiting for confirmation`;

    case StateType.Settled:
      return `Transaction settled ${timestamp}`;
  }
}

export const summaryIconProgress = (summary: Summary): number => {
  const sum = summary.counts[StateType.Confirmed] + summary.counts[StateType.Settled];
  if (sum === 0) { return 0; }

  const progress = summary.confirmations / (summary.minConfirmations * sum);

  return progress !== 0 ? progress * 100 : 15;
}

export const summaryIconRotate = (counts: SummaryCounts): boolean => {
  return (counts.failed > 0 && counts.pending > 0) && (counts.confirmed === 0 && counts.settled === 0);
}

export const summaryIconState = (counts: SummaryCounts): IconState => {
  if (counts.failed > 0) {
    return IconState.Negative;
  } else if (counts.confirmed > 0 || counts.pending > 0) {
    return IconState.Caution;
  }

  return IconState.Positive;
}

export const summaryText = (counts: SummaryCounts): string => {
  let sum = 0;
  let state = StateType.Settled;

  if (counts[StateType.Settled] > 0) {
    sum = counts[StateType.Settled];
  }
  if (counts[StateType.Failed] > 0) {
    sum = counts[StateType.Failed];
    state = StateType.Failed;
  }
  if (counts[StateType.Confirmed] > 0 || counts[StateType.Pending] > 0) {
    sum = counts[StateType.Confirmed] + counts[StateType.Pending];
    state = StateType.Pending
  }

  if (sum > 1) {
    return `${sum} transactions ${state}`;
  }

  return `transaction ${state}`;
}

interface CostSummary {
  depositRad: currency.Rad;
  depositUsd: currency.Usd;
  feeRad: currency.Rad;
  feeUsd: currency.Usd;
  totalRad: currency.Rad;
  totalUsd: currency.Usd;
}

export const costSummary = (
  messageType: MessageType,
  fee: currency.MicroRad,
  costs: Costs): CostSummary => {

  let deposit = 0;

  switch (messageType) {
    case MessageType.OrgRegistration:
      deposit = costs.orgRegistrationDeposit;
      break;
    case MessageType.MemberRegistration:
      deposit = costs.memberRegistrationDeposit;
      break;
    case MessageType.ProjectRegistration:
      deposit = costs.projectRegistrationDepoist;
      break;
    case MessageType.UserRegistration:
      deposit = costs.userRegistrationDeposit;
      break;
    default:
      throw(`MessageType: ${messageType} not implemented`)
      break;
  }

  const total = deposit + fee;

  const depositRad = currency.microRadToRad(deposit);
  const feeRad = currency.microRadToRad(fee);
  const totalRad = currency.microRadToRad(total);

  const depositUsd = currency.radToUsd(depositRad)
  const feeUsd = currency.radToUsd(feeRad)
  const totalUsd = currency.radToUsd(totalRad)

  return {
    depositRad,
    depositUsd,
    feeRad,
    feeUsd,
    totalRad,
    totalUsd,
  }
}
