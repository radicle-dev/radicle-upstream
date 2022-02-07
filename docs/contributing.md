# Contributing to Radicle Upstream

## Learn more about Upstream

To familiarise yourself with the Upstream app and the Radicle ecosystem, have a
look at the following resources:

- [development.md][dm] for more information about hacking on the project.
- [Discord][dc]
- [radicle.xyz][ra]
- [Radicle documentation][rd]

## Your first bugfix

If you have found a bug or see an [issue][oi] you'd like to fix, we are keen on
receiving a contribution from you.

We are currently accepting [PR's through GitHub][pr] while we make our own
infrastructure more robust. If it's your first time using GitHub Pull Requests,
learn more about it [here][gh].

We also accept contributions through [Upstream Patches][up] -- our first version
of Pull Requests in Upstream. Learn more about the contribution flow [here][cb].
For us to see you've contributed a Patch, we need to [add your Device ID as a
remote][ar] to our repo. You can either add it to the [Remotes to track
issue][rt] or post it in [#upstream on Discord][dc].

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
Certificate of Origin (DCO)][do]. This means that the messages of all your
commits must include the following line at the end.

    Signed-off-by: John Doe <john.doe@example.com>

You can create commits with this line by running `git commit -s`.

The DCO was created by the Linux Kernel community and is a simple statement that
you, as a contributor, have the legal right to make the contribution.

## Documentation

We're writing documentation as we are developing new features. If you find
something that is confusing or not covered at all, feel free to [open a bug][ob]
or [contribute][cd]

## Contributing to the Design

Since Radicle is an open source project, anyone can contribute. This is normal
in open source development, but we do it for design too!

- Start by joining our [Discord server][dc] to chat with anyone on the core
  team and ask any questions you have. It’s all public and open for anyone to
  join and chat. We even have our “internal” chats in public where we chat
  regularly about features. The #Upstream channel is an example of that.
- There are also a lot of issues on our GitHub marked as “Design needed” which
  are open for anyone to grab and submit a solution for. You can find them
  [here][dn]. Some are easier than others. Feel free to read through those and
  ask any questions directly on the GitHub issue.
- You can also reach out directly on Discord with any questions. If you
  need any help getting set up with our [Figma file][ff] just ask one of the
  core designers on the team ([@brandonhaslegs][bo] and [@juliendonck][jd]).
  You’ll need to duplicate the file and make changes in your own private file.
  If we accept them, we’ll integrate them into the official file.
- Please submit design solutions on GitHub. Just post screenshots, videos, or
  Figma prototype links of your solution and a description on the issue.


[ar]: http://docs.radicle.xyz/docs/using-radicle/tracking-and-viewing#adding-remotes
[cb]: https://docs.radicle.xyz/docs/using-radicle/overview
[cc]: https://www.conventionalcommits.org/en/v1.0.0
[cd]: https://github.com/radicle-dev/radicle-docs#readme
[dc]: https://discord.com/channels/841318878125490186/843873418205331506
[dm]: development.md
[do]: ../DCO
[gh]: https://guides.github.com/introduction/flow/
[ob]: https://github.com/radicle-dev/radicle-docs/issues/new/choose
[oi]: https://github.com/radicle-dev/radicle-upstream/issues
[pr]: https://github.com/radicle-dev/radicle-upstream/pulls
[ra]: https://radicle.xyz
[rd]: https://docs.radicle.xyz
[rt]: https://github.com/radicle-dev/radicle-upstream/issues/1958
[up]: http://docs.radicle.xyz/docs/using-radicle/creating-patches
[dn]: https://github.com/radicle-dev/radicle-upstream/issues?q=is%3Aopen+is%3Aissu+label%3Adesign-needed
[ff]: https://www.figma.com/file/owmgsbs6lnUt8R1bixstCA/Radicle-Upstream?node-id=4147%3A7246
[bo]: https://github.com/brandonhaslegs
[jd]: https://github.com/juliendonck
