# Submit your first patch using Upstream

This guide will walk you through the necessary steps to set up Upstream and
create your first patch. Have a look at the [issue list][il] for ideas on where
we need some help.


## Set up Upstream

1. Download and install the latest version of Radicle Upstream from our
   [website][ti].

2. Start Upstream and create an identity by following the onboarding
   instructions in the app, on the last screen with the title "All set!" choose
   "Go to profile". Setting up a wallet is not required for submitting
   patches.

## Set up `rad`

1. Install the latest version of the `rad` CLI. You can find instructions
   [here](https://github.com/radicle-dev/radicle-cli/#installation).

2. Run `rad auth` to authenticate the CLI


## Get the Upstream project

1. Open the "Search" dialog by pressing `⌘` + `p` (on macOS) or `ctrl` + `p`
   (on Linux).

2. Follow the Upstream project by pasting its Project URN
   `rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o` into the search bar and
   pressing the `enter` key.

   Upstream will navigate to your "Profile" screen showing a card with the
   title "Still looking…". Once the project has been fetched from the seed, the
   title of this card will switch to "radicle-upstream". On a broadband
   connection this should not take longer than a couple of seconds.

3. Navigate to the project by clicking the "radicle-upstream" project card.


## Create a local working copy

1. Click the "Fork" button.

2. Click the "Choose" button inside the modal and select a directory on your
   local disk where the working copy should be checked out.

3. Click the "Fork" button inside the modal.
   After a while a notification should appear confirming the completion of the
   checkout saying "radicle-upstream checked out to …".

4. Navigate to the folder containing working copy and run `git config --local
   rad.seed https://maple.radicle.garden` to configure the seed peer for
   Upstream.

## Make Upstream commands available in your terminal

1. Add this line to your terminal configuration or the `~/.profile` file:
   ```
   export PATH="$HOME/.radicle/bin:$PATH"
   ```

2. Restart your terminal.

3. Verify that the terminal integration was successful by running these
   commands and comparing the output:
   ```
   which upstream
   /Users/rudolfs/.radicle/bin/upstream

   which git-remote-rad
   /Users/rudolfs/.radicle/bin/git-remote-rad
   ```
   The paths may differ depending on your OS, the important thing is that
   the files are found. If the output mentions "upstream not found" or
   "git-remote-rad not found" the terminal integration didn't work and needs
   further troubleshooting.


## Create a patch

1. Navigate to the working copy that you created in the
   "Create a local working copy" section.
   ```
   cd /path/to/working/copy
   ```

2. Create and checkout a new branch that will contain the changes for your
   patch. The branch name should be short and descriptive – it will be used to
   identify the patch.
   ```
   git checkout --branch my-proposed-changes
   ```

3. Make your changes to the code.

4. Commit your changes following the ["Commit hygiene" guidelines][co] and
   signing the [DCO][do] for each commit with:
   ```
   git commit -s
   ```

5. Create your patch:
   ```
   upstream patch create
   ```
   Your default editor will open allowing you to enter the title and
   description for the patch. The first line is the title, followed by a blank
   newline, followed by the description of the patch. Reference relevant Github
   issues by including their URL in the patch description. Save and exit the editor.

6. Publish your patch:
   ```bash
   rad sync
   ```

7. Verify that the patch shows up in Upstream by navigating to the project’s
   "Patches" tab.


## Inform Upstream maintainers about your proposed patch

You have two options to announce your patch:
- on the [#Upstream][dc] discord channel
- on this [GitHub issue][gh]

Please include the patch URL in your message.

You can copy the patch URL by clicking the patch ID which is visible
in both the patch list screen as well as the patch screen. The button looks
something like this:
`hyn5r6yejjco8r77yf7gu6gqsetgjsqt5oitpzu5eu791wej6p3xz6/no-session-error`
and says "Copy shareable link to clipboard" when you hover it.

A patch URL has the following format:
radicle://upstream/v0/project/<PROJECT_ID>/patch/<PEER_ID>/<PATCH_ID>


## Address requested changes if needed

At this point one of the maintainers will review your patch. If it passes our
review, it will be merged and published by the maintainers.

We'll reach out to you in case there are any changes required.
To update and re-publish your patch do the following:

1. Go to the project checkout directory, and make sure you're
   on the same branch as when you proposed the patch:
   ```
   cd path/to/working/copy
   git checkout my-proposed-changes
   ```

2. Make the requested changes.

3. Commit the changes with `git commit -s`.

4. Once you've addressed all the comments, update the patch:
   ```
   upstream patch update
   ```
   This will open the existing patch title/description in your editor.
   You can either update the title/description or save as it is. Once you leave
   your editor, the terminal will prompt you for your passphrase and publish
   the updated patch.

5. Inform the maintainers that you addressed the requested changes and
   re-published your patch.

This process may be repeated until the maintainer accepts and merges in your
patch.


[co]: contributing.md
[dc]: https://discord.gg/radicle
[do]: ../DCO
[gh]: https://github.com/radicle-dev/radicle-upstream/issues/1958
[il]: https://github.com/radicle-dev/radicle-upstream/issues
[ti]: https://radicle.xyz/tryit
