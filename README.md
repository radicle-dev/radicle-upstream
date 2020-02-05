[![Build status][ba]][st]

# What is Upstream?

Upstream is a cross-platform desktop client for the radicle protocol.

```
.
├── CHANGELOG.md
├── DEVELOPMENT.md     # guidelines for contributing to this repository
├── LICENSE
├── README.md
├── app                # electron based frontend code lives here
├── fixtures           # test data shared amongst front- and backend
└── proxy              # business logic that ties together both the radicle
                       # protocol and radicle-registry, it provides a GraphQL
                       # API for the frontend
```

At the moment we support Linux and macOS.

At some point we'll provide ready-made packages for both platforms, however
for now a good way to explore the project is to read the documentation and
have a go at building it locally.

A good entry-point for exploration is [`app/README.md`][re]. There
you'll find information on how to build upstream from source. The final build
package will contain both the backend proxy service as well as the electron
shell bundled into a single binary package for your platform.

If you have questions or would like to get in touch, check out
[radicle.community][rc].


[ba]: https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master
[st]: https://buildkite.com/monadic/radicle-upstream
[rc]: https://radicle.community
[re]: app/README.md
