// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Create a plugin object that with  methods that call Cypress tasks.
//
// `methods` must include all the key names of `T`.
export function createPlugin<T>(
  namespace: string,
  methods: Array<keyof T>
): ChainableApi<T> {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const nodeManagerPlugin: any = {};
  methods.forEach(method => {
    nodeManagerPlugin[method] = (arg: unknown) =>
      cy.task(`${namespace}:${method}`, arg, { log: false });
  });
  return nodeManagerPlugin;
}

// Replaces the return type `Promise<S>` of the function type `T` with
// `Cypress.Chainable<S>`.
//
// For example, if
//
//    T ≡ (foo: number) => Promise<string>
//
// then
//
//    ChainableReturn<T> ≡ (foo: number) => Cypress.Chainable<string>
//
type ChainableReturn<T> = T extends (...params: infer P) => Promise<infer R>
  ? (...params: P) => Cypress.Chainable<R>
  : never;

// Replaces the return type `Promise<S>` of all the properties in the
// API object `R` with `Cypress.Chainable<S>`.
//
// For example, if
//
//    R ≡ { bar: (foo: number) => Promise<string> }
//
// then
//
//    ChainableApi<R> ≡ { bar: (foo: number) => Cypress.Chainable<string> }
//
type ChainableApi<R> = {
  [K in keyof R]: ChainableReturn<R[K]>;
};
