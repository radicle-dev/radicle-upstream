[![Build status][ba]][st]

# What is Upstream?

Upstream is a cross-platform desktop client for the radicle code collaboration
and registry protocols.

At the moment we support Linux and macOS. Later on we'll provide ready-made
packages for both platforms, however for now a good way to explore the project
is to read the documentation and have a go at building it locally.

The [UI][ui] is written in JavaScript using [Svelte][sv] and Electron and the
node [proxy][pr] logic is implemented in [Rust][ru].

A good entry-point for exploration is [`DEVELOPMENT.md`][de], where you'll find
information on how to build Upstream from source.

If you have questions or would like to get in touch, check out
[radicle.community][rc].


[ba]: https://badge.buildkite.com/4fb43c6b471ab7cc26509eae235b0e4bbbaace11cc1848eae6.svg?branch=master
[st]: https://buildkite.com/monadic/radicle-upstream
[ui]: ui
[sv]: https://svelte.dev
[ru]: https://www.rust-lang.org
[pr]: proxy
[de]: DEVELOPMENT.md
[rc]: https://radicle.community
