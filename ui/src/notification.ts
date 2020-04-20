import * as transaction from './transaction'

export enum Kind {
  Transaction,
}

interface MsgInterface {
  kind: Kind;
}

interface Transaction extends MsgInterface {
  kind: Kind.Transaction;
  transaction: transaction.Transaction;
}

export type Msg = Transaction

export type State = {}

export function init(): State {
  return {}
}

export function update(state: State, msg: Msg): State {
  switch (msg.kind) {
    case Kind.Transaction:
      break
  }

  return state
}
