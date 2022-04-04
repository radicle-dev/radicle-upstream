# Contributing to Radicle Upstream

## Learn more about Upstream

To familiarise yourself with Upstream and the Radicle ecosystem, have a look at
the following resources:

- [development.md][dm] for more information on building Upstream locally
- [Discord][dc]
- [radicle.xyz][ra]
- [Radicle documentation][rd]

## Your first contribution to Upstream

If you find a bug or see an [issue][oi] you'd like to fix, we are keen on
receiving a contribution from you.

We accept contributions through Upstream Patches -- our first version
of Pull Requests in Upstream. Follow [this guide][tg] to create your first
patch.

In case there's an issue with Upstream Patches, we are also accepting
[Pull Requests through GitHub][pr] as a fallback option.

## Commit hygene

Commits have to adhere to the following guidelines to be accepted.

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

The DCO was created by the Linux Kernel community and is a simple statement
that you, as a contributor, have the legal right to make the contribution.


[cc]: https://www.conventionalcommits.org/en/v1.0.0
[dc]: https://discord.com/channels/841318878125490186/843873418205331506
[dm]: development.md
[do]: ../DCO
[oi]: https://github.com/radicle-dev/radicle-upstream/issues
[pr]: https://github.com/radicle-dev/radicle-upstream/pulls
[ra]: https://radicle.xyz
[rd]: https://docs.radicle.xyz
[tg]: submit-patch-using-upstream.md
