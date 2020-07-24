import { Readable, Writable, get, derived, writable } from "svelte/store";
import * as timeago from "timeago.js";

import * as api from "./api";
import { Avatar, getAvatar, Usage, EmojiAvatar } from "./avatar";
import * as currency from "./currency";
import * as event from "./event";
import { Identity } from "./identity";
import { Domain } from "./project";
import * as remote from "./remote";

const POLL_INTERVAL = 10000;

// Types.
type Height = number;

interface Timestamp {
  secs: number;
  nanos: number;
}

// Note: The schemas of each variant must correspond to
// their proxy > registry > Message variant counterpart.
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
  domainType: Domain;
  domainId: string;
  projectName: string;
  cocoId: string;
}

interface UserRegistration {
  type: MessageType.UserRegistration;
  handle: string;
  id: string;
}

type Message =
  | OrgRegistration
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
  fee: currency.MicroRad;
  registrationFee?: currency.MicroRad;
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
  txs.reduce(
    (acc, tx): Summary => {
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
    },
    {
      confirmations: 0,
      minConfirmations: 0,
      counts: {
        confirmed: 0,
        failed: 0,
        pending: 0,
        settled: 0,
        sum: 0,
      },
    }
  );

const transactionsStore = remote.createStore<Transactions>();
export const transactions = transactionsStore.readable;

const summaryStore: Writable<Summary | null> = writable(null);
export const summary = derived(transactionsStore, store => {
  if (store.status === remote.Status.Success) {
    const updated = summarizeTransactions(store.data);

    summaryStore.set(updated);

    return updated;
  }

  // eslint-disable-next-line @typescript-eslint/no-unsafe-return
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
      api
        .post<ListInput, Transactions>("transactions", { ids: msg.ids })
        .then(transactionsStore.success)
        .catch(transactionsStore.error);

      break;

    case Kind.RefetchList:
      api
        .post<ListInput, Transactions>("transactions", { ids: [] })
        .then(transactionsStore.success)
        .catch(transactionsStore.error);

      break;
  }
};

export const fetchList = (ids?: Array<string>): void =>
  event.create<Kind, Msg>(Kind.FetchList, update)({ ids: ids || [] });

export const fetch = (
  id: string
): Readable<remote.Data<Transaction | null>> => {
  const store = remote.createStore<Transaction | null>();

  api
    .post<ListInput, Transactions>("transactions", { ids: [id] })
    .then(txs => store.success(txs.length === 1 ? txs[0] : null))
    .catch(store.error);

  return store;
};
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
      return "Member registration";

    case MessageType.MemberUnregistration:
      return "Member unregistration";

    case MessageType.ProjectRegistration:
      return "Project registration";

    case MessageType.UserRegistration:
      return "Handle registration";
  }
};

export const formatDesc = (msg: Message): string => {
  switch (msg.type) {
    case MessageType.OrgRegistration:
    case MessageType.OrgUnregistration:
      return msg.id;

    case MessageType.MemberRegistration:
    case MessageType.MemberUnregistration:
      return msg.handle;

    case MessageType.ProjectRegistration:
      return msg.domainId;

    case MessageType.UserRegistration:
      return msg.handle;
  }
};

export const headerIcon = (msg: Message): string => {
  switch (msg.type) {
    case MessageType.OrgRegistration:
    case MessageType.OrgUnregistration:
      return "Register";

    case MessageType.MemberRegistration:
    case MessageType.MemberUnregistration:
    case MessageType.UserRegistration:
      return "Member";

    case MessageType.ProjectRegistration:
      return "Source";
  }
};

export const formatStake = (msg: Message): string =>
  `${formatMessage(msg)} fee`;

// Having both enums & interfaces here is somewhat verbose; the reason we do this
// is so we have compatibility with non-TS svelte components while still enjoying
// some type strictness
export enum PayerType {
  Org = "org",
  User = "user",
}

// TODO(sos): coordinate payer shape with proxy/registry
interface Payer {
  type: PayerType;
  name: string;
  avatarFallback: EmojiAvatar;
  imageUrl?: string;
}

export const formatPayer = (identity: Identity): Payer =>
  identity && {
    name: identity.metadata.handle,
    type: PayerType.User,
    avatarFallback: identity.avatarFallback,
  };

export enum SubjectType {
  User = "user",
  OrgProject = "org_project",
  UserProject = "user_project",
  Org = "org",
  Member = "member",
}

interface Subject {
  name: string;
  type: SubjectType;
  avatarSource?: Promise<Avatar>;
}

export const formatSubject = (msg: Message): Subject => {
  let avatarSource, name, type;

  switch (msg.type) {
    case MessageType.OrgRegistration:
      name = msg.id;
      type = SubjectType.Org;
      avatarSource = getAvatar(Usage.Org, msg.id);
      break;

    case MessageType.OrgUnregistration:
      name = msg.id;
      type = SubjectType.Org;
      avatarSource = getAvatar(Usage.Org, msg.id);
      break;

    // TODO(sos): replace with actual avatar lookup for the identity associated with
    // the member, should it exist
    case MessageType.MemberRegistration:
      name = msg.handle;
      type = SubjectType.Member;
      // avatarSource = getAvatar(Usage.Identity, msg.handle)
      break;

    case MessageType.MemberUnregistration:
      name = msg.handle;
      type = SubjectType.Member;
      // avatarSource = getAvatar(Usage.Identity, msg.handle)
      break;

    // TODO(sos): replace with actual avatar lookup for the identity associated with
    // the user, should it exist
    case MessageType.UserRegistration:
      name = msg.handle;
      type = SubjectType.User;
      avatarSource = getAvatar(Usage.Identity, msg.id);
      break;

    // TODO(sos): replace with associated identity handle for user, should it exist
    // TODO(sos): once we can register projects to users, accommodate circle avatars
    case MessageType.ProjectRegistration:
      name = `${msg.domainId} / ${msg.projectName}`;
      type = SubjectType.OrgProject;
      avatarSource = getAvatar(
        msg.domainType === Domain.User ? Usage.Identity : Usage.Org,
        msg.domainId
      );
      break;
  }

  return {
    name,
    type,
    avatarSource,
  };
};

export const subjectAvatarShape = (subjectType: SubjectType): string => {
  switch (subjectType) {
    case SubjectType.User:
    case SubjectType.Member:
    case SubjectType.UserProject:
      return "circle";
    default:
      return "square";
  }
};

export const iconProgress = (state: State): number => {
  switch (state.type) {
    case StateType.Confirmed:
      return (state.confirmations / state.minConfirmations) * 100;
    case StateType.Settled:
      return 100;
    default:
      return 0;
  }
};

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
};

export const statusText = (state: State): string => {
  const timestamp = timeago.format(state.timestamp.secs * 1000);

  switch (state.type) {
    case StateType.Confirmed:
      return `In progress`;

    case StateType.Failed:
      return `Transaction failed ${timestamp}`;

    case StateType.Pending:
      return `Waiting for confirmation`;

    case StateType.Settled:
      return `Settled ${timestamp}`;
  }
};

export const timestamp = (state: State): string => {
  const timestamp = new Date(state.timestamp.secs * 1000);
  const options = {
    year: "numeric",
    month: "long",
    day: "numeric",
  };
  return `${timestamp.toLocaleTimeString(undefined, options)}`;
};

export const formatDate = (timestamp: number, option: string): string => {
  const time = new Date(timestamp * 1000);
  const day = {
    day: "numeric",
  };
  const month = {
    month: "long",
  };

  const options = option === "day" ? day : month;
  return `${time.toLocaleString(undefined, options)}`;
};

export const summaryIconProgress = (summary: Summary): number => {
  const sum =
    summary.counts[StateType.Confirmed] + summary.counts[StateType.Settled];
  if (sum === 0) {
    return 0;
  }

  const progress = summary.confirmations / (summary.minConfirmations * sum);

  return progress !== 0 ? progress * 100 : 15;
};

export const summaryIconRotate = (counts: SummaryCounts): boolean => {
  return (
    counts.failed > 0 &&
    counts.pending > 0 &&
    counts.confirmed === 0 &&
    counts.settled === 0
  );
};

export const summaryIconState = (counts: SummaryCounts): IconState => {
  if (counts.failed > 0) {
    return IconState.Negative;
  } else if (counts.confirmed > 0 || counts.pending > 0) {
    return IconState.Caution;
  }

  return IconState.Positive;
};

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
    state = StateType.Pending;
  }

  if (sum > 1) {
    return `${sum} transactions ${state}`;
  }

  return `Transaction ${state}`;
};

interface CostSummary {
  registrationFee?: FeeAmount;
  txFee: FeeAmount;
  total: FeeAmount;
}

const paysRegistrationFee = (messageType: MessageType): boolean => {
  return (
    messageType === MessageType.OrgRegistration ||
    messageType === MessageType.UserRegistration
  );
};

interface FeeAmount {
  rad: currency.Rad;
  usd: currency.Usd;
}

const feeAmount = (microRad: currency.MicroRad): FeeAmount => {
  return {
    rad: currency.microRadToRad(microRad),
    usd: currency.microRadToUsd(microRad),
  };
};

export const costSummary = (transaction: Transaction): CostSummary => {
  const registrationFee: FeeAmount | undefined = transaction.registrationFee
    ? feeAmount(transaction.registrationFee)
    : undefined;
  const txFee = feeAmount(transaction.fee);
  const totalMicroRad =
    transaction.fee * 1 +
    (transaction.registrationFee ? transaction.registrationFee * 1 : 0);
  const total = feeAmount(totalMicroRad);

  return {
    registrationFee,
    txFee,
    total,
  };
};
