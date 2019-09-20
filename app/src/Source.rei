/** Name and metadata of a keypair owned by a user. **/
type account = {
  avatarUrl: string,
  keyName: string,
};

/** Returned upon new account creation, either the full account or a reason why
 ** the creation failed
 **/
type createAccountResult = Belt.Result.t(account, string);

/** Returend upon account fetch, with maybe an account or a reason why the fetch
 ** failed.
 **/
type fetchAccountResult = Belt.Result.t(option(account), string);

/** The type used to fetch data vital to the app. **/
type source = {
  createAccount: (string, string) => Js.Promise.t(createAccountResult),
  fetchAccount: unit => Js.Promise.t(fetchAccountResult),
};

/** Returns an implementation of `source` which gives back local data. **/
let createLocalSource: unit => source;
