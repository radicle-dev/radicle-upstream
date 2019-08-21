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
[reason-cli][0] and enforce it locally on `precommit` with [husky][1] and
[lint-staged][2].

Run `yarn test` to have all tests executed and `yarn test:watch` to have
a continuous feedback from our test runner.

### Deployment

The app can be deployed to a CDN as a static page, so far it only shows dummy
data and the deployment is meant only for presentation purposes.

The project is hosted on Firebase, so deployments require the [Firebase CLI][3].
Install and configure the deployment tooling like so:

```
npm install -g firebase-tools
firebase login
```

To be able to deploy you'll need to ask for permissions to the Firebase project
from someone on the team. This can be done via the [Firebase console][4]:
`Project Overview (cogwheel icon)` -> `Users and permissions` -> `Add member`

To build and deploy the latest changes run `npm run deploy`. The published
app can be seen [here][5].


[0]: https://github.com/reasonml/reason-cli
[1]: https://github.com/typicode/husky
[2]: https://github.com/okonet/lint-staged
[3]: https://firebase.google.com/docs/cli/
[4]: https://console.firebase.google.com/project/product-mvp-gf3s2
[5]: https://product-mvp-gf3s2.firebaseapp.com
