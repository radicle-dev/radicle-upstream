import * as api from "./api";
import * as avatar from "./avatar";
import * as event from "./event";
import * as project from "./project";
import * as remote from "./remote";
import * as validation from "./validation";
import * as transaction from "./transaction";
import * as user from "./user";

// Types
export interface Org {
  name: string;
  shareableEntityIdentifier: string;
  avatarFallback: avatar.EmojiAvatar;
}

export interface Project {
  name: string;
  orgId: string;
  shareableEntityIdentifier: string;
  maybeProject: project.Project;
}

type Projects = Project[];

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

// State
const orgStore = remote.createStore<Org>();
export const org = orgStore.readable;

const projectsStore = remote.createStore<Projects>();
export const projects = projectsStore.readable;

// Api
export const getOrg = (id: string): Promise<Org> => api.get<Org>(`orgs/${id}`);
export const getNameAvailability = (id: string): Promise<boolean> =>
  getOrg(id).then(org => !org);
const validateUserExistence = (handle: string): Promise<boolean> =>
  user.get(handle).then(user => !!user);

// Events
enum Kind {
  Fetch = "FETCH",
  FetchProjectList = "FETCH_PROJECT_LIST"
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
  userId: string
): transaction.Transaction => ({
  id: "",
  messages: [
    {
      type: transaction.MessageType.OrgMemberRegistration,
      orgId,
      userId
    }
  ],
  state: { type: transaction.StateType.Applied, blockHash: "0x000" }
});

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchProjectList = event.create<Kind, Msg>(
  Kind.FetchProjectList,
  update
);
export const register = (id: string): Promise<transaction.Transaction> =>
  api.post<RegisterInput, transaction.Transaction>(`orgs`, { id });

// Name validation
const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9]+$");
export const nameConstraints = {
  presence: {
    message: `Org name is required`,
    allowEmpty: false
  },
  format: {
    pattern: VALID_NAME_MATCH,
    message: `Org name should match [a-z0-9][a-z0-9_-]+`
  }
};

// Make sure we make a new one every time
export const orgNameValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(nameConstraints, {
    promise: getNameAvailability,
    validationMessage: "Sorry, this name is already taken"
  });

const memberNameConstraints = {
  presence: {
    message: "Member name is required",
    allowEmpty: false
  }
};

export const memberNameValidationStore = (): validation.ValidationStore =>
  validation.createValidationStore(memberNameConstraints, {
    promise: validateUserExistence,
    validationMessage: "Cannot find this user"
  });
