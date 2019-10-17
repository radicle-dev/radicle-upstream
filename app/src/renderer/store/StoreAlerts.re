type severity =
  | Info
  | Success
  | Error;

type alert = {
  severity,
  message: string,
  id: int,
};

type action =
  | Add(alert)
  | Remove(alert);

type state = array(alert);

let initialState = [||];

let reducer = (state, action) =>
  switch (action) {
  | Add(alert) => Array.append(state, [|alert|])
  | Remove(alert) => Belt.Array.keep(state, a => alert.id !== a.id)
  };
