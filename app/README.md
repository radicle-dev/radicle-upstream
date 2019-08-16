# MVP

This is the app for the product explorations.

### Setup

We consistently use `yarn` for our dependency management and script execution.
Basis for the list of dependencies as well as the scripts supported is the
`package.json`. To install all packages relevant for development run: `yarn`

### Run

In order to access to latest state of the app which updates on any change, two
commands need to run in parallel. One for the compiler to recomplile the
sources: `yarn start`.

And in another session have the development server running which live reloads
the output produced by the compiler and applies any build pipeline changes
configured via `webpack.config.js`: `yarn server`.

### Development

We follow the code formatting dictated by `refmt` which is part of the
[reason-cli](https://github.com/reasonml/reason-cli) and enforce it locally on
`precommit` with [husky](https://github.com/typicode/husky) and
[lint-staged](https://github.com/okonet/lint-staged).

Run `yarn test` to have all tests executed and `yarn test:watch` to have
a continuous feedback from our test runner.
