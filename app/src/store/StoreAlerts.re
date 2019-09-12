open Molecule.Alert;

type alert = {
  severity,
  message: string,
};

type action =
  | Show(alert)
  | Remove(int);

type state = array(alert);

let initialState = [||];

let reducer = (state, action) =>
  switch (action) {
  | Show(alert) => Array.append(state, [|alert|])
  | Remove(index) => Belt.Array.keepWithIndex(state, (_, i) => index !== i)
  };
