# Upstream
[![Build status][ba]][st]

Upstream is a cross-platform desktop client for the radicle code collaboration
protocol.

At the moment we support Linux and macOS. Later on we'll provide ready-made
packages for both platforms, however for now a good way to explore the project
is to read the documentation and have a go at building it locally.

The [UI][ui] is written in JavaScript using [Svelte][sv] and Electron and the
node [proxy][pr] logic is implemented in [Rust][ru].

A good entry point for exploration is [`DEVELOPMENT.md`][de], where you'll find
information on how to build Upstream from source.

If you're looking to contribute, take a look at [`CONTRIBUTING.md`][co] to learn
about the different ways that we accept contributions.

If you have questions or would like to get in touch, check out
[radicle.community][rc].


### Attribution

Upstream uses:
  - [Twemoji by Twitter][tw]
  - [The Inter typeface family by Rasmus Andersson][ra]
  - [Source Code Pro font family by Adobe][so]


[ba]: https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master
[de]: DEVELOPMENT.md
[co]: CONTRIBUTING.md
[pr]: proxy
[ra]: https://rsms.me/inter
[rc]: https://radicle.community
[ru]: https://www.rust-lang.org
[st]: https://buildkite.com/monadic/radicle-upstream
[so]: https://adobe-fonts.github.io/source-code-pro
[sv]: https://svelte.dev
[tw]: https://twemoji.twitter.com
[ui]: ui
