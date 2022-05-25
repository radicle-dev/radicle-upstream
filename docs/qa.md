## Prerequisites

- Download and install Upstream for the platform specified in the title of this issue:
  - [macOS M1][ms]
  - [macOS][mc]
  - [Linux][ln]

## Checklist
- [ ] Start Upstream and complete all the onboarding steps until you land on the `Profile` screen
  - [ ] macOS Gatekeeper **does not** prevent you from launching Upstream
- [ ] The version number in the _Settings_ screen matches the version number in the title of this issue
- [ ] Track the `radicle-upstream` project by running the following command in your terminal
  - [ ] macOS `open radicle://link/v0/rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o`
  - [ ] Linux `xdg-open radicle://link/v0/rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o`
- [ ] Add a new remote to the `radicle-upstream` project: `hyn5r6yejjco8r77yf7gu6gqsetgjsqt5oitpzu5eu791wej6p3xz6`
  - [ ] Closed and merged patches by "geigerzaehler" appear in the "All" patches list
- [ ] Track the `test-blog` project: `rad:git:hnrkjqfp79nqdyqd9q9y8fzot6gr8anjkn8oo`
  - [ ] Fork the `test-blog` project
  - [ ] Open and publish a patch
- [ ] Create a new project via the terminal and verify that it automatically shows up in Upstream
```
mkdir test-project
cd test-project
echo 'Hello world!' >> README.md
git init --initial-branch main
git add .
git commit --message "initial commit"
rad init --name test-project --default-branch main --description "Testing a release"
```


[ln]: https://releases.radicle.xyz/radicle-upstream-X.X.X-rc.AppImage
[mc]: https://releases.radicle.xyz/radicle-upstream-X.X.X-rc.dmg
[ms]: https://releases.radicle.xyz/radicle-upstream-X.X.X-arm64-rc.dmg
