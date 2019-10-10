# MVP

This is the app for the product explorations.

### Setup

We consistently use `yarn` for our dependency management and script execution.
Basis for the list of dependencies as well as the scripts supported is the
`package.json`. To install all packages relevant for development run: `yarn`


### Development

In order to access to latest state of the app which updates on any code change
run: `yarn start`.

#### Tests
We follow the code formatting dictated by `refmt` which is part of the
[reason-cli][0] and enforce it locally on `precommit` with [husky][1] and
[lint-staged][2].

Run `yarn test` to have all tests executed and `yarn test:watch` to have
a continuous feedback from our test runner.


[0]: https://github.com/reasonml/reason-cli
[1]: https://github.com/typicode/husky
[2]: https://github.com/okonet/lint-staged
