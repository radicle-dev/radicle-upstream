import * as api from "./api";
import * as avatar from "./avatar";
import * as currency from "./currency";
import * as event from "./event";
import * as project from "./project";
import * as remote from "./remote";
import * as validation from "./validation";
import * as transaction from "./transaction";
import * as user from "./user";

// Types
export interface Org {
  id: string;
  shareableEntityIdentifier: string;
  avatarFallback: avatar.EmojiAvatar;
  members: [user.User];
}

export interface Project {
  name: string;
  orgId: string;
  shareableEntityIdentifier: string;
  maybeProject: project.Project;
}

type Projects = Project[];

export enum RegistrationFlowState {
  Preparation,
  Confirmation,
}

// State
const orgStore = remote.createStore<Org>();
export const org = orgStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

// Api
export const getOrg = (id: string): Promise<Org> => api.get<Org>(`orgs/${id}`);

// Check if the given id is already taken
export const isIdTaken = (id: string): Promise<boolean> =>
  getOrg(id).then(org => !!org || user.get(id).then(user => !!user));

// Check if the given id is available
const isIdAvailable = (id: string): Promise<boolean> =>
  isIdTaken(id).then(taken => !taken);

const validateUserExistence = (handle: string): Promise<boolean> =>
  user.get(handle).then(user => !!user);

const validateNewMember = (orgId: string) => (
  handle: string
): Promise<boolean> =>
  getOrg(orgId).then(
    org => !org.members.find(member => member.handle == handle)
  );

// Events
enum Kind {
  Fetch = "FETCH",
  FetchProjectList = "FETCH_PROJECT_LIST",
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

interface FetchProjectList extends event.Event<Kind> {
  kind: Kind.FetchProjectList;
  id: string;
}

type Msg = Fetch | FetchProjectList;

interface RegisterInput {
  id: string;
  transactionFee: currency.MicroRad;
}

interface RegisterMemberInput {
  handle: string;
  transactionFee: currency.MicroRad;
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Fetch:
      orgStore.loading();
      api
        .get<Org>(`orgs/${msg.id}`)
        .then(orgStore.success)
        .catch(orgStore.error);

      break;
    case Kind.FetchProjectList:
      projectsStore.loading();
      api
        .get<Projects>(`orgs/${msg.id}/projects`)
        .then(projectsStore.success)
        .catch(projectsStore.error);

      break;
  }
};

export const registerMemberTransaction = (
  orgId: string,
  handle: string,
  fee: currency.MicroRad
) => ({
  fee,
  messages: [
    {
      type: transaction.MessageType.MemberRegistration,
      orgId,
      handle,
    },
  ],
  state: {
    type: transaction.StateType.Confirmed,
    block: 1,
    confirmations: 2,
    minConfirmations: 6,
    timestamp: {
      secs: 1,
      nanos: 1,
    },
  },
  timestamp: {
    secs: 1,
    nanos: 1,
  },
});

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchProjectList = event.create<Kind, Msg>(
  Kind.FetchProjectList,
  update
);
export const register = (
  id: string,
  transactionFee: currency.MicroRad
): Promise<transaction.Transaction> =>
  api.post<RegisterInput, transaction.Transaction>(`orgs`, {
    id,
    transactionFee,
  });
export const registerMember = (
  orgId: string,
  handle: string,
  transactionFee: currency.MicroRad
): Promise<transaction.Transaction> =>
  api.post<RegisterMemberInput, transaction.Transaction>(
    `orgs/${orgId}/members`,
    { handle, transactionFee }
  );

// ID validation
const VALID_ID_MATCH = new RegExp("^[a-z0-9][a-z0-9]+$");
export const idConstraints = {
  presence: {
    message: `Org id is required`,
    allowEmpty: false,
  },
  format: {
    pattern: VALID_ID_MATCH,
    message: `Org id should match [a-z0-9][a-z0-9_-]+`,
  },
};

// Make sure we make a new one every time
export const orgIdValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(idConstraints, [
    {
      promise: isIdAvailable,
      validationMessage: "Sorry, this id is already taken",
    },
  ]);

const memberHandleConstraints = {
  presence: {
    message: "Member handle is required",
    allowEmpty: false,
  },
};

export const memberHandleValidationStore = (
  orgId: string
): validation.ValidationStore => {
  return validation.createValidationStore(memberHandleConstraints, [
    {
      promise: validateUserExistence,
      validationMessage: "Cannot find this user",
    },
    {
      promise: validateNewMember(orgId),
      validationMessage: "This user is already a member",
    },
  ]);
};
