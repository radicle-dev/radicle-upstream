open Molecule.Alert;

type alert = {
  severity,
  message: string,
  id: int,
};

type action =
  | Add(alert)
  | Remove(alert);

type state = {
  idCounter: int,
  all: array(alert),
};

let initialState = {idCounter: 0, all: [||]};

let reducer = (state, action) =>
  switch (action) {
  | Add(alert) => {
      idCounter: state.idCounter + 1,
      all: Array.append(state.all, [|alert|]),
    }
  | Remove(alert) => {
      ...state,
      all: Belt.Array.keep(state.all, a => alert.id !== a.id),
    }
  };
