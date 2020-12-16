## Prerequisites

- [ ] Make sure you are installing into an environment with no old
      configuration and no user data [ℹ️](#01)
- [ ] Make sure you have configured your name and email in git [ℹ️](#02)
- [ ] [Download][bu] the binary package
- [ ] Install Upstream from the downloaded package [ℹ️](#03)
  - [ ] macOS Gatekeeper **does not** show the following message:
        _macOS cannot verify that this app is free from malware_


## QA checklist

### Packaging

- [ ] App icon is shown correctly
  - macOS:
    - [ ] Dock
    - [ ] <kbd>⌘</kbd> + <kbd>tab</kbd> task switcher
    - [ ] Mounted dmg
    - [ ] App icon
  - Linux:
    - [ ] Dock
    - [ ] Menu bar


### Onboarding

- [ ] Start Upstream
  - on Linux: run `PATH_TO_DOWNLOAD>/radicle-upstream-X.X.X.AppImage` by
    executing it from the terminal or clicking on it
  - on macOS: run `/Applications/Radicle Upstream.app` by double clicking it
- [ ] Onboarding starts on the _Get started_ screen
- [ ] Pressing <kbd>enter</kbd> leads to the next screen
- [ ] Using `$$` as the _Display name_ shows a validation error
- [ ] Entering two different passphrases shows a validation error
- [ ] On the _All set!_ screen clicking the _Device ID_ copies it to the
      clipboard
- [ ] Clicking the _Go to my projects_ button leads to the _Profile_ screen
- [ ] Hovering the network icon in the sidebar shows
      _You're connected to 1 peer_


### Creating projects

- [ ] Can create a new project with a new repository
  - [ ] Using `%%` for the project name shows a validation error
  - [ ] A placeholder is shown for the missing `README.md`
- [ ] Can create a new project from a larger existing repository
  - [ ] UI interaction is blocked while project creation is in progress
  - [ ] `README.md` files are shown by default and markdown is rendered as HTML
    - [ ] Links to external resources open in external browser
    - [ ] Links to internal resources don't do anything
  - [ ] Syntax highlighting works for source files (.toml, .sol, .ts, .svelte)
  - [ ] Binary files show a placeholder
  - [ ] It's possible to navigate to deeper hierarchies via the tree browser
  - [ ] It's possible to select different branches
  - [ ] Metadata and stats in UI reflect what is in the actual repository
        [ℹ️](#04)
  - [ ] Commit tab shows a list of all the commits in the branch that was
        selected
  - [ ] Clicking on a commit shows the commit metadata as well as the diff
- [ ] No unreleased features are visible in the UI
    - [ ] _Issues_ and _Revisions_ tabs on the Project screen
    - [ ] _Wallets_ tab on the User Profile screen
    - [ ] Tags are not visible in the revision selector on the _Project Source_
      screen


### Settings

- [ ] Links to external help resources open in an external browser
- [ ] All documented shortcuts work. Press <kbd>?</kbd> to open the
      _Keyboard shortcuts_ help screen.
  - [ ] Only one modal is allowed at a time (no modal stacking possible)
  - [ ] _Design Sytem Guide_ is not listed in the shortcuts modal
        <kbd>?</kbd> and the respective global hotkey is disabled
        <kbd>⌘</kbd>+<kbd>d</kbd>
- [ ] The version number in the _Settings_ screen matches:
  - [ ] The version number in the package filename
  - [ ] The version number in the _About Radicle Upstream_ dialog


### Replicating projects

- [ ] Replicate a project from seedling.radicle.xyz
  - [ ] Pick a project from seedling.radicle.xyz and search for it by pasting
        the `Radicle ID` of the project into the search bar by pressing
        <kbd>⌘</kbd>+<kbd>p</kbd>, then click the _Follow_ button
    - [ ] The project shows up in the _Following_ tab of the profile screen in
          the waiting area
    - [ ] After a while the project is replicated and moves out of the waiting
          area
    - [ ] When going to the Project source screen, the maintainer is selected
          in the peer selector
      - [ ] Your identity does not show up in the peer selector
  - [ ] Set up the remote helper according to the instructions in the _Fork_
        modal hint
  - [ ] Fork the replicated project to a local directory
    - [ ] The forked project should now appear in the _Projects_ tab of the
          profile screen
    - [ ] Your identity shows up in the peer selector and is selected by
          default instead of the maintainer's identity
    - [ ] The button is now called _Checkout_ instead of _Fork_
    - [ ] Create a commit and publish your chages via `git rad push`
      - [ ] The published commit appears in the _Commits_ tab of the project
  - [ ] Add a remote that doesn't track the project, e.g.
        `hync6jo9q9n4rfgag3o99gann391nsjhuq3oaemoe13oyn3gruwrgh`
    - [ ] Shows the remote in the _Still looking…_ section
  - [ ] Clicking the _Unfollow_ button removes the remote


### Lifecycle

- [ ] Preferences are persisted across app reboots
  - [ ] Color theme selection
  - [ ] Network seed changes
  - [ ] Remote helper hint (in the Checkout and "New project" modals) is not
        shown after app restart once it is dismissed by clicking the `x` icon
        in the top right corner
- [ ] Killing the proxy process while the app is running shows a blue error
      screen with the proxy logs
  - [ ] Clicking the button copies the logs to the clipboard


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

**On Linux (AppImage):**

  1. `chmod +x <PATH_TO_DOWNLOAD>/radicle-upstream-X.X.X.AppImage`


### How to query repo stats from the CLI? <a href="#user-content-04" id="04">🔗</a>

```
  # local branch count
  git branch | wc -l

  # all unique contributors across all branches
  git shortlog --summary --numbered --email --all

  # all unique contributors in a specific branch
  git shortlog --summary --numbered --email myfunkybranch

  # unique commit count across all branches
  git rev-list --all --count

  # commit count in a specific branch
  git rev-list --count myfunkybranch
```



[rs]: https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/scripts/reset-state.sh
[bu]: https://releases.radicle.xyz/radicle-upstream-X.X.X.dmg
