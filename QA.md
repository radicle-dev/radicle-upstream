## Prerequisites

- [ ] make sure you are installing into an environment with no old
      configuration and no user data [ℹ️](#01)
- [ ] make sure you have configured your name and email in git [ℹ️](#02)
- [ ] [download][bu] the binary package
- [ ] install Upstream from the downloaded package [ℹ️](#03)
  - [ ] macOS Gatekeeper **does not** show the following message:
        _macOS cannot verify that this app is free from malware_

## QA checklist

### Packaging and distribution

- [ ] Check that unreleased features are not visible in the UI
  - [ ] Experimental features and developer helpers are not accessible
    - Issues and Revisions tabs on the Project screen
    - Wallets tab on the User Profile screen
    - Design Sytem Guide is not listed in the shortcuts modal <kbd>?</kbd> and
      the respective global hotkey is disabled <kbd>⌘</kbd>+<kbd>d</kbd>
    - Tags are not visible in the revision selector on the Project Source
      screen
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
- [ ] After passphrase input the identity is created and a copyable Device ID
      is provided
- [ ] After completion we land on our profile page which contains a placeholder
      with instructions on how to create your first project


### Settings

- [ ] Preferences are persisted across app reboots
  - [ ] Color theme selection
  - [ ] Network seeds
  - [ ] Remote helper hint (in the Checkout and "New project" modals) is not
        shown after app restart once it is dismissed by clicking the `x` icon
        in the top right corner
- [ ] Links to external help resources open in an external browser


### Projects

- [ ] Can create a new project with a new repository
  - [ ] Validations work
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
    - [ ] Pushing new commits to Radicle via `git push rad` work
    - [ ] Pulling changes work: make changes in the project folder you created
          in project creation, push them to Radicle with `git push rad`, switch
          to the checkout working directory, do a `git pull`


#### Source browsing

- [ ] Metadata and stats in UI reflect what is in the actual repository; here
      are a couple commands that will help you get the numbers from
      a repository:
  - local branch count
    `git branch | wc -l`
  - all unique contributors across all branches
    `git shortlog --summary --numbered --email --all`
  - all unique contributors in a specific branch
    `git shortlog --summary --numbered --email myfunkybranch`
  - unique commit count across all branches
    `git rev-list --all --count`
  - commit count in a specific branch
    `git rev-list --count myfunkybranch`
- [ ] `README.md` files are shown by default and markdown is rendered as HTML
  - [ ] Links to external resources open in external browser
  - [ ] Links to internal resources don't do anything
  - [ ] If the project doesn't have a `README.md`, a placeholder is shown
- [ ] Syntax highlighting works for source files (.toml, .sol, .ts, .svelte)
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

### Proxy Error

- [ ] Killing the proxy process while the app is running shows a blue error
      screen with the proxy logs.


## Hints

### How to set up a clean environemnt? <a href="#user-content-01" id="01">🔗</a>

**Safe method**: use a temporary user account on your computer.

  - on macOS:
    - if you are **not** using FileVault, switch to the "Guest User". You may
      have to enable this in "System Preferences -> Users & Groups".  When
      you're done, all data will be removed automatically.
    - if you **are** using FileVault, create a new user in "System
      Preferences -> Users & Groups". When you're done, you'll need to remove
      this user manually.
  - on Linux:
    - create a new user with `sudo useradd -m qa`, and log into that account.
      When you're done, remove the user with `sudo userdel -r qa`.  _Note:
      "qa" is just an example user name, you can choose anything you like_

**Dangerous method**: remove all directories manually.

You can use [this script][rs]. Make sure you have a backup of your data,
or are using this in combination with the safe method (i.e. while logged
in with a temporary user account).


### How to set up git? <a href="#user-content-02" id="02">🔗</a>
    git config --global user.name "Mona Lisa"
    git config --global user.email "email@example.com"


### How to install? <a href="#user-content-03" id="03">🔗</a>

**On macOS:**

  1. open the `radicle-upstream-X.X.X.dmg` package
  2. install Upstream by dragging the `Radicle Upstream` binary to
     `/Applications`
  3. run `/Applications/Radicle Upstream.app` by double clicking it

**On Linux (AppImage):**

  1. `chmod +x <PATH_TO_DOWNLOAD>/radicle-upstream-X.X.X.AppImage`
  2. run `PATH_TO_DOWNLOAD>/radicle-upstream-X.X.X.AppImage` by executing it
     from the terminal or clicking on it.



[rs]: https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/scripts/reset-state.sh
[bu]: https://releases.radicle.xyz/radicle-upstream-0.1.5.dmg
