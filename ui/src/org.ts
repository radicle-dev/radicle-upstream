import { MessageType } from './transaction';

export enum RegistrationFlowState {
  NameSelection,
  TransactionConfirmation
}

const imageUrl =
  "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg";

export const transaction = {
  messages: [
    {
      type: MessageType.OrgRegistration,
      orgId: "1234"
    }
  ]
};

export const payer = {
  name: "someone",
  kind: "org",
  avatarFallback: null,
  imageUrl: imageUrl
};

export const subject = {
  name: "",
  kind: "org",
  avatarFallback: null,
  imageUrl: imageUrl
};
