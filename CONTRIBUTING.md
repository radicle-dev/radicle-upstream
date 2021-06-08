# Contributing to Radicle Upstream

## Learn more about Upstream

To familiarise yourself with the Upstream app and the Radicle ecosystem, have a
look at the following resources:

- [DEVELOPMENT.md][dmd] for more information about hacking on the project.
- [Discord][dc]
- [radicle.xyz][ra]
- [Radicle documentation][rd]

## Your first bugfix

If you have found a bug or see an [issue][oi] you'd like to fix, we are keen on
receiving a contribution from you.

We are currently accepting [PR's through GitHub][pr] while we make our own
infrastructure more robust. If it's your first time using GitHub Pull Requests,
learn more about it [here][ghf].

We also accept contributions through [Upstream Patches][up] -- our first version
of Pull Requests in Upstream. Learn more about the contribution flow
[here][cbf]. For us to see you've contributed a Patch, we need to [add your
Device ID as a remote][ar] to our repo. You can either add it to the [Remotes to
track issue][rtr] or post it in [#upstream on Discord][dc-up].

## Creating a commit

For you commits to be accepted they have to follow a couple of guidelines.

### Conventional commit message

Your commits should be formatted according to the [conventional commits
specification][cc].

Here are a couple of examples:

```plain
fix: fix clippy on CI (#430)
refactor(ui): improve cypress spec reliability (#429)
style(ui): icon refresh (#411)
chore(release): 0.0.11 (#417)
test(ui): add missing project creation specs (#404)
feat(proxy): improve session (#380)
```

### Certificate of Origin

We require commits to be signed off to show your agreement to the [Developer
Certificate of Origin (DCO)](./DCO). This means that the messages of all your
commits must include the following line at the end.

    Signed-off-by: John Doe <john.doe@example.com>

You can create commits with this line by running `git commit -s`.

The DCO was created by the Linux Kernel community and is a simple statement that
you, as a contributor, have the legal right to make the contribution.

## Documentation

We're writing documentation as we are developing new features. If you find
something that is confusing or not covered at all, feel free to [open a bug][ob]
or [contribute][cd]

[gz]: https://github.com/geigerzaehler
[ru]: https://github.com/rudolfs
[bhl]: https://github.com/brandonhaslegs
[jd]: https://github.com/juliendonck
[dmd]: DEVELOPMENT.md
[dc]: https://discord.gg/HRdnwAwGbG
[dc-up]: https://discord.com/channels/841318878125490186/843873418205331506
[ra]: https://radicle.xyz
[rd]: https://docs.radicle.xyz
[oi]: https://github.com/radicle-dev/radicle-docs/issues
[pr]: https://github.com/radicle-dev/radicle-upstream/pulls
[ghf]: https://guides.github.com/introduction/flow/
[up]: http://docs.radicle.xyz/docs/using-radicle/creating-patches
[cbf]: https://docs.radicle.xyz/docs/using-radicle/overview
[ar]: http://docs.radicle.xyz/docs/using-radicle/tracking-and-viewing#adding-remotes
[rtr]: https://github.com/radicle-dev/radicle-upstream/issues/1958
[mt]: mailto:julien@monadic.xyz
[ob]: https://github.com/radicle-dev/radicle-docs/issues/new/choose
[cd]: https://github.com/radicle-dev/radicle-docs#readme
[cc]: https://www.conventionalcommits.org/en/v1.0.0
