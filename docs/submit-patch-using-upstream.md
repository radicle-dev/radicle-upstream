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

3. After onboarding, navigate to the "Network" screen by pressing
   `⌘` + `b` (on macOS) or `ctrl` + `b` (on Linux).

4. Add our seed node to the list of seeds:
   ```
   hydyq6xmgp3amt44z41n6cbods1osx73j5z6fky5xx4yx33afycyfc@34.88.37.244:8776
   ```
   Once you do that, the network status indicator on the right-hand side should
   switch from "You’re not connected to any peers" to "You’re connected to 1
   peer".


## Get the source code

1. Open to the "Search" dialog by pressing `⌘` + `p` (on macOS) or `ctrl` + `p`
   (on Linux).

2. Follow the Upstream project by pasting in its project ID
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
   checkout saying "radicle-upstream checked out to …"


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

1. In your terminal, navigate to the working copy that you created in the
   "Create a local working copy" section.
   ```
   cd /path/to/working/copy
   ```

2. Create and checkout a new branch that will contain the changes for your
   patch. The branch name should be short and descriptive – it will be used to
   identify the patch
   ```
   git checkout --branch my-proposed-changes
   ```

3. Make your changes to the code.

4. Commit your changes following the ["Commit hygene" guidelines][co] and
   signing the [DCO][do] for each commit with:
   ```
   git commit -s
   ```

5. Publish your patch:
   ```
   upstream patch create
   ```
   Your default editor will open allowing you to enter the title and
   description for the patch. The first line is the title, followed by a blank
   newline, followed by the description of the patch. Reference relevant Github
   issues by including their URL in the patch description.

   Save and exit the editor, this will prompt you for the passphrase you chose
   when setting up your Radicle identity in the "Setting up Upstream" section.
   After entering your passphrase and pressing the `enter` key, the patch will
   be published.

   Note: Upstream should be running in the background for the patch to be
   synced to the seed node. At the moment there is no visual feedback whether
   the patch has been synced or not.

6. Verify that the patch shows up in Upstream by refreshing the UI via
   `⌘` + `r` (on macOS) or `ctrl` + `r` (on Linux), navigating to the project
   and the "Patches" tab, where you should see your patch.


## Inform Upstream maintainers about your proposed patch

You have two options to announce your patch:
- on the [#Upstream][dc] discord channel
- on this [GitHub issue][gh]

You should include your Device ID and Patch ID in the message, e.g.:
```
Patch: `hyy4aj3wiqq1o6m5un17sq4m74i4btx8w4ypryri91nrauz9odhfis/upstream-contrib-docs-names`,
Device ID: `hydqsnkr181w1zfidtocgosxghdu8n8d1wsemzgtszhzjru55ggazk`
```

You can find your Device ID in the settings screen `⌘` + `,` (on macOS) or
`ctrl` + `,` (on Linux) and the Patch ID in the "Patches" tab of the project.


## Address requested changes if needed

At this point one of the maintainers will review your patch. If it passes our
review, it will be merged and published by the maintainers.

We'll reach out to you in case there are any changes required.
To update and re-publish your patch do the following:

1. In your terminal, go to the project checkout directory, and make sure you're
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
