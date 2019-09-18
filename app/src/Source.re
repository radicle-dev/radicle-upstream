type account = {
  avatarUrl: string,
  keyName: string,
};

type fetchAccountResult = Belt.Result.t(option(account), string);

type createAccountResult = Belt.Result.t(account, string);

type source = {
  createAccount: (string, string) => Js.Promise.t(createAccountResult),
  fetchAccount: unit => Js.Promise.t(fetchAccountResult),
};

let createLocalSource = () => {
  let localAccount = ref(None);

  let createAccount = (keyName, avatarUrl) =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let account = {keyName, avatarUrl};
      localAccount := Some(account);

      resolve(. Belt.Result.Ok(account));
    });

  let fetchAccount = () =>
    Js.Promise.make((~resolve, ~reject as _) =>
      resolve(. Belt.Result.Ok(localAccount^))
    );

  {createAccount, fetchAccount};
};
