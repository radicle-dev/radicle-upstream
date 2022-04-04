### Colors

The design system supports multiple color palettes via themes which can be
changed in the Settings screen.

Throughout the codebase we use only CSS variables. Raw color codes should not
be used so changes to global styling can be applied in one central place:
`public/colors.css`.

Read more about the colors used in Upstream in the [Color System
post](https://radicle.community/t/color-system/166).

### Typography

The design system provides a constrained set of typographic styles. This
consists of a set of styled headers, a set of styled paragraphs and a set of
modifiers. These also overlap with the components we have in our design system
in Figma, where the design of the app exists. All classes are prefixed with
`typo-` so this might be helpful if you have any autocomplete in your editor.

For the headers you can just use `<h1>` up to `<h5>`, if you want to apply the
same styles to other html elements you can use the matching classes
`typo-header-1` to `typo-header-5` (use `<h1>` to `<h5>` where you can).

For text we you can use the classes that start with `typo-text`. These come in
2 sizes, the normal one and `typo-text-small`. Check out
[typography.css](./static/typography.css) to get an idea of the possible
combinations. All the ones we're using in Figma are represented here.

The modifiers give us some flexibility and allow us to create classes for
certain css functionality we use over and over. Such as,
`typo-overflow-ellipsis` and `typo-all-caps`. These should be self-explanatory.

We also added a set of modifiers that allow you to add the font-family as a
class where you need it, here again we would recommend not doing that as most
styles should fit into one of the two categories above.

The only place in the app where we're not using this is in `<Markdown />`,
since the library we use doesn't allow us to overwrite the styles without using
global declarations. If you have any questions or improvements, open an issue
and we're happy to help you along.

## Contributing to design

Since Radicle is an open source project, anyone can contribute. This is normal
in open source development, but we do it for design too!

  - Start by joining our [Discord server][dc] to chat with anyone on the core
    team and ask any questions you have. It’s all public and open for anyone to
    join and chat. We even have our “internal” chats in public where we chat
    regularly about features. The #Upstream channel is an example of that.

  - There are also a lot of issues on our GitHub marked as “Design needed”
    which are open for anyone to grab and submit a solution for. You can find
    them [here][dn]. Some are easier than others. Feel free to read through
    those and ask any questions directly on the GitHub issue.

  - You can also reach out directly on Discord with any questions. If you need
    any help getting set up with our [Figma file][ff] just ask one of the core
    designers on the team ([@brandonhaslegs][bo] and [@juliendonck][jd]).
    You’ll need to duplicate the file and make changes in your own private
    file. If we accept them, we’ll integrate them into the official file.

  - Please submit design solutions on GitHub. Just post screenshots, videos, or
    Figma prototype links of your solution and a description on the issue.


[bo]: https://github.com/brandonhaslegs
[dc]: https://discord.com/channels/841318878125490186/843873418205331506
[dn]: https://github.com/radicle-dev/radicle-upstream/issues?q=is%3Aopen+is%3Aissu+label%3Adesign-needed
[ff]: https://www.figma.com/file/owmgsbs6lnUt8R1bixstCA/Radicle-Upstream?node-id=4147%3A7246
[jd]: https://github.com/juliendonck
