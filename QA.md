This checklist should be updated with every release, any changes to
functionality should be reflected here.

### Packaging and distribution
- [ ] Check that unreleased features are not visible in the UI
  - [ ] Issues, Revisions, Wallets, and anything else that's in the UI but
    clearly not wired up
- [ ] App icon is shown correctly
  - [ ] macOS: dock, <kbd>⌘</kbd> + <kbd>tab</kbd> task switcher, mounted dmg,
    app icon, "About radicle-upstream" window
  - [ ] Linux: dock, menu bar
- [ ] Version in the Settings screen matches version in package filename

### Onboarding
- [ ] Always shown when no identity is set up
  - [ ] First app start
  - [ ] Subsequent app starts if the identity creation was not completed
  - [ ] Can't exit onboarding before identity is created and global keyboard
    shortcuts are disabled
- [ ] It's possible to go to the next screen by pressing <kbd>enter</kbd> if
  the input field validations allow it
- [ ] Handle and passphrase validations work
- [ ] After passphrase input the identity is created and a copyable URN is
  provided
- [ ] After completion we land on our profile page which contains a placeholder
  with instructions on how to create your first project

### Settings
- [ ] Preferences are persisted across app reboots
  - [ ] Color theme selection
  - [ ] Peer entries
  - [ ] Remote helper hint (in the Checkout and "New project" modals) is not
    shown after app restart once it is closed
- [ ] Links to external help resources open in an external browser

### Projects
- [ ] Can create a new project with a new repository
- [ ] Can create a new project from an existing repository
  - [ ] Adding larger projects don't crash the app
  - [ ] UI interaction is blocked while project creation is in progress

#### Working directory (from which a new project was initialised)
- [ ] When pushing changes to Radicle via `git push rad` they should appear in
  the app (a page refresh is still needed for the changes to show up)
- [ ] Pulling changes from Radicle work (to test this you'll need to have a
  checked out another working copy in a different folder)

#### Checkout (a separate working copy)
  - [ ] Follow instructions in the UI to set up the path to the git helper in
    your shell
  - [ ] It's possible to create a new working copy from an existing project
    - [ ] Pushing new commits to Radicle via `git push rad` work (temporary
      password until we have passphrases: `radicle-upstream`)
    - [ ] Pulling changes work: make changes in the project folder you created
      in project creation, push them to Radicle with `git push rad`, switch to
      the checkout working directory, do a `git pull`

#### Source browsing
- [ ] Metadata and stats in UI reflect what is in the actual repository
- [ ] `README.md` files are shown by default and markdown is rendered as HTML
  - [ ] Links to external resources open in external browser
  - [ ] Links to internal resources don't do anything
  - [ ] If the project doesn't have a readme, a placeholder is shown
- [ ] Syntax highlighting works for source files
- [ ] Binary files show a placeholder
- [ ] It's possible to navigate to deeper hierarchies via the tree browser
- [ ] It's possible to select different branches

#### Commit browsing
- [ ] Commit tab shows a list of all the commits in the branch that was
  selected
- [ ] Clicking on a commit shows the commit metadata as well as the diff

### Misc UI
- [ ] Clicking on an identifier copies it to the clipboard

#### Global keyboard shortcuts
- [ ] All documented shortcuts (a list is provided by pressing `?`) work
- [ ] Only one modal is allowed at a time (no modal stacking possible)


[re]: https://github.com/radicle-dev/radicle-upstream/blob/master/CHANGELOG.md
