## Prerequisites

- [ ] Make sure you are installing into an environment with no old
      configuration and no user data [ℹ️](#01)
- [ ] Make sure you have configured your name and email in git [ℹ️](#02)
- [ ] Download [Linux][ln] or [macOS][mc] binary package
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
- [ ] Complete all the onboarding steps until you land on the Profile screen


### Creating projects

- [ ] Can create a new project from a larger existing repository (e.g. radicle-upstream)
  - [ ] UI interaction is blocked while project creation is in progress
  - [ ] `README.md` files are shown by default and markdown is rendered as HTML
    - [ ] Links to external resources open in external browser
    - [ ] Links to internal resources don't do anything
  - [ ] Syntax highlighting works for source files (.toml, .ts, .svelte, etc.)
  - [ ] Commit tab shows a list of all the commits in the branch that was
        selected
  - [ ] Clicking on a commit shows the commit metadata as well as the diff
- [ ] No unreleased features are visible in the UI
    - [ ] _Issues_ and _Revisions_ tabs on the Project screen
    - [ ] _Wallets_ tab on the User Profile screen
    - [ ] Tags are not visible in the revision selector on the _Project Source_
      screen
    - [ ] _Design Sytem Guide_ is not listed in the shortcuts modal
          <kbd>?</kbd> and the respective global hotkey is disabled
          <kbd>⌘</kbd>+<kbd>d</kbd>


### Settings & Misc

- [ ] Links to external help resources open in an external browser
- [ ] The version number in the _Settings_ screen matches:
  - [ ] The version number in the package filename
  - [ ] The version number in the _About Radicle Upstream_ dialog
- [ ] Only one modal is allowed at a time (no modal stacking possible)


### Lifecycle

- [ ] Preferences are persisted across app reboots
  - [ ] Remote helper hint (in the Checkout and "New project" modals) is not
        shown after app restart once it is dismissed by clicking the `x` icon
        in the top right corner


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



[ln]: https://releases.radicle.xyz/radicle-upstream-X.X.X.AppImage
[mc]: https://releases.radicle.xyz/radicle-upstream-X.X.X.dmg
[rs]: https://raw.githubusercontent.com/radicle-dev/radicle-upstream/master/scripts/reset-state.sh
