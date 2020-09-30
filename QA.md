**OS version:** (eg. `Catalina 10.15.6` or `Ubuntu Xenial Xerus 16.04.1 LTS`)\
**Link to build:** (eg. https://buildkite.com/monadic/radicle-upstream/builds/4727)


### Prerequisites

- [ ] make sure you are installing into an environment with no old configuration
      and user data

     **Safe method**: Use a temporary user account on your computer

     - on macOS:
       - If you are **not** using FileVault, switch to the "Guest User". You
         may have to enable this in "System Preferences -> Users & Groups".
         When you're done, all data will be removed automatically.
       - If you **are** using FileVault, create a new user in "System
         Preferences -> Users & Groups". When you're done, you'll need to
         remove this user manually.
     - on Linux:
       - Create a new user with `sudo useradd -m qa`, and log into that
         account. When you're done, remove the user with `sudo userdel -r qa`.
         _Note: "qa" is just an example user name, you can choose anything you
         like_

     **Dangerous method**: Remove all directories manually

      You can use [this script](./scripts/reset-state.sh). Make sure you have a
      backup of your data, or are using this in combination with the safe method
      (i.e. while logged in with a temporary user account).

- [ ] download the binary package from the location provided above
- [ ] remove any old configuration and user data
  - on macOS:
    1. open the `radicle-upstream-x.x.x.dmg` package
    2. install Upstream by dragging the `radicle-upstream` binary to
       `/Applications`
    3. open `/Applications` in finder, locate the `radicle-upstream` app and
       right-click it, then select `Open` from the context menu, you should see
       a dialog with two buttons `Move to Bin` and `Cancel`
    4. dismiss the dialog by clicking `Cancel`, to bypass macOS security
       measures which disallow running unsigned binaries, repeat the previous
       step, now you should see three buttons: `Move to Bin`, `Open` and
       `Cancel`, click `Open`
    5. quit the app with <kbd>⌘</kbd> + <kbd>q</kbd>, open your terminal
       and run:
    ```
    /Applications/radicle-upstream.app/Contents/MacOS/radicle-upstream --reset-state
    ```
  - on Linux: run the `.AppImage` like so:
    ```
    /<PATH_TO_DOWNLOAD>/radicle-upstream-0.0.14.AppImage --reset-state
    ```
- [ ] confirm that the state reset was actually performed, the output should
      look like this:

  ```
  INFO  api > Resetting application state...
  INFO  api > done
  INFO  api > Resetting CoCo state...
  INFO  api > done
  ```


### Packaging and distribution

- [ ] Check that unreleased features are not visible in the UI
  - [ ] Experimental features and developer helpers are not accessible
    - Issues and Revisions tabs on the Project screen
    - Wallets tab on the User Profile screen
    - Design Sytem Guide is not listed in the shortcuts modal <kbd>?</kbd> and
      the respective global hotkey is disabled <kbd>⌘</kbd>+<kbd>d</kbd>
    - Tags are not visible in the revision selector on the Project Source screen
    - There is no "Session management" section and no "Clear session" button on
      the Settings screen
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
  - handle
    - starts with a letter or digit
    - is at least 2 characters long
    - can contain the letters `a`-`z`, `A`-`Z`, digits `0`-`9` and special
      characters `-`, `_`
    - all of the following examples should show a validation error:
      `_john`, `x`, `linu$$$`, `-moira`
  - passphrase
    - is at least 4 characters long
    - the repeated passphrase is equal to the first one
    - all of the following example pairs should show a validation error:
      `1`/`1`, `12`/`12`, `123`/`123`, `supersecret`/`supersceret`
- [ ] After passphrase input the identity is created and a copyable URN is
      provided
- [ ] After completion we land on our profile page which contains a placeholder
      with instructions on how to create your first project


### Settings

- [ ] Preferences are persisted across app reboots
  - [ ] Color theme selection
  - [ ] Peer entries
  - [ ] Remote helper hint (in the Checkout and "New project" modals) is not
        shown after app restart once it is dismissed by clicking the `x` icon
        in the top right corner
- [ ] Links to external help resources open in an external browser


### Projects

- [ ] Can create a new project with a new repository
  - [ ] Name validations work
      - starts with a letter or digit
      - is at least 2 characters long
      - can contain the letters `a`-`z`, `A`-`Z`, digits `0`-`9` and special
        characters `-`, `_`, `.`
      - all of the following examples should show a validation error:
        `1`, `-fancy-`, `_doh_`, `rx~pixels`, `💁👌🎍😍`
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
