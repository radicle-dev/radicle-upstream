# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.3.1](https://github.com/radicle-dev/radicle-upstream/compare/v0.3.0...v0.3.1) (2022-05-24)

### 💡 General Notes

This release has been primarily focused on **improving the usefulness of our
patches.** We know that the patches functionality is a core piece of how teams
collaborate using the Radicle platform, and this release adds **patch
statuses** to allow your team members to easily manage the various patches
within your project.

Be sure to check out our [new onboarding docs](https://docs.radicle.xyz/getting-started)
that explain how to get set up using Radicle for your project!

### 🎉 Notable Changes

#### Patch statuses implementation

Previously, the only way for you to close a patch is by merging it into your
branch. Now obviously this results in a bunch of statuses that remain open if
you don’t merge it for any reason (as we’ve painfully experienced ourselves).
Now **both project delegates and patch creators will be able to manually close
patches on their projects**. This means that project delegates can go into each
patch and close them out. Patch creators can also view their own created patch
and do the same. Time to finally clean out your open patches!

#### UI/UX Improvements

Along with the upgrade to patches mentioned above, we’ve made some **additional
improvements to the UI**, allowing you to view the number of patches for each
status (Open, Merged, Closed and All).  You’ll also notice that the actions you
can take for patches (such as creating a patch, checking out, and merging a
patch) only show when it makes sense within the context.

Further, we’ve made some additional improvements to our design system, allowing
our teams to deliver cohesive features for you faster.

#### Onboarding bug fixes

In our previous release, we built a helpful onboarding flow that helps check
dependencies in your system to ensure that your environment is ready to go. In
this release, we’ve fixed a few additional bugs in that flow, specifically
around `git` detection. Our goal is to ensure that no one has any issues
installing and using Upstream.

If you come across any issues at all, please feel free to reach out to us in
our [Discord support channel](https://discordapp.com/channels/841318878125490186/841342872082579466)
and we’ll make sure we address them!


## [0.3.0](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.14...v0.3.0) (2022-04-18)

### 💡 General Notes

As you’ll see below, we’ve made some fairly significant changes to the Radicle
Upstream client for this release. The primary thing we were focused on is to
**make sure that the client is stable** so your teams can begin using it for
code collaboration. As such, we’ve had to make the difficult decision to remove
a few features that were causing networking problems.

Moving forward, our goal is to **make sure that all products within Radicle are
compatible.** This means that any projects that your team is collaborating on
will be available in the Upstream client, the `rad-cli`, and the Radicle [web
client](https://app.radicle.network).

Be sure to check out our [new onboarding docs](https://www.notion.so/Onboarding-Docs-ebf2b8a25e7a4ca98a626825e1a6f33c)
that explain how to get set up using Radicle for your project!

### 🎉 Notable Changes

#### Temporary removal of Organizations

You’ll notice that the Orgs tab has disappeared. That’s because **we’ve
temporarily hidden the tab on the UI**. Don’t worry, though, your organizations
are still there and can be found on our [web client](https://app.radicle.network/).
The reason we decided to remove it for the time being on the Upstream client is
that not only was it causing stability problems, but we want to **make sure
it’s actually useful for you**. In the next development cycle, our team will be
compiling your feedback to make sure that once we implement organizations
again, we also include valuable features.

#### Temporary removal of Ethereum integration

We’ve also **temporarily removed the Ethereum integration from Upstream**.
Similar to the organization issues, we’ve also run into a few problems with the
current Ethereum implementation. Thus, we’ve decided to remove it for the time
being. Along with the Organizations, we’ll also be going back to the drawing
board to make sure that what we implement will be useful for everyone.

**Note:** We know some of you that have created organizations and integrated
with Ethereum have spent money. You should have already been compensated for
any transaction fees you’ve spent. If not, please reach out to us on
[Discord](https://canary.discord.com/channels/841318878125490186/843873418205331506)
and we’ll make sure you’re reimbursed.

#### New onboarding flow

We’ve **greatly improved our app onboarding flow**. Not only is it much more
friendly, but we also perform a number of system checks to make sure your
environment is all ready to go. The checks the `rad-cli`, `upstream-cli`, and
Rad ID. That way we can make sure that you’re able to use all of the features
in Radicle. Further, we’ve also integrated with your `ssh-agent` so your
password only needs to be input once per session. Subsequent app opens will
detect your Radicle ID and automatically unlock.

#### Network compatibility

We’ve ensured that the Upstream client works with all the other tools in the
Radicle suite, including the `rad-cli` and Radicle web app. This means we’ve
**integrated the push/pull network** to be compatible in Upstream and **removed
the p2p network**. On the network screen, you’ll see that we’ve automatically
added 3 seeds to the client: Willow, Pine, and Maple. These are the three
default seeds run by Radicle. We are **currently working on re-implementing the
p2p network** to ensure decentralization; it’ll be available once we can ensure
system stability.


### [0.2.14](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.13...v0.2.14) (2022-02-22)


### Features

* add file diff collapse button ([76e1e1e](https://github.com/radicle-dev/radicle-upstream/commit/76e1e1e466d295f1be9f59171eee195468dbd374))
* allow test/development/production envs to run side-by-side ([49afcb8](https://github.com/radicle-dev/radicle-upstream/commit/49afcb8b62d08ebdc246f51c6c58c52aa6de2257))
* link to patch via custom protocol ([c67696d](https://github.com/radicle-dev/radicle-upstream/commit/c67696dfdd11b34509c4fad4ae898a4b62d0217e))
* add upstream cli with patch support ([42337e9](https://github.com/radicle-dev/radicle-upstream/commit/42337e9963c0ec765d57f3e916e61b55ea1a99cd))


### Bug Fixes

* 404 broken link on wallet screen ([#2711](https://github.com/radicle-dev/radicle-upstream/issues/2711)) ([ba7f0c0](https://github.com/radicle-dev/radicle-upstream/commit/ba7f0c06882634934052bbb9f9031db829cac982))
* announcements not reliable on seed ([1b3b8ba](https://github.com/radicle-dev/radicle-upstream/commit/1b3b8ba6b9f2d9485bf0684d583a15e3155eac06))
* clear badge text on peer selector change ([69416ce](https://github.com/radicle-dev/radicle-upstream/commit/69416ce69a1ba311b0444f0fabcb8f319b0c4b43))
* custom protocol not triggering action on cold start ([f2ebb90](https://github.com/radicle-dev/radicle-upstream/commit/f2ebb90860ad0b5de7a99305567e367b1ba83d84))
* fix patch tests ([4cd9567](https://github.com/radicle-dev/radicle-upstream/commit/4cd95673e985247d3a44a5a61161c9feb8f4e542))
* prevent commit teaser overflow in project source screen ([0c4ab3d](https://github.com/radicle-dev/radicle-upstream/commit/0c4ab3d996a9517bba24d7cb73d46f03f32e47d6))
* fix typos on docs ([77ce3ea](https://github.com/radicle-dev/radicle-upstream/commit/77ce3eade2fc26c73e35e00fbe7edc2c747cb5e3))
* update to the latest radicle-source/surf ([9378362](https://github.com/radicle-dev/radicle-upstream/commit/9378362cbdf5d9170fa71022bd8f712becc17c05))
* properly open URLs in AppImage ([614a499](https://github.com/radicle-dev/radicle-upstream/commit/614a499472427d42e226e6129a08eb8bd8bc1a5e))
* fix crash on second instance invocation ([169be50](https://github.com/radicle-dev/radicle-upstream/commit/169be500e7bbf3f1a100b874f22a7f69efc04786))
* parse by consuming string instead of regex ([574c544](https://github.com/radicle-dev/radicle-upstream/commit/574c54496714a957797588d6a071a667d9c22eec))

### [0.2.13](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.12...v0.2.13) (2021-12-17)


### Features

* add proxy logs tab to diagnostics screen ([b8233e1](https://github.com/radicle-dev/radicle-upstream/commit/b8233e1a1d51c9a0e1818dfbf802373bfc0ee73c))
* move attestation widget beneath wallet balance ([ba1ec00](https://github.com/radicle-dev/radicle-upstream/commit/ba1ec00afaed48848b242051dc95c5ca3c1ee5dc))


### Bug Fixes

* add default bg color for image avatars with transparency ([599179f](https://github.com/radicle-dev/radicle-upstream/commit/599179fcf27552c287a9a5a67334c3040785f36b))
* clear seed input after adding seed ([0ce1de7](https://github.com/radicle-dev/radicle-upstream/commit/0ce1de729316edeb1ca85aa94a828b4f93a5e51f))
* fix multisig org anchor buttons ([834f9f2](https://github.com/radicle-dev/radicle-upstream/commit/834f9f28061a8972d7b35edd9d7390e39a011951))
* show newly created org only after Gnosis API picks it up ([0ce7ff4](https://github.com/radicle-dev/radicle-upstream/commit/0ce7ff48bfea45e497d0b03d6a4bc6ff22a6cb0d))
* use the Graph for Gnosis Safe queries ([7342afe](https://github.com/radicle-dev/radicle-upstream/commit/7342afe0802ce06417b70ccbcd34a4273770eb99))

### [0.2.12](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.11...v0.2.12) (2021-12-01)


### Features

* add isolated p2p network tests ([755c20d](https://github.com/radicle-dev/radicle-upstream/commit/755c20d9158d231604ade95523c35139865d192b))
* add peer membership to diagnostics screen ([3970130](https://github.com/radicle-dev/radicle-upstream/commit/3970130edefd87e1212a64551b7d646fbd8e7c45))
* add proxy CLI option to unseal key for development ([7e5125a](https://github.com/radicle-dev/radicle-upstream/commit/7e5125a6d9139a3c53b9352f4d8f1ea6931d228e))
* add switch to disable HTTP API auth token check ([a56ad1e](https://github.com/radicle-dev/radicle-upstream/commit/a56ad1e2fac831db3168bf7f676d2f3d2fbab323))
* add user hovercard ([3696a79](https://github.com/radicle-dev/radicle-upstream/commit/3696a7930af35e81bf8c712708a0f408d3fb19dc))
* allow specifying key file via a CLI option ([cc6ddff](https://github.com/radicle-dev/radicle-upstream/commit/cc6ddfff1ade77c66f674bb7e6521627e240fa33))
* decouple design-system from Upstream ([23c2fe9](https://github.com/radicle-dev/radicle-upstream/commit/23c2fe9fc1925cae8572ba91cf99bcc2b9ece7a2))
* enable CI builds for macOS on Github Actions ([02b827c](https://github.com/radicle-dev/radicle-upstream/commit/02b827c84fd2e317b5e8d1339f3649bf1268a503))
* implement "go back" hotkey ([a046a20](https://github.com/radicle-dev/radicle-upstream/commit/a046a201e5c3dcfeda0a70d87ec0b1584860bb65))
* improve diagnostics screen ([b6e607f](https://github.com/radicle-dev/radicle-upstream/commit/b6e607f3b81d0dd2cd242d28ee94882f53512350))
* improve search modal ([0a2ce83](https://github.com/radicle-dev/radicle-upstream/commit/0a2ce837b4c5d5db39350a7bb4078b68239063a2))
* make design-system deployable to Github pages ([5883fe5](https://github.com/radicle-dev/radicle-upstream/commit/5883fe545ba6e00b4b7c1b9453899aa970687916))
* org explore screen ([#2450](https://github.com/radicle-dev/radicle-upstream/issues/2450)) ([574aac6](https://github.com/radicle-dev/radicle-upstream/commit/574aac648c7f2ad78c4c3c2942b2104b45d98b82)), closes [#2460](https://github.com/radicle-dev/radicle-upstream/issues/2460) [#2471](https://github.com/radicle-dev/radicle-upstream/issues/2471)
* properly shutdown proxy when app is closed ([23f43b8](https://github.com/radicle-dev/radicle-upstream/commit/23f43b88b40492728670d482bc9f40a4a22543d2))
* show loading animation on project screen ([e5f7753](https://github.com/radicle-dev/radicle-upstream/commit/e5f775332520a63d5c9894775a9b311b2755d5ae))
* show user hovercard on PeerSelector hover ([904eb43](https://github.com/radicle-dev/radicle-upstream/commit/904eb432a575ee2a0e0448c9bd12c76cc3d59857))
* store dismissed notifications ([db4c2a8](https://github.com/radicle-dev/radicle-upstream/commit/db4c2a8c29d0fe266738661838e33f4e6cc7587f))
* use rad-identities to get person information ([8deb07d](https://github.com/radicle-dev/radicle-upstream/commit/8deb07d235ebeff09ad4e506702053d8a3e16ca3)), closes [#2432](https://github.com/radicle-dev/radicle-upstream/issues/2432)
* view org when not connected to a wallet ([9740d4b](https://github.com/radicle-dev/radicle-upstream/commit/9740d4b391321d82358ce43ec5bd443df3a73c9e))


### Bug Fixes

* disable yarn log grouping on CI ([238fa1d](https://github.com/radicle-dev/radicle-upstream/commit/238fa1d7286f9156940991793ddd7f5b3daa1798))
* don't show diagnostics on devtools hotkey ([d63b1e0](https://github.com/radicle-dev/radicle-upstream/commit/d63b1e0b287b1ae964396268c1b541f29784e13b))
* force kill test nodes ([cad4e12](https://github.com/radicle-dev/radicle-upstream/commit/cad4e1204f2a7071a897618525d89324676e82d6))
* ignore cargo-deny advisories ([abea957](https://github.com/radicle-dev/radicle-upstream/commit/abea957eca6585334fcca770f21664f4702560cd))
* ignore RUSTSEC-2020-0071 ([10c3053](https://github.com/radicle-dev/radicle-upstream/commit/10c3053af7c97043114008cbcb972762d57ca650))
* reduce spec flakiness ([a9983f1](https://github.com/radicle-dev/radicle-upstream/commit/a9983f1e5fc4246f9a049685b70a664b7125c36b))
* relative positioning for tooltips ([20136cf](https://github.com/radicle-dev/radicle-upstream/commit/20136cf890bfb32bc6eafe320e28dd4a3ceccfcb))
* show commit not found screen instead of error ([0619c51](https://github.com/radicle-dev/radicle-upstream/commit/0619c51c5a80512a07fa234e60f69996aeba96d6))
* show handle from metadata if none is passed via props ([b3e33e8](https://github.com/radicle-dev/radicle-upstream/commit/b3e33e8538c81cff64c9b8deda87560e2b713c89))
* use new URL layout for Gnosis Safe deep links ([15c1479](https://github.com/radicle-dev/radicle-upstream/commit/15c1479e41aaaf9a1b49ad27f820368d2870391f))
* use recommended lang tag to avoid warning ([324bbb2](https://github.com/radicle-dev/radicle-upstream/commit/324bbb225f2c99496fa15067ecc01bbc49f1016c))
* use upstream container registry ([f16ca79](https://github.com/radicle-dev/radicle-upstream/commit/f16ca79f4fbb4216d1ba133bc789078121041295))

### [0.2.11](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.10...v0.2.11) (2021-09-29)


### Features

* add dedicated endpoint for network seeds ([d846f10](https://github.com/radicle-dev/radicle-upstream/commit/d846f10e1b3afd63d1cc56de19d10b50059da9a8)), closes [#1137](https://github.com/radicle-dev/radicle-upstream/issues/1137)
* add experimental windows support ([dd1eeda](https://github.com/radicle-dev/radicle-upstream/commit/dd1eedadebfb17b1e3dbfe313c49bff26592a300))
* add primary font color to settings ([86f9a60](https://github.com/radicle-dev/radicle-upstream/commit/86f9a60a6180c21a9fd512860702f154725569e4))
* add tests for ethereum attestation ([f178d4b](https://github.com/radicle-dev/radicle-upstream/commit/f178d4b2c6c003e873b914f7294ef6fc195c4481))
* add validated browser storage abstraction ([55abc41](https://github.com/radicle-dev/radicle-upstream/commit/55abc4177503331d14ef3f7a6699a4f9777b03d6))
* implement signMessage for test wallet ([3de455b](https://github.com/radicle-dev/radicle-upstream/commit/3de455bc2347034e6fd783f1c7103f9d915baa70))
* improve DesignSystem guide ([e972998](https://github.com/radicle-dev/radicle-upstream/commit/e9729984c9e7c9ca80951f23f436af78fb5d0d0f))
* improve keyboard shortcuts ([2a74775](https://github.com/radicle-dev/radicle-upstream/commit/2a74775f617fe467152e1fa4adcd282f5480e85d))
* improve org sidebar ux ([2c15838](https://github.com/radicle-dev/radicle-upstream/commit/2c15838fca61bc3cd1397f972b49d360a53177e8))
* integrate org ENS name registration ([3fd6112](https://github.com/radicle-dev/radicle-upstream/commit/3fd6112bde2111a723ba478374f78c143df87079))
* show anchors on project page ([b028ed4](https://github.com/radicle-dev/radicle-upstream/commit/b028ed4834f9fc287ffe829ed5a5de12208bf07a))
* show notification when route loading takes long ([f14121c](https://github.com/radicle-dev/radicle-upstream/commit/f14121cbc8366d13f18728832f9166f4bafb9589))


### Bug Fixes

* cache claimed identities for one minute ([b0b59a3](https://github.com/radicle-dev/radicle-upstream/commit/b0b59a36369598479bbc1642a972f468af3ec940))
* catch formatting errors in .ts files in pre-commit check ([b1ee6d9](https://github.com/radicle-dev/radicle-upstream/commit/b1ee6d96083cc75ef43d091de4036d20b9ffae28))
* cookie handling after electron and cypress upgrade ([5508f24](https://github.com/radicle-dev/radicle-upstream/commit/5508f24f77edb5fd67f962f1a61e1a77d58a9ff9)), closes [#1197](https://github.com/radicle-dev/radicle-upstream/issues/1197)
* disable failing patches test ([b871b77](https://github.com/radicle-dev/radicle-upstream/commit/b871b774353ca20ed8649fc0deb5352bae869849)), closes [#2308](https://github.com/radicle-dev/radicle-upstream/issues/2308)
* display token balance properly ([7f79ebd](https://github.com/radicle-dev/radicle-upstream/commit/7f79ebd8e1194e2616972f54c1d5933a0f382396)), closes [#2264](https://github.com/radicle-dev/radicle-upstream/issues/2264)
* don’t optimize webpack builds in development ([1d14cf4](https://github.com/radicle-dev/radicle-upstream/commit/1d14cf4799e82eb2b98bec918719f5a218329dc6))
* don’t swallow transaction error source ([3e45a15](https://github.com/radicle-dev/radicle-upstream/commit/3e45a15bf86951b98008c04076d8eadc01409786))
* fix prettier formatting ([ceb42bf](https://github.com/radicle-dev/radicle-upstream/commit/ceb42bf56517ef0850d77ccbde9e20d9a14b9009))
* fix regression from Modal update ([92da21a](https://github.com/radicle-dev/radicle-upstream/commit/92da21ae6ea043a384ca248dd3d6a3f2e66600c6))
* get rid of typescript warning ([413e0f8](https://github.com/radicle-dev/radicle-upstream/commit/413e0f887be68f3a6dd77f7bf2a265bdd9ac928e))
* handle wallet connect rejection ([f8daaec](https://github.com/radicle-dev/radicle-upstream/commit/f8daaeceef22c2b1c179f7d29feaa7a70b4dbaad))
* improve pending anchor message ([3b0b8bd](https://github.com/radicle-dev/radicle-upstream/commit/3b0b8bd1d5672ae90d4a7e66deab5772805fbb7e))
* increase timeout to unseal ([5895280](https://github.com/radicle-dev/radicle-upstream/commit/5895280be11ac26c9699187102cec7e9fb71729f)), closes [#2252](https://github.com/radicle-dev/radicle-upstream/issues/2252)
* make DirectoryInput text field open the file dialog ([1019cbd](https://github.com/radicle-dev/radicle-upstream/commit/1019cbdaaae2e7c08791caad06b43b372f5ad220))
* make sure all svelte files have the typescript lang tag ([242c565](https://github.com/radicle-dev/radicle-upstream/commit/242c565e050f19c4488990a5f4a049d054bd4054))
* network page scroll and button alignment ([e9979df](https://github.com/radicle-dev/radicle-upstream/commit/e9979dfd1c71ade69f4dd734ceeecdad46333592)), closes [#242](https://github.com/radicle-dev/radicle-upstream/issues/242)
* persist connection state of test wallet connect ([02a9329](https://github.com/radicle-dev/radicle-upstream/commit/02a93291ae0fdbb8fe797ea9342d97b5e62c2dee))
* reduce The Graph API request amount ([3e3c553](https://github.com/radicle-dev/radicle-upstream/commit/3e3c553dadcfa5db6b7afb44edbe196312b2ce57))
* reload org list in sidebar on wallet account switch ([bc964e1](https://github.com/radicle-dev/radicle-upstream/commit/bc964e1308c6c4dc935da167cfa02f7edf94587f))
* respond with 404 if project is not found ([8f39aa9](https://github.com/radicle-dev/radicle-upstream/commit/8f39aa9fb03af2874313fbf846149fa054ad47f9)), closes [#2427](https://github.com/radicle-dev/radicle-upstream/issues/2427)
* show passphrase button always visible ([c95c10a](https://github.com/radicle-dev/radicle-upstream/commit/c95c10a3b22788139b6882eb049292d96e589cb5))
* store appearance settings in browser ([be05d08](https://github.com/radicle-dev/radicle-upstream/commit/be05d080c4b3b3e52070583a8bbce4984627b50e)), closes [#2130](https://github.com/radicle-dev/radicle-upstream/issues/2130) [#1849](https://github.com/radicle-dev/radicle-upstream/issues/1849)
* store remote helper hint data locally ([074bcc8](https://github.com/radicle-dev/radicle-upstream/commit/074bcc89dff62f1c1dab464414acbca285580778)), closes [#1849](https://github.com/radicle-dev/radicle-upstream/issues/1849)
* try to avoid CI race conditions in networking tests ([2ce35ae](https://github.com/radicle-dev/radicle-upstream/commit/2ce35aef3158f13a31d117502648a09c7a61fddf))
* type component props consistently ([325a4b8](https://github.com/radicle-dev/radicle-upstream/commit/325a4b8db7c81c01d4ffd1b15f171ba2bee14666))
* update link to docs on how to obtain RAD token ([7ded359](https://github.com/radicle-dev/radicle-upstream/commit/7ded359776d3e2cad957897ba41accab46728714))
* update org ENS names in sidebar after registration ([77bb1d8](https://github.com/radicle-dev/radicle-upstream/commit/77bb1d808e18fe8ca4959a07ffd8f2e21a8d61bf))
* update release bucket for latest release ([1ee551b](https://github.com/radicle-dev/radicle-upstream/commit/1ee551b44ddbade6acce4146bf1362f770f25da2))
* use Gnosis Safe API instead of The Graph index ([2371bc9](https://github.com/radicle-dev/radicle-upstream/commit/2371bc920db89b282f44e001967e303a5ef4c8b1))
* use our own walletconnect bridge ([377dcf5](https://github.com/radicle-dev/radicle-upstream/commit/377dcf506a3264fa70c73df14697e0528be306eb))

### [0.2.10](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.9...v0.2.10) (2021-08-16)


### Features

* add `suffix` property to text input ([0f55cbd](https://github.com/radicle-dev/radicle-upstream/commit/0f55cbdfb18cc8e38fa21530b699360d06f6cda1)), closes [#2191](https://github.com/radicle-dev/radicle-upstream/issues/2191)
* add signTypedData to wallet connect ([0a54a05](https://github.com/radicle-dev/radicle-upstream/commit/0a54a0593b78b2a2d6e6d7d29f79a21c99f67223))
* show other user profiles ([0925202](https://github.com/radicle-dev/radicle-upstream/commit/09252027248c9db8b74675160b017ae05f5ef867))
* show RAD token balance on wallet screen ([2314723](https://github.com/radicle-dev/radicle-upstream/commit/231472330adc7c8397760e5ea75ae34951a6b582))


### Bug Fixes

* check all files for licenses by default ([a359dfc](https://github.com/radicle-dev/radicle-upstream/commit/a359dfcbd63abafe18ab6c5921485b2697aee7e1))
* don’t handle unhandled errors in cypress runner ([86aa571](https://github.com/radicle-dev/radicle-upstream/commit/86aa571f3d857763ee5341c6fc06c87a96769b77))
* fix dev commands for merging prs ([dfcfbac](https://github.com/radicle-dev/radicle-upstream/commit/dfcfbac0369609b0cdc1693f6573c0d3716610cd))
* fix race conditions in source browsing tests ([0c8de68](https://github.com/radicle-dev/radicle-upstream/commit/0c8de68d216edf43771b0379e986c1d622f51273))
* ignore security advisory that we’re unaffected by ([966d53e](https://github.com/radicle-dev/radicle-upstream/commit/966d53e9ba2b243f6a7dec818c3bf9820334329c))
* include more data in claim errors ([8080e29](https://github.com/radicle-dev/radicle-upstream/commit/8080e296d01a3a9af3744cf9c676757a4df7ba83))
* properly memoize `undefined` return value ([ae8fc76](https://github.com/radicle-dev/radicle-upstream/commit/ae8fc76d5b7e9e6319b5d0298e38ca788762de14))
* properly scale avatar images ([30f3bdc](https://github.com/radicle-dev/radicle-upstream/commit/30f3bdccb55dbfb3076e197025f4acbe9cb25ac4))
* remove WalletConnect signer defaults ([f1777c6](https://github.com/radicle-dev/radicle-upstream/commit/f1777c6dda25bab98c669af6bc872906c0685735))
* retry graph requests on 503 ([80ac00b](https://github.com/radicle-dev/radicle-upstream/commit/80ac00b5df24d1780fa1d88445dca9ad1ed50bf3)), closes [#2183](https://github.com/radicle-dev/radicle-upstream/issues/2183)
* show icon and website when connecting via walletConnect ([c377197](https://github.com/radicle-dev/radicle-upstream/commit/c377197a9aa5ce4aa549989868c7dd5fa5305147))
* silent retries if graph is unavailable ([f53d739](https://github.com/radicle-dev/radicle-upstream/commit/f53d73907a497a022faff07e1f95ddae46c70f7d))
* tests wait until keystore is sealed ([4b17c2e](https://github.com/radicle-dev/radicle-upstream/commit/4b17c2efec8edd4e144853be8b558295e298ffb9))
* update radicle-link to properly shutdown ([06bd982](https://github.com/radicle-dev/radicle-upstream/commit/06bd9826ac38f54151b70f408b8d5594affc2d29)), closes [#2133](https://github.com/radicle-dev/radicle-upstream/issues/2133)
* use proxy IP instead of hostname ([3766946](https://github.com/radicle-dev/radicle-upstream/commit/37669464dad7bd6598c7818e7fa0eddedf282d2a)), closes [#2181](https://github.com/radicle-dev/radicle-upstream/issues/2181)
* wait until session is unsealed ([0b491a2](https://github.com/radicle-dev/radicle-upstream/commit/0b491a2f4d4e9252935eb5d1b47074dd8c996ae8))
* **ui:** fix top and bottom tooltip alignment ([c5ffde9](https://github.com/radicle-dev/radicle-upstream/commit/c5ffde9e92bf4fcf8bb32096d6c29c28c2edc91f))

### [0.2.9](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.8...v0.2.9) (2021-07-23)


### Features

* **ui:** show (eye) icon next to passphrase fields ([e9f49e8](https://github.com/radicle-dev/radicle-upstream/commit/e9f49e86a8241908a677e05b2d22e7959d0170be)), closes [#2142](https://github.com/radicle-dev/radicle-upstream/issues/2142)


### Bug Fixes

* cache org data ([bdbb2f4](https://github.com/radicle-dev/radicle-upstream/commit/bdbb2f4ad3139fd08a09089ba856dc7b91377894))
* event based wallet connection for immediate response ([f50babc](https://github.com/radicle-dev/radicle-upstream/commit/f50babc192a7bceb96c99620b3490cf8e7f9d76d))
* reduce infura polled request amount ([6692732](https://github.com/radicle-dev/radicle-upstream/commit/669273290057ba28f035e3b4a89b256235b2d842))
* use tracing for logs ([dad7694](https://github.com/radicle-dev/radicle-upstream/commit/dad7694fa33e20d9a054e8994ca9c99ca88757fe))
* **tests:** forward `stdout` of test node to console ([044ebfa](https://github.com/radicle-dev/radicle-upstream/commit/044ebfaf1b58d98a35dbddf8ceb0cf6bf2c2008e))
* remove console.log debug statement ([99b49e0](https://github.com/radicle-dev/radicle-upstream/commit/99b49e0ed330234797a37bcc33cc9ec7e69ef99b))

### [0.2.8](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.7...v0.2.8) (2021-07-15)


### Bug Fixes

* ensure that a valid ethereum environment is selected ([3ef925a](https://github.com/radicle-dev/radicle-upstream/commit/3ef925afe2add1f3265b645746ff090307bbf8cc))

### [0.2.7](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.6...v0.2.7) (2021-07-15)


### Features

* remove orgs feature flagging ([9f66d66](https://github.com/radicle-dev/radicle-upstream/commit/9f66d667aa6e498f9fe7df39067bf04cb842a06c))
* **dev:** add cargo-deny ([3d591e2](https://github.com/radicle-dev/radicle-upstream/commit/3d591e2a20c242c44b77d1975dd7154aa90cccbc))
* **dev:** add contributing.md ([6b7fff6](https://github.com/radicle-dev/radicle-upstream/commit/6b7fff6b6342d56d6d6e167ce38aee9bc8f4807a))
* **dev:** check license compliance for NPM packages ([8b51ebd](https://github.com/radicle-dev/radicle-upstream/commit/8b51ebd3d0b88137ac15ed1d0e7dcc3a79436b73)), closes [#2024](https://github.com/radicle-dev/radicle-upstream/issues/2024)
* **dev:** fast key store for development ([4c3813b](https://github.com/radicle-dev/radicle-upstream/commit/4c3813bd4be99eff3ee1f8353a03e918c80950bf)), closes [#1614](https://github.com/radicle-dev/radicle-upstream/issues/1614)
* **dev:** improve contribution guide ([ec5215a](https://github.com/radicle-dev/radicle-upstream/commit/ec5215a0bc6524b5ddbf857e9f14ac44fd2cb8c6))
* **ui:** add Rinkeby testnet and Mainnet ([7d770da](https://github.com/radicle-dev/radicle-upstream/commit/7d770daf5fd8c946ca464fcb439c794deb2130c5))
* **ui:** network page ([#2066](https://github.com/radicle-dev/radicle-upstream/issues/2066)) ([8b1836c](https://github.com/radicle-dev/radicle-upstream/commit/8b1836c7610a2ef51af0abb1e731380bf2b4fa68))
* notifications can be removed now ([0b8c02f](https://github.com/radicle-dev/radicle-upstream/commit/0b8c02f179d1e559f2c93a1729ceebf73eb8ccce))
* radicle Orgs on Ethereum via Gnosis Safe ([d567444](https://github.com/radicle-dev/radicle-upstream/commit/d567444183e347e8fd68067b671323166c61cc05))
* **ui:** add selectedNetwork to wallet tooltip ([500d63b](https://github.com/radicle-dev/radicle-upstream/commit/500d63b013eb4de4b6d675ac1bd86e27aa750345))


### Bug Fixes

* add padding to copyable when icon is not shown ([08c2ed7](https://github.com/radicle-dev/radicle-upstream/commit/08c2ed76b1b362b1865d8fc42fe21dc2476165d6))
* adjust code to the updated Org Graph index ([9c84637](https://github.com/radicle-dev/radicle-upstream/commit/9c8463777a224268623649d794a1d1d51b388c92))
* commit teaser message overflow ([a5019dd](https://github.com/radicle-dev/radicle-upstream/commit/a5019dd42212c22f97a4c849c79d8c3f2cbb71a6))
* correct radicle-contracts types ([5518e4d](https://github.com/radicle-dev/radicle-upstream/commit/5518e4d33e7fd471b54e1370b8a926627d54eaca))
* disable placeholder anchor button when there are no projects ([d4ca5fb](https://github.com/radicle-dev/radicle-upstream/commit/d4ca5fb58d43e969a6c15a82e765529f5ddb5748))
* don’t show intermittent 502 errors from Graph requests ([38faba1](https://github.com/radicle-dev/radicle-upstream/commit/38faba1ea9b32aadb1bafa49b6b3a5517d473169)), closes [#2042](https://github.com/radicle-dev/radicle-upstream/issues/2042)
* enable option transition only when not active ([2487af1](https://github.com/radicle-dev/radicle-upstream/commit/2487af1f98c7af3e585cce94d9e91087b8fcbe53))
* explain that app needs to be restarted after reset ([a9ff111](https://github.com/radicle-dev/radicle-upstream/commit/a9ff111f914cca77e616b82ad6b4430bd1c90ea7)), closes [#2119](https://github.com/radicle-dev/radicle-upstream/issues/2119)
* fix wallet icon ([741a977](https://github.com/radicle-dev/radicle-upstream/commit/741a977a369394231af065964685dae2c7cbb5b3))
* improve org creation notification ([210bf22](https://github.com/radicle-dev/radicle-upstream/commit/210bf22a316bc2121906939b66a413b903527dc5))
* improved error message from server ([27c9737](https://github.com/radicle-dev/radicle-upstream/commit/27c9737983b61ab0d3f1f13c4715409038b66f3d))
* on invalid passphrase clear and refocus input ([dcade0d](https://github.com/radicle-dev/radicle-upstream/commit/dcade0d50a2f07da3842754aa7944ca1531dbce6)), closes [#1179](https://github.com/radicle-dev/radicle-upstream/issues/1179)
* project error ([64881cd](https://github.com/radicle-dev/radicle-upstream/commit/64881cdb332be49e8e6122368a532632337a060f)), closes [#1959](https://github.com/radicle-dev/radicle-upstream/issues/1959)
* race condition when watching claim updates ([c162fa3](https://github.com/radicle-dev/radicle-upstream/commit/c162fa395f56374fd11db029f1038c5148a0a429))
* reapply commit to truncate device ID in settings screen ([05a655d](https://github.com/radicle-dev/radicle-upstream/commit/05a655dee340313660e132457db255263774c981)), closes [#2087](https://github.com/radicle-dev/radicle-upstream/issues/2087)
* truncate device ID in settings screen ([c2cc531](https://github.com/radicle-dev/radicle-upstream/commit/c2cc5319fe5557475f0eaa48ac129942bc41276f))
* **ci:** check that packages are deduped ([ae00144](https://github.com/radicle-dev/radicle-upstream/commit/ae001448521fe08bab1b4e1362f4cc564e9e62f1))
* **ci:** force link override for cargo deny on CI ([ffd295e](https://github.com/radicle-dev/radicle-upstream/commit/ffd295e9ac79a4398e5841655e9e5d20c0d6ed13))
* **dev:** align parameter name with options name ([6be234b](https://github.com/radicle-dev/radicle-upstream/commit/6be234bcbca9db2c9e014d5a4602150e0566eb1b))
* **dev:** allow empty function lint ([99a5724](https://github.com/radicle-dev/radicle-upstream/commit/99a5724b9648e8bcefc560e6811e52d0b0e162cd))
* **dev:** avoid yarn run to properly kill process ([5697266](https://github.com/radicle-dev/radicle-upstream/commit/569726667a28c3b9f925083229e31246bc9351ba))
* **dev:** jest module name mapping for TS paths ([22f8afc](https://github.com/radicle-dev/radicle-upstream/commit/22f8afcc3fc6ddca41c7e1a8bbc0214190d72978))
* **test:** fix flaky networking test ([23814d1](https://github.com/radicle-dev/radicle-upstream/commit/23814d194078d72f13d9127e684b20947b2d9a13))
* remove tx notifications after confirmation ([3c916ef](https://github.com/radicle-dev/radicle-upstream/commit/3c916efe836351d580a053f3c6bf2798c94fc27e))
* use dedicated walletconnect bridge ([06701c5](https://github.com/radicle-dev/radicle-upstream/commit/06701c59b5ce61559d20037af8fa376edb23be3e))
* **dev:** allow more modules to be required in jest tests ([ddc0e51](https://github.com/radicle-dev/radicle-upstream/commit/ddc0e512d1b4c99182c90c5552da154a9940fc62))
* **ui:** use revision icon for patches ([aa60ed3](https://github.com/radicle-dev/radicle-upstream/commit/aa60ed3e90afebeb1e84e27daece46bdf200f5cc))
* prevent lock race condition ([edd0816](https://github.com/radicle-dev/radicle-upstream/commit/edd0816a39eb17af8509d76f03087f345ad13ad4))
* **tests:** properly select peer in networking tests ([f880856](https://github.com/radicle-dev/radicle-upstream/commit/f880856a4e15e185c103e4c9a9107fc65c1a582e))

### [0.2.6](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.5...v0.2.6) (2021-06-07)


### Features

* **ui:** add wallet status to wallet sidebar icon ([#1930](https://github.com/radicle-dev/radicle-upstream/issues/1930)) ([1d4b450](https://github.com/radicle-dev/radicle-upstream/commit/1d4b450fc796f17a60d60cd811cee0de556cb1f9))
* **ui:** Wallet update ([#1900](https://github.com/radicle-dev/radicle-upstream/issues/1900)) ([7ef2ec4](https://github.com/radicle-dev/radicle-upstream/commit/7ef2ec4c4f7bc1ce8e90e7f9b2a9667f3b58f138))
* patches ([dbf9d1b](https://github.com/radicle-dev/radicle-upstream/commit/dbf9d1b81dfae4b71fe308f4109c9e03bee98e7b))


### Bug Fixes

* don’t destroy wallet when all views unsubscribe ([d35b04d](https://github.com/radicle-dev/radicle-upstream/commit/d35b04d2d3d477d8a7cfcdd4e94ea3c7c3a010c1))
* don’t leak ethereum account polls ([972c000](https://github.com/radicle-dev/radicle-upstream/commit/972c000c5fb9e890f9c961e5b3549d4e1f1eeb70))
* **tests:** unload app when test is successful ([e92e498](https://github.com/radicle-dev/radicle-upstream/commit/e92e498a96d9fe0b96a2be82232d817af4785dfb))
* update patches screen when a new patch arrives ([924b747](https://github.com/radicle-dev/radicle-upstream/commit/924b7477a7afaadca45bde616b4d787a82715880))
* **ui:** fix network diagnostics shortcut ([c697515](https://github.com/radicle-dev/radicle-upstream/commit/c6975154ca5280d6846a09ec01b5001620a13199))
* show peer count properly in sidebar ([5205959](https://github.com/radicle-dev/radicle-upstream/commit/5205959925c6e776b9464d4b9c047ab1e58fec7d))

### [0.2.5](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.4...v0.2.5) (2021-05-25)


### Bug Fixes

* remote status indicator vanishing ([ea93a31](https://github.com/radicle-dev/radicle-upstream/commit/ea93a31eb16642d7de9d0ca8ba6cec2bc072780c))
* **ci:** force removal of cache directory on cleanup ([7216e34](https://github.com/radicle-dev/radicle-upstream/commit/7216e3434b7ccd571f415739a2c1702a46883618))
* **ci:** improved svelte/typescript linting ([535abce](https://github.com/radicle-dev/radicle-upstream/commit/535abce5c4dee4a353e06882da083c3cdf8627b1)), closes [/github.com/sveltejs/eslint-plugin-svelte3/blob/master/CHANGELOG.md#310](https://github.com/radicle-dev//github.com/sveltejs/eslint-plugin-svelte3/blob/master/CHANGELOG.md/issues/310)
* **dev:** don’t cache latest release information ([6a86f4d](https://github.com/radicle-dev/radicle-upstream/commit/6a86f4d86cb2dbd15037d403b06af75639bc97cb))
* **dev:** slim down release QA script ([d6e1c29](https://github.com/radicle-dev/radicle-upstream/commit/d6e1c296f4c954192065a8517f3087201d385503))
* **proxy:** update tests to work with latest git-platinum ([a1de9e5](https://github.com/radicle-dev/radicle-upstream/commit/a1de9e5e53d8eb04018fe3744300b757841ce60d))

### [0.2.4](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.3...v0.2.4) (2021-05-04)


### Features

* **ui:** improve error reporting for parse failures ([52a5114](https://github.com/radicle-dev/radicle-upstream/commit/52a5114b31fff6225259346306375aca44caa7dc))


### Bug Fixes

* **dev:** implement ethereum dev node in typescript ([57a15d5](https://github.com/radicle-dev/radicle-upstream/commit/57a15d5e7250380fd6e6531c434a3b05a63e7ce5))
* **ui:** more robust session context ([29a4545](https://github.com/radicle-dev/radicle-upstream/commit/29a45459948dca38dd0af97c452be064396949d6))
* **ui:** properly handle failed project ([1352f61](https://github.com/radicle-dev/radicle-upstream/commit/1352f61c8c629a31bba2dcfe625818d3bf149287))

### [0.2.3](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.2...v0.2.3) (2021-04-28)


### Bug Fixes

* **proxy:** qualify announcement ([72c54d2](https://github.com/radicle-dev/radicle-upstream/commit/72c54d22a4a159f073ceef4ec4b3d6a866a547fc))
* **ui:** display branch a commit belongs to ([0141921](https://github.com/radicle-dev/radicle-upstream/commit/0141921168c80d946422d93882a6522b5ed2506a)), closes [#1717](https://github.com/radicle-dev/radicle-upstream/issues/1717) [#1798](https://github.com/radicle-dev/radicle-upstream/issues/1798) [#1717](https://github.com/radicle-dev/radicle-upstream/issues/1717) [#1793](https://github.com/radicle-dev/radicle-upstream/issues/1793)

### [0.2.2](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.1...v0.2.2) (2021-04-27)


### Features

* **ci:** collapse cypress tests in CI logs ([3aad194](https://github.com/radicle-dev/radicle-upstream/commit/3aad1946a5ff813a1a49b7d0f0a740b2dd789919))
* **test:** store test node logs in file ([2137e6c](https://github.com/radicle-dev/radicle-upstream/commit/2137e6c320c0d13d02625d1d958cb94e463b488a))
* **ui:** improve error handling ([e483582](https://github.com/radicle-dev/radicle-upstream/commit/e483582a7da637d02f8e8c015ed8e4a5d53ac8f7))


### Bug Fixes

* **proxy:** properly decode repo paths on project creation ([c6d2fcb](https://github.com/radicle-dev/radicle-upstream/commit/c6d2fcbbc8c1693143d9cd52091cc8606a563532)), closes [#1806](https://github.com/radicle-dev/radicle-upstream/issues/1806)
* **tests:** eliminate unnecessary waits ([dd253a0](https://github.com/radicle-dev/radicle-upstream/commit/dd253a0590b515fdc4f868d5d9ba7ee62c4acf70))
* **tests:** use explicit assertions for retry ([2ae60b6](https://github.com/radicle-dev/radicle-upstream/commit/2ae60b6861e057339bc25ff07c07f906f5c64988))
* **ui:** clear search input after every search ([cb4a387](https://github.com/radicle-dev/radicle-upstream/commit/cb4a387ffaf9b03042a836299aa33ed0f325fe57))
* **ui:** fix deep-links on macOS ([eaaf4af](https://github.com/radicle-dev/radicle-upstream/commit/eaaf4afcf116bdf3a433b3cd0c13f246d09b1522))
* **ui:** replace `path.active` ([be98c75](https://github.com/radicle-dev/radicle-upstream/commit/be98c75fdacef413ce4af7c655b9acd4d5dbf21e))
* **ui:** search enter key follow only when fetched ([7872159](https://github.com/radicle-dev/radicle-upstream/commit/78721592250bd33cf82ee0a822eccfddd6e8c8bd))
* **ui:** validate event payload and fix fields ([f104765](https://github.com/radicle-dev/radicle-upstream/commit/f1047653a2a2aba287311b220f22a2bd765f0d19))

### [0.2.1](https://github.com/radicle-dev/radicle-upstream/compare/v0.2.0...v0.2.1) (2021-04-12)


### Features

* **ui:** handle "enter" hotkey in search modal ([b8c4ad4](https://github.com/radicle-dev/radicle-upstream/commit/b8c4ad4afb781342c3446beebf57ee36e3f42b35))
* **ui:** open Upstream via radicle:// ([#1652](https://github.com/radicle-dev/radicle-upstream/issues/1652)) ([1bcd941](https://github.com/radicle-dev/radicle-upstream/commit/1bcd9412fb5682cb4c37c90f663910d31bbb30bb))


### Bug Fixes

* **tests:** wait for test project to be created ([4b93e75](https://github.com/radicle-dev/radicle-upstream/commit/4b93e75d407be4093f0dde505171bcb1e4398dbf))
* **ui:** parse ethereum field of identity in ui ([aefbaf4](https://github.com/radicle-dev/radicle-upstream/commit/aefbaf405044f2d515f6468bc690f7c5e0e528a4)), closes [#1756](https://github.com/radicle-dev/radicle-upstream/issues/1756) [#1712](https://github.com/radicle-dev/radicle-upstream/issues/1712)
* **ui:** show an error when a project is missing peers ([9545069](https://github.com/radicle-dev/radicle-upstream/commit/95450696c21235d8cd33597a895808f4c1a3ac12))

## [0.2.0](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.13...v0.2.0) (2021-04-08)


### Features

* **ci:** enable Github Actions for external PRs and master ([653337f](https://github.com/radicle-dev/radicle-upstream/commit/653337f6e5bc7028f2f6b6091d105526143b22c5)), closes [#1471](https://github.com/radicle-dev/radicle-upstream/issues/1471)
* **proxy:** add PUT identity metadata endpoint ([f59c80b](https://github.com/radicle-dev/radicle-upstream/commit/f59c80b17d0eebd129b8e0b404052e35f1c1150b))
* **proxy:** bump link dependencies ([cf70496](https://github.com/radicle-dev/radicle-upstream/commit/cf704961c91ba1c279e3e0ed454ddcf44dea448e))
* **test:** reusable `nodeManager.exec` command ([3741682](https://github.com/radicle-dev/radicle-upstream/commit/3741682828c077675ae060ffc537c144c6073ba4))
* **ui:** add ClaimsContract ([c2f7ed2](https://github.com/radicle-dev/radicle-upstream/commit/c2f7ed2ccf5263c8ae9f58a52e2e2b00d78ce4df))
* **ui:** add new h4xor color theme ([e097dd7](https://github.com/radicle-dev/radicle-upstream/commit/e097dd715f18ae641b574e669d59c56f07a4ec96))
* **ui:** design improvements from merge request work ([29703cd](https://github.com/radicle-dev/radicle-upstream/commit/29703cd8092b8420211960266b82d3dd8a5d1e4b))
* **proxy**: raidcle-upstream upgrades involving radicle-link's `next` changes ([f0081b3](https://github.com/radicle-dev/radicle-upstream/commit/f0081b33bca6612fc7dbbea416a04e6f90c80b48))
* **proxy**: log timestamps ([3f6bcc7](https://github.com/radicle-dev/radicle-upstream/commit/3f6bcc7b9e45711d04a52aea7cdf9bf86cd87409))


### Bug Fixes

* **build:** fix macOS notarization bundle identifier ([a0ed66e](https://github.com/radicle-dev/radicle-upstream/commit/a0ed66e30fdf0182b52db5e7030968ea9abf871b))
* **build:** fix test:integration:debug script ([ddefd6f](https://github.com/radicle-dev/radicle-upstream/commit/ddefd6fb3959245099c4afcfbab8ab9021a5b2e3))
* **build**: add 'Buffer' to webpack.ProvidePlugin list ([938f360](https://github.com/radicle-dev/radicle-upstream/commit/938f360666e3592c5e84cc2452b729d9d63cfe75))
* **ci:** automatically clean cache when it is almost full ([6ec277f](https://github.com/radicle-dev/radicle-upstream/commit/6ec277fafc12fd5d5f1ab7287240a1d3e749833d))
* **ci:** don’t clean free cache on mac agent ([1e02a7d](https://github.com/radicle-dev/radicle-upstream/commit/1e02a7d0bb278f6f39e2d14d87fe42169952bc12)), closes [#1707](https://github.com/radicle-dev/radicle-upstream/issues/1707)
* **ci:** only run github ci for external contributors ([7bded01](https://github.com/radicle-dev/radicle-upstream/commit/7bded011b8ae8ae94332b5ab7abc6b526bd61773))
* **ci:** replace deprecated rustdoc lint ([560a941](https://github.com/radicle-dev/radicle-upstream/commit/560a941622484a460a281d2f7e872c9c48e79ba1))
* **ci:** use new ci script on mac os agent ([2d343f2](https://github.com/radicle-dev/radicle-upstream/commit/2d343f23de59650c97820d0bead065140bafb1a1))
* **fmt:** replace deprecated rustfmt option ([7ddb73a](https://github.com/radicle-dev/radicle-upstream/commit/7ddb73a5c509def928e8f4dd6a7e854d01c417ed))
* **ui:** don't leak store subscriptions ([99c55b6](https://github.com/radicle-dev/radicle-upstream/commit/99c55b6e79574e84b26cf707e0c7c042994a60d4))
* **ui:** wait for process streams to end ([7b6340d](https://github.com/radicle-dev/radicle-upstream/commit/7b6340ded0892a3f83c627a0d7c03da41fed61b1)), closes [#1409](https://github.com/radicle-dev/radicle-upstream/issues/1409)
* make reset script backwards compatible ([7be2cb4](https://github.com/radicle-dev/radicle-upstream/commit/7be2cb49a0a3e60af2bd4d1b0d8c446d5a96114d))

### [0.1.13](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.12...v0.1.13) (2021-03-23)


### Bug Fixes

* **build:** don't crash app on typescript error ([183fba5](https://github.com/radicle-dev/radicle-upstream/commit/183fba53c8b6de47325551530c95906fcd4ff1dd))
* **build:** fix bundling of electron main files ([f316539](https://github.com/radicle-dev/radicle-upstream/commit/f316539412f552e9576a042886a2e3117b6c06dc))
* **ci:** run-p --race kills process ([c8db022](https://github.com/radicle-dev/radicle-upstream/commit/c8db022c25c5fd924ba3f2ca15a173d392f57238))
* **test:** isolate git config in cypress tests ([4a93462](https://github.com/radicle-dev/radicle-upstream/commit/4a9346259295a40fd3a266a7f622aa299f896d80)), closes [#1631](https://github.com/radicle-dev/radicle-upstream/issues/1631)
* **tests:** cypress tests wait until bundle is build ([273a295](https://github.com/radicle-dev/radicle-upstream/commit/273a2953e42ad64756f6618427f80fd7fc5b1a20))
* **ui:** enable Electron context isolation ([785f5d5](https://github.com/radicle-dev/radicle-upstream/commit/785f5d5afeeda67b760e29d7ff5c7317ea31d73b)), closes [#1307](https://github.com/radicle-dev/radicle-upstream/issues/1307)
* **ui:** Fix waitUnsealed when session already unsealed ([1c1b63c](https://github.com/radicle-dev/radicle-upstream/commit/1c1b63c7a739773f5d3a77349efcb42edbbacc90))
* **ui:** remove unnecessary call to path.resolve ([2a83d47](https://github.com/radicle-dev/radicle-upstream/commit/2a83d473fd0000ea02630d8628f8d713dee6a00d))

### [0.1.12](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.11...v0.1.12) (2021-03-09)


### Bug Fixes

* **ui:** close any open modal on "Go to settings" ([f9124db](https://github.com/radicle-dev/radicle-upstream/commit/f9124dbdefa6cbae9faeba6f4e62d12008cc09bc))
* set macOS minimumSystemVersion to 10.14 ([bed30a8](https://github.com/radicle-dev/radicle-upstream/commit/bed30a88b39a3463f65d060295cd9393127a931d))


### [0.1.11](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.10...v0.1.11) (2021-02-24)


### Features

* **ui:** click on project name to go to root view ([eb0d62b](https://github.com/radicle-dev/radicle-upstream/commit/eb0d62bd71ac47a3d0492000b72afa683ffcfc7c))
* **ui:** pick global git default branch ([1ad2ae3](https://github.com/radicle-dev/radicle-upstream/commit/1ad2ae34d8ebe97bef201f6fde5b63ee57b9b10e))
* **ui:** preselect main or master on import ([e9bac61](https://github.com/radicle-dev/radicle-upstream/commit/e9bac61c9809ebb95bf86dbce43dd3a0a9782baa))
* **ui:** support Ropsten as a funding environment ([12b8926](https://github.com/radicle-dev/radicle-upstream/commit/12b89267ad45d756733163cd0cf6175de351e5a6))


### Bug Fixes

* **infra:** check minimum node version ([79420ab](https://github.com/radicle-dev/radicle-upstream/commit/79420abcf662454692ad4b63491bfa4b42384a17))
* **tests:** fix flaky version tests ([33a54ac](https://github.com/radicle-dev/radicle-upstream/commit/33a54ac7ad590003fab75c0b73e64aefb31ee219))
* **ui:** more permissive seed validation ([6ee2c53](https://github.com/radicle-dev/radicle-upstream/commit/6ee2c53150b9441a8a9f37e6fbcf55069f387a71)), closes [#1618](https://github.com/radicle-dev/radicle-upstream/issues/1618)
* **ui:** reinit walletConnect dismissed modal ([1c49788](https://github.com/radicle-dev/radicle-upstream/commit/1c49788142317ad94da3c3aa3187586d397b6f4d))

### [0.1.10](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.9...v0.1.10) (2021-02-10)


### Bug Fixes

* **test:** properly assert in tests ([bdf8322](https://github.com/radicle-dev/radicle-upstream/commit/bdf83223e1e5281dec173d6479308f25dd29dfd1))
* handle disconnecting wallet-side ([1747a8e](https://github.com/radicle-dev/radicle-upstream/commit/1747a8e124f9638688dc484fe92e6fb48f744235))
* **ui:** correct msg on copying Transaction hash ([9562130](https://github.com/radicle-dev/radicle-upstream/commit/95621304854c4094cc7121764fd47546535d0385))
* **ui:** let the transaction center be closed ([08051ef](https://github.com/radicle-dev/radicle-upstream/commit/08051efc75fb25cf0100177cf61ed52e181f5276))

### [0.1.9](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.8...v0.1.9) (2021-01-27)


### Features

* **ui:** implement app updates ([52d1bcd](https://github.com/radicle-dev/radicle-upstream/commit/52d1bcdb58b6f4604c666447e55ba12663ca2ba0))
* **ui:** improve notifications ([ec11c40](https://github.com/radicle-dev/radicle-upstream/commit/ec11c40dfc587e72ba175695532b98ce5f61d3ed))
* **ui:** add [you] badge to Peer representations ([d787fb0](https://github.com/radicle-dev/radicle-upstream/commit/d787fb07d1512489e193fa528749cc22be938981)), closes [#1391](https://github.com/radicle-dev/radicle-upstream/issues/1391)


### [0.1.8](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.7...v0.1.8) (2021-01-21)


### Features

* add support to create all directories under RAD_HOME ([c04dc4b](https://github.com/radicle-dev/radicle-upstream/commit/c04dc4b24eff0da1b32addc76170e5dd56fb6bd2))


### Bug Fixes

* **ui:** improve and fix funding-related bits ([dc8b347](https://github.com/radicle-dev/radicle-upstream/commit/dc8b347b4c0656434daf029967dd4809bfef1531))
* **ui:** resolve visual regressions ([2444e9c](https://github.com/radicle-dev/radicle-upstream/commit/2444e9c5af4c53775f0259df0c3b1d6d04b69e32))

### [0.1.7](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.6...v0.1.7) (2021-01-20)


### Features

* **ui:** Funding Pool v0 ([5b4a327](https://github.com/radicle-dev/radicle-upstream/commit/5b4a327c9eddcb93b37c8c67cbf82c096dc74e5f)), closes [#974](https://github.com/radicle-dev/radicle-upstream/issues/974) [#1493](https://github.com/radicle-dev/radicle-upstream/issues/1493) [#1314](https://github.com/radicle-dev/radicle-upstream/issues/1314)
* **ui:** label default branch in RevisionSelector ([#1503](https://github.com/radicle-dev/radicle-upstream/issues/1503)) ([2224157](https://github.com/radicle-dev/radicle-upstream/commit/2224157f952ef2a5dcaf246602987eb345a6875f))


### Bug Fixes

* **ui:** fix window before ready ([8a6f07f](https://github.com/radicle-dev/radicle-upstream/commit/8a6f07fdddb203cedd2f845dcc4ca35464559488))
* **ui:** preserve NewProject Modal layout ([f41a759](https://github.com/radicle-dev/radicle-upstream/commit/f41a7599dfe6603627120c32be3fc5f19d7228e9))
* **ui:** use alternative support channel link [#1511](https://github.com/radicle-dev/radicle-upstream/issues/1511) ([8bbf461](https://github.com/radicle-dev/radicle-upstream/commit/8bbf4614a9a547961f99e994237d847cad07a2b3))

### [0.1.6](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.5...v0.1.6) (2021-01-05)


### Bug Fixes

* **ui:** make logo text color work in dark/light mode ([#1491](https://github.com/radicle-dev/radicle-upstream/issues/1491)) ([34e09fe](https://github.com/radicle-dev/radicle-upstream/commit/34e09fe2721e3c329c3827c0db9a718269491edb))
* **ui:** fix list border overflow ([#1467](https://github.com/radicle-dev/radicle-upstream/issues/1467)) ([ec227a4](https://github.com/radicle-dev/radicle-upstream/commit/ec227a4e8878cb6fa7ab9905f0ecadb43de08a17)), closes [#1447](https://github.com/radicle-dev/radicle-upstream/issues/1447)
* **proxy:** fix proxy crash due to waitingroom bug ([#1500](https://github.com/radicle-dev/radicle-upstream/pull/1500)) ([3f0c7d9](https://github.com/radicle-dev/radicle-upstream/commit/3f0c7d9997fa58293f5ca15553ac4709ad954c1b)), closes [#1433](https://github.com/radicle-dev/radicle-upstream/issues/1433)

### [0.1.5](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.4...v0.1.5) (2020-12-08)


### Features

* **ui:** extend effect of enter key in onboarding ([#1436](https://github.com/radicle-dev/radicle-upstream/issues/1436)) ([2e1964c](https://github.com/radicle-dev/radicle-upstream/commit/2e1964cffc3efb681f446e8d03988eff416d4061))


### Bug Fixes

* **ui:** avoid notify on internal request handling ([05bd5aa](https://github.com/radicle-dev/radicle-upstream/commit/05bd5aa950d5ee35903dcd6904f50d49724e20df)), closes [#1424](https://github.com/radicle-dev/radicle-upstream/issues/1424)
* **ui:** open only one modal at a time ([#1434](https://github.com/radicle-dev/radicle-upstream/issues/1434)) ([f996315](https://github.com/radicle-dev/radicle-upstream/commit/f996315868786ef57d2f9ad9b002d46778b4f8ac))
* **ui:** revert to default after untrack ([#1450](https://github.com/radicle-dev/radicle-upstream/issues/1450)) ([d1ce77e](https://github.com/radicle-dev/radicle-upstream/commit/d1ce77ee04a385f75edc594979da60e8d7aa8faf)), closes [#1210](https://github.com/radicle-dev/radicle-upstream/issues/1210)

### [0.1.4](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.3...v0.1.4) (2020-11-30)


### Features

* reactive remote management ([#1393](https://github.com/radicle-dev/radicle-upstream/issues/1393)) ([3f5c25b](https://github.com/radicle-dev/radicle-upstream/commit/3f5c25b127574705fd63bc2a24a52cc3cfe916f9)), closes [#1365](https://github.com/radicle-dev/radicle-upstream/issues/1365)

### [0.1.3](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.2...v0.1.3) (2020-11-27)


### Bug Fixes

* **ci:** timeout for proxy tests ([#1379](https://github.com/radicle-dev/radicle-upstream/issues/1379)) ([fcf45d1](https://github.com/radicle-dev/radicle-upstream/commit/fcf45d128d355c6c229e74fc30f7e930e7eaf3a3))
* **proxy:** avoid double seed emission ([#1400](https://github.com/radicle-dev/radicle-upstream/issues/1400)) ([12214b7](https://github.com/radicle-dev/radicle-upstream/commit/12214b7a567f3eac8ab53f1d59579325ca3bd107))
* **proxy:** only save most recent logs from proxy ([#1377](https://github.com/radicle-dev/radicle-upstream/issues/1377)) ([adb105e](https://github.com/radicle-dev/radicle-upstream/commit/adb105ee04be17a109d99fe8ea41968c5f64cb6b)), closes [#1359](https://github.com/radicle-dev/radicle-upstream/issues/1359)
* **proxy:** quit app when render process crashes ([#1398](https://github.com/radicle-dev/radicle-upstream/issues/1398)) ([2765554](https://github.com/radicle-dev/radicle-upstream/commit/27655544bee7db72a70dcd7d7e3c9cfe52c8ec65)), closes [#1352](https://github.com/radicle-dev/radicle-upstream/issues/1352)
* **proxy:** remote tracking semantics ([#1371](https://github.com/radicle-dev/radicle-upstream/issues/1371)) ([6c115a8](https://github.com/radicle-dev/radicle-upstream/commit/6c115a8557d568cfa816ecd954c0df6ce7e976ae)), closes [#1243](https://github.com/radicle-dev/radicle-upstream/issues/1243) [#1243](https://github.com/radicle-dev/radicle-upstream/issues/1243)
* **proxy:** remove seeds guard to enable reconnect ([#1392](https://github.com/radicle-dev/radicle-upstream/issues/1392)) ([ea1202c](https://github.com/radicle-dev/radicle-upstream/commit/ea1202c40cb2541da0334e6609b67d13dee59203)), closes [#1316](https://github.com/radicle-dev/radicle-upstream/issues/1316)
* **proxy:** validate default git config ([#1385](https://github.com/radicle-dev/radicle-upstream/issues/1385)) ([08f232a](https://github.com/radicle-dev/radicle-upstream/commit/08f232a086f0fd02b71b5fae422a79b5f671d5e7))
* **ui:** increase hit area for sidebar buttons ([#1387](https://github.com/radicle-dev/radicle-upstream/issues/1387)) ([2fd7326](https://github.com/radicle-dev/radicle-upstream/commit/2fd73261a07c2af73ea2c769ebd255b780562d91))
* **ui:** make stats component reactive ([#1382](https://github.com/radicle-dev/radicle-upstream/issues/1382)) ([29d87ec](https://github.com/radicle-dev/radicle-upstream/commit/29d87ecc2024f02ba1079aa4a0434d8f71e66dac))
* **ui:** trim whitespace from search input ([#1386](https://github.com/radicle-dev/radicle-upstream/issues/1386)) ([8a6f546](https://github.com/radicle-dev/radicle-upstream/commit/8a6f546729abf5cd93c42fcbca3dc770678c0e2d)), closes [#1367](https://github.com/radicle-dev/radicle-upstream/issues/1367)
* **ui:** update doc link ([#1381](https://github.com/radicle-dev/radicle-upstream/issues/1381)) ([9d97cc3](https://github.com/radicle-dev/radicle-upstream/commit/9d97cc3727468058fcaab58a4243bb510f0783d9))

### [0.1.2](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.1...v0.1.2) (2020-11-26)


### Features

* **proxy:** update to radicle-surf v0.5.3 ([#1374](https://github.com/radicle-dev/radicle-upstream/pull/1374)) ([20f2b9b](https://github.com/radicle-dev/radicle-upstream/commit/20f2b9bb93feb1f34ee843561344701ea5db9ebc))

### Bug Fixes

* **ui:** align checkout modal with button ([#1370](https://github.com/radicle-dev/radicle-upstream/issues/1370)) ([4598283](https://github.com/radicle-dev/radicle-upstream/commit/4598283a02bbcc3f2a0edc6c7b688d3136efa4bd))


### [0.1.1](https://github.com/radicle-dev/radicle-upstream/compare/v0.1.0...v0.1.1) (2020-11-25)


### Bug Fixes

* **docs:** document that AppImage needs to be made executable ([#1355](https://github.com/radicle-dev/radicle-upstream/issues/1355)) ([abbf5a2](https://github.com/radicle-dev/radicle-upstream/commit/abbf5a22251ab6fe11466b014fe8fb4c292d6eab)), closes [#1339](https://github.com/radicle-dev/radicle-upstream/issues/1339)
* **proxy:** filter log lines from `quinn` create ([#1354](https://github.com/radicle-dev/radicle-upstream/issues/1354)) ([c08317b](https://github.com/radicle-dev/radicle-upstream/commit/c08317b1aac14336041026aede86a23a23ec4fa0))
* **proxy:** query peer refs after tracking ([#1332](https://github.com/radicle-dev/radicle-upstream/issues/1332)) ([0a216f8](https://github.com/radicle-dev/radicle-upstream/commit/0a216f8c1591dab8c706640b1ff065a01556d58d)), closes [#1242](https://github.com/radicle-dev/radicle-upstream/issues/1242)
* **ui:** fix BSOD overflow scroll bug ([#1353](https://github.com/radicle-dev/radicle-upstream/issues/1353)) ([8250317](https://github.com/radicle-dev/radicle-upstream/commit/82503179515528a24e080cd740589e19443f71a8))
* **ui:** remove logging on opening external links ([#1357](https://github.com/radicle-dev/radicle-upstream/issues/1357)) ([3e3285b](https://github.com/radicle-dev/radicle-upstream/commit/3e3285bdbf6f3a94859e273df22570102fd31edf))

## [0.1.0](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.17...v0.1.0) (2020-11-24)


### Features

* **proxy:** persist WaitingRoom ([#1247](https://github.com/radicle-dev/radicle-upstream/issues/1247)) ([64d402e](https://github.com/radicle-dev/radicle-upstream/commit/64d402e836d61c60ea613aaeb3e08b7cf71d7f5f))
* **proxy:** push all branches when creating a project ([#1278](https://github.com/radicle-dev/radicle-upstream/issues/1278)) ([930d3d2](https://github.com/radicle-dev/radicle-upstream/commit/930d3d24b2b4ad9a04437f05101bcced152deca1))
* **ui:** use error logging for all `catch` statements ([#1251](https://github.com/radicle-dev/radicle-upstream/issues/1251)) ([015965d](https://github.com/radicle-dev/radicle-upstream/commit/015965da9f64b2050574633a4967da26e6bb6469)), closes [#1187](https://github.com/radicle-dev/radicle-upstream/issues/1187)


### Bug Fixes

* **proxy:** fix announcemnets ([#1305](https://github.com/radicle-dev/radicle-upstream/issues/1305)) ([d1bd4b7](https://github.com/radicle-dev/radicle-upstream/commit/d1bd4b7266489b7ebb68e0357022dbf428c8c8f7)), closes [#1299](https://github.com/radicle-dev/radicle-upstream/issues/1299)
* **proxy:** return tracking remotes as contributor ([#1304](https://github.com/radicle-dev/radicle-upstream/issues/1304)) ([3339678](https://github.com/radicle-dev/radicle-upstream/commit/3339678a0ef52dec740ed268daff97c5b677e61c)), closes [#1297](https://github.com/radicle-dev/radicle-upstream/issues/1297)
* **ui:** don't show stats that equal zero ([#1291](https://github.com/radicle-dev/radicle-upstream/issues/1291)) ([9f10048](https://github.com/radicle-dev/radicle-upstream/commit/9f100482f87a554ba4adaa9e5d4f7e25684fd651))
* **ui:** fetch list of requested projects after search ([#1281](https://github.com/radicle-dev/radicle-upstream/issues/1281)) ([170edd9](https://github.com/radicle-dev/radicle-upstream/commit/170edd9fa756b5c4ea9b4d1b58d335656771eacf)), closes [#1279](https://github.com/radicle-dev/radicle-upstream/issues/1279)
* **ui:** improve error code for backend termination ([#1265](https://github.com/radicle-dev/radicle-upstream/issues/1265)) ([a0477fa](https://github.com/radicle-dev/radicle-upstream/commit/a0477fa170653095f5c857629fdd849d42908cac)), closes [radicle-dev/radicle-docs/pull/21](https://github.com/radicle-dev/radicle-docs/pull/21#discussion_r522785330)
* **ui:** move `go to profile` to isExperimental ([#1261](https://github.com/radicle-dev/radicle-upstream/issues/1261)) ([37966ed](https://github.com/radicle-dev/radicle-upstream/commit/37966eda899c396e20344c2cdee7fd6e31292754))
* **ui:** only allow scripts from origin ([#1256](https://github.com/radicle-dev/radicle-upstream/issues/1256)) ([cb37499](https://github.com/radicle-dev/radicle-upstream/commit/cb374993fd6897c855e2574f81f8a36a83cdbaab))
* **ui:** remove unused props ([#1257](https://github.com/radicle-dev/radicle-upstream/issues/1257)) ([2442d16](https://github.com/radicle-dev/radicle-upstream/commit/2442d16c16babd6e2c4cf714b9c1b7c4c51b897b))
* **ui:** specify List keys ([#1280](https://github.com/radicle-dev/radicle-upstream/issues/1280)) ([b1a2a5f](https://github.com/radicle-dev/radicle-upstream/commit/b1a2a5f561d4efb4918809ad58375ea1150fc912))
* **ui:** truncate notification messages to fit into view ([#1267](https://github.com/radicle-dev/radicle-upstream/issues/1267)) ([c0f4086](https://github.com/radicle-dev/radicle-upstream/commit/c0f40867324b6671c396fb9348e8076b46c5abae))

### [0.0.17](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.16...v0.0.17) (2020-11-13)


### Features

* **proxy:** Add unsealed guard ([#1008](https://github.com/radicle-dev/radicle-upstream/issues/1008)) ([a7ea4a3](https://github.com/radicle-dev/radicle-upstream/commit/a7ea4a3b9cd4b6de5ba2aa97b859247039c4c489))
* **proxy:** Generate random cookie values ([#1149](https://github.com/radicle-dev/radicle-upstream/issues/1149)) ([f681a97](https://github.com/radicle-dev/radicle-upstream/commit/f681a978be063f2b7178d0974f6db0183ee5f803))
* **proxy:** list project requests ([#1103](https://github.com/radicle-dev/radicle-upstream/issues/1103)) ([7e1aaf9](https://github.com/radicle-dev/radicle-upstream/commit/7e1aaf9ad1101bc4abb8f1d96b3ef7433a8ab2d1)), closes [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* **proxy:** peer status update events ([#1035](https://github.com/radicle-dev/radicle-upstream/issues/1035)) ([ec582fe](https://github.com/radicle-dev/radicle-upstream/commit/ec582fef4739915639da1b1ce0a29b87dc98d643))
* **proxy:** streaming seed discovery ([#1089](https://github.com/radicle-dev/radicle-upstream/issues/1089)) ([a6e3b83](https://github.com/radicle-dev/radicle-upstream/commit/a6e3b83a4bf445683f737d82f290b56055e7f386))
* **proxy:** track endpoint ([#1005](https://github.com/radicle-dev/radicle-upstream/issues/1005)) ([f6d9dfd](https://github.com/radicle-dev/radicle-upstream/commit/f6d9dfd830021040f4740be6fa44502e982a823f))
* **proxy:** untrack peer ([#1101](https://github.com/radicle-dev/radicle-upstream/issues/1101)) ([2d299f4](https://github.com/radicle-dev/radicle-upstream/commit/2d299f473533a43b9ac52f76f42eee12b67896e0))
* **proxy:** Use password from user to unseal key store ([#1153](https://github.com/radicle-dev/radicle-upstream/issues/1153)) ([0f3290e](https://github.com/radicle-dev/radicle-upstream/commit/0f3290ead63c6e8643373760e2d161bcd0a85e05))
* **ui:** add blue screen of death ([#1092](https://github.com/radicle-dev/radicle-upstream/issues/1092)) ([958dee7](https://github.com/radicle-dev/radicle-upstream/commit/958dee7b1a865dd133c3c2797bae941be355c350))
* **ui:** add peerId to settings ([#1152](https://github.com/radicle-dev/radicle-upstream/issues/1152)) ([8b96f56](https://github.com/radicle-dev/radicle-upstream/commit/8b96f5693a1ebbbc522221a3062a3ebc6f9018b9))
* **ui:** add profile follow tab empty state ([#985](https://github.com/radicle-dev/radicle-upstream/issues/985)) ([3bcbdd0](https://github.com/radicle-dev/radicle-upstream/commit/3bcbdd0593b9ec71d0e5655ba1bf21c4efe72ba8))
* **ui:** check if project exists when searching ([#1030](https://github.com/radicle-dev/radicle-upstream/issues/1030)) ([646c50e](https://github.com/radicle-dev/radicle-upstream/commit/646c50e220ab717fef83b665b9e0bc8f6f548bd1)), closes [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* **ui:** implement new seed input form ([#1082](https://github.com/radicle-dev/radicle-upstream/issues/1082)) ([3d2b441](https://github.com/radicle-dev/radicle-upstream/commit/3d2b4411309f32c4cea8079befb6a70ae0b0d572))
* **ui:** introduce structured error logging ([#1221](https://github.com/radicle-dev/radicle-upstream/issues/1221)) ([9135743](https://github.com/radicle-dev/radicle-upstream/commit/9135743a1cfa68de3238eeaacb975398ba718a17))
* **ui:** list project requests ([#1104](https://github.com/radicle-dev/radicle-upstream/issues/1104)) ([f0122e1](https://github.com/radicle-dev/radicle-upstream/commit/f0122e17959857845fdf961d3ff764a0e6f797b1)), closes [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* **ui:** lock screen on key store unseal ([#1194](https://github.com/radicle-dev/radicle-upstream/issues/1194)) ([4a8213b](https://github.com/radicle-dev/radicle-upstream/commit/4a8213bc3934ba1ba661c8cc5e46411476adbe32))
* **ui:** log unhandled exceptions ([#1250](https://github.com/radicle-dev/radicle-upstream/issues/1250)) ([79d2372](https://github.com/radicle-dev/radicle-upstream/commit/79d23729dbcaacb7bd9488d076a0ed8235bacb79))
* **ui:** reduce number of retries after proxy restart ([#1228](https://github.com/radicle-dev/radicle-upstream/issues/1228)) ([0b646d3](https://github.com/radicle-dev/radicle-upstream/commit/0b646d3805ae9342740144c71d388305a5777e47)), closes [#1153](https://github.com/radicle-dev/radicle-upstream/issues/1153)
* **ui:** show proxy logs when proxy crashes ([#1216](https://github.com/radicle-dev/radicle-upstream/issues/1216)) ([9b328e8](https://github.com/radicle-dev/radicle-upstream/commit/9b328e82e0caba62672958caf0cac2d03a4af830)), closes [#1095](https://github.com/radicle-dev/radicle-upstream/issues/1095)
* add project search notifications ([#1117](https://github.com/radicle-dev/radicle-upstream/issues/1117)) ([6b0d4dd](https://github.com/radicle-dev/radicle-upstream/commit/6b0d4dda135791316f9cdf1ca06b58290b8cb42f)), closes [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* require keystore unsealing ([#1120](https://github.com/radicle-dev/radicle-upstream/issues/1120)) ([19f5c51](https://github.com/radicle-dev/radicle-upstream/commit/19f5c513beaf7fae2cb8099cd888288b548c4224))
* **ui:** project remote management ([#1014](https://github.com/radicle-dev/radicle-upstream/issues/1014)) ([4e78cad](https://github.com/radicle-dev/radicle-upstream/commit/4e78cad0fbdd0e79dcf84339489d31f210baedfc))
* cancel project search ([#1105](https://github.com/radicle-dev/radicle-upstream/issues/1105)) ([7377dbf](https://github.com/radicle-dev/radicle-upstream/commit/7377dbfd889b118f5a80feb4591c434637c10fe8)), closes [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* **ui:** show network connectivity indicator ([#1027](https://github.com/radicle-dev/radicle-upstream/issues/1027)) ([d84a88c](https://github.com/radicle-dev/radicle-upstream/commit/d84a88c50cd0cfe0a8cfa28c895d1178410b0ec4))
* **ui:** use consistent shareable identifiers across UI ([#1019](https://github.com/radicle-dev/radicle-upstream/issues/1019)) ([69c78cb](https://github.com/radicle-dev/radicle-upstream/commit/69c78cbeb4749b463ab6bdad65ba7e53225d2ed2))


### Bug Fixes

* **ci:** don’t stall ci builds when tests fail ([#1185](https://github.com/radicle-dev/radicle-upstream/issues/1185)) ([bbdc951](https://github.com/radicle-dev/radicle-upstream/commit/bbdc9518fafca98d24fff012e2dce4275e476e9e))
* **ci:** prevent concurrent builds on master branch ([#1162](https://github.com/radicle-dev/radicle-upstream/issues/1162)) ([76b6bbc](https://github.com/radicle-dev/radicle-upstream/commit/76b6bbcf6c55b4f5452e6309e2a13fe708cbc7b4))
* **proxy:** add include to project creation ([#1205](https://github.com/radicle-dev/radicle-upstream/issues/1205)) ([06cc610](https://github.com/radicle-dev/radicle-upstream/commit/06cc6102ce592419965ae0173887b8d4cf2da544))
* **proxy:** always overwrite git helper with latest one ([#1200](https://github.com/radicle-dev/radicle-upstream/issues/1200)) ([62fb02f](https://github.com/radicle-dev/radicle-upstream/commit/62fb02f29c7af4bb7b9980176d42e4d7358487c9))
* **proxy:** avoid double discovery on startup ([#1093](https://github.com/radicle-dev/radicle-upstream/issues/1093)) ([46aae51](https://github.com/radicle-dev/radicle-upstream/commit/46aae51121b40afdeca8d425ffbde1bcd8f7081d))
* **proxy:** await request urn ([#1025](https://github.com/radicle-dev/radicle-upstream/issues/1025)) ([2cdad05](https://github.com/radicle-dev/radicle-upstream/commit/2cdad0563849fac1207ba864c7abbb7a8148f970)), closes [#992](https://github.com/radicle-dev/radicle-upstream/issues/992) [#984](https://github.com/radicle-dev/radicle-upstream/issues/984)
* **proxy:** change announcement strategy ([#1177](https://github.com/radicle-dev/radicle-upstream/issues/1177)) ([e7fb302](https://github.com/radicle-dev/radicle-upstream/commit/e7fb3020cfef5dcb70083185325e2d425c9eb352)), closes [#1143](https://github.com/radicle-dev/radicle-upstream/issues/1143)
* **proxy:** check waiting room has urn ([#1156](https://github.com/radicle-dev/radicle-upstream/issues/1156)) ([1dfd500](https://github.com/radicle-dev/radicle-upstream/commit/1dfd5006b63017591fd2d366b74fb1096a46acd4))
* **proxy:** checkout peer ([#1010](https://github.com/radicle-dev/radicle-upstream/issues/1010)) ([bdcec04](https://github.com/radicle-dev/radicle-upstream/commit/bdcec04a96ec9b40dbf28a9381d39ebb42fc4800))
* **proxy:** clean up long-running notifications ([#1053](https://github.com/radicle-dev/radicle-upstream/issues/1053)) ([030714b](https://github.com/radicle-dev/radicle-upstream/commit/030714b134adc6a3bb0d205572855f886fc1ffb8))
* **proxy:** correct handling of connections ([#1094](https://github.com/radicle-dev/radicle-upstream/issues/1094)) ([5974b87](https://github.com/radicle-dev/radicle-upstream/commit/5974b870d3509ac701280c2450ccdbefa196c5df))
* **proxy:** do not drop event tasks ([#1217](https://github.com/radicle-dev/radicle-upstream/issues/1217)) ([0dfc333](https://github.com/radicle-dev/radicle-upstream/commit/0dfc333345740028f9c3ca4e7fd9d6537e28220f))
* **proxy:** extensive validation ([#1190](https://github.com/radicle-dev/radicle-upstream/issues/1190)) ([cfa046d](https://github.com/radicle-dev/radicle-upstream/commit/cfa046d2183fb90165aa5a854e4a6ca013497057))
* **proxy:** fix waiting room logic for next query ([#1071](https://github.com/radicle-dev/radicle-upstream/issues/1071)) ([761864e](https://github.com/radicle-dev/radicle-upstream/commit/761864e86ffb912228f5de5a218f53f911db9d05)), closes [#1040](https://github.com/radicle-dev/radicle-upstream/issues/1040)
* **proxy:** ignore fetch errors in when syncing ([#989](https://github.com/radicle-dev/radicle-upstream/issues/989)) ([2ca0503](https://github.com/radicle-dev/radicle-upstream/commit/2ca050344582afe4ca4cfc601ed6e6de20b3401f)), closes [#944](https://github.com/radicle-dev/radicle-upstream/issues/944)
* **proxy:** partial project lists ([#1129](https://github.com/radicle-dev/radicle-upstream/issues/1129)) ([5af0c61](https://github.com/radicle-dev/radicle-upstream/commit/5af0c618438bc948ce7c02eb5e8d064fd306c5b6))
* **proxy:** peer browsing ([#1015](https://github.com/radicle-dev/radicle-upstream/issues/1015)) ([5c7ba2a](https://github.com/radicle-dev/radicle-upstream/commit/5c7ba2ad11f64e816270a014126405a8740f1377))
* **proxy:** persist temp dir on unseal in test mode ([#1170](https://github.com/radicle-dev/radicle-upstream/issues/1170)) ([8207a75](https://github.com/radicle-dev/radicle-upstream/commit/8207a7589874518920f02ffccabb80c1cff62b10)), closes [#1124](https://github.com/radicle-dev/radicle-upstream/issues/1124)
* **proxy:** prevent halting peer subroutines ([#992](https://github.com/radicle-dev/radicle-upstream/issues/992)) ([35bc0b4](https://github.com/radicle-dev/radicle-upstream/commit/35bc0b41f1e9d7d0e102b185879230918f55f0dc)), closes [#981](https://github.com/radicle-dev/radicle-upstream/issues/981) [#1009](https://github.com/radicle-dev/radicle-upstream/issues/1009)
* **proxy:** validate creation paths ([#1134](https://github.com/radicle-dev/radicle-upstream/issues/1134)) ([a73f0cb](https://github.com/radicle-dev/radicle-upstream/commit/a73f0cb4132b05e5d38acc4670a736497b148e05))
* **proxy**: terminate proxy on app shutdown ([#1087](https://github.com/radicle-dev/radicle-upstream/issues/1087)) ([c2bc98c](https://github.com/radicle-dev/radicle-upstream/commit/c2bc98ccc31a876e0d220d939d1e512542695506)), closes [#1085](https://github.com/radicle-dev/radicle-upstream/issues/1085)
* **proxy:** handle missing source object gracefully ([#937](https://github.com/radicle-dev/radicle-upstream/issues/937)) ([d16dbc8](https://github.com/radicle-dev/radicle-upstream/commit/d16dbc8164c93c8df99f6224be3a74685bed2a6e)), closes [#934](https://github.com/radicle-dev/radicle-upstream/issues/934)
* **ui:** always tracked toggle for projects ([#1047](https://github.com/radicle-dev/radicle-upstream/issues/1047)) ([1217583](https://github.com/radicle-dev/radicle-upstream/commit/121758327f67ea42123cd93f8958e21d1b4ebe13))
* **ui:** correct check for listinng ([#1114](https://github.com/radicle-dev/radicle-upstream/issues/1114)) ([974d810](https://github.com/radicle-dev/radicle-upstream/commit/974d8101d967248f9cc2ddd1d9227e7063c9921d))
* **ui:** correct left positioned Tooltip ([#1070](https://github.com/radicle-dev/radicle-upstream/issues/1070)) ([a3aa1e2](https://github.com/radicle-dev/radicle-upstream/commit/a3aa1e28b99849232c0314136a795cf885027f0e)), closes [#1068](https://github.com/radicle-dev/radicle-upstream/issues/1068)
* **ui:** extend password text ([#990](https://github.com/radicle-dev/radicle-upstream/issues/990)) ([3ac0e4b](https://github.com/radicle-dev/radicle-upstream/commit/3ac0e4beff1bd9970d35900ea37b9fed9151412d))
* **ui:** fix peer selection on repo change ([#1074](https://github.com/radicle-dev/radicle-upstream/issues/1074)) ([844b527](https://github.com/radicle-dev/radicle-upstream/commit/844b5271513897615dd5acb9f7ae8b29e09cf035)), closes [#1038](https://github.com/radicle-dev/radicle-upstream/issues/1038)
* **ui:** handle special symbols in filenames ([#1028](https://github.com/radicle-dev/radicle-upstream/issues/1028)) ([21c5f62](https://github.com/radicle-dev/radicle-upstream/commit/21c5f624afcbc35d3e33fbb5b1dd65951dde1d8d))
* **ui:** limit display name and project name max length ([#1055](https://github.com/radicle-dev/radicle-upstream/issues/1055)) ([6680689](https://github.com/radicle-dev/radicle-upstream/commit/6680689fa54e5576597905ec6b23524d88348bc2))
* **ui:** make manage remotes modal consistent with peer selector ([#1155](https://github.com/radicle-dev/radicle-upstream/issues/1155)) ([2db9771](https://github.com/radicle-dev/radicle-upstream/commit/2db9771fbc1c2dd29fb859b65f2ea3201c0da1dd))
* **ui:** only one overlay open at a time ([#963](https://github.com/radicle-dev/radicle-upstream/issues/963)) ([b7eaeb9](https://github.com/radicle-dev/radicle-upstream/commit/b7eaeb9c332860c82f70855c0fe908d5718d83ee))
* **ui:** pluralize peer count correctly ([#1150](https://github.com/radicle-dev/radicle-upstream/issues/1150)) ([e78c679](https://github.com/radicle-dev/radicle-upstream/commit/e78c679b504fcbb5435173b1f18a19cc40a9cb7d))
* **ui:** remove project name from manage remote modal ([#1237](https://github.com/radicle-dev/radicle-upstream/issues/1237)) ([84eca18](https://github.com/radicle-dev/radicle-upstream/commit/84eca18e0a41fc13b93dbb3cfe6210fe1611bfe4))
* **ui:** revive avatar in profile ([#1110](https://github.com/radicle-dev/radicle-upstream/issues/1110)) ([37cf591](https://github.com/radicle-dev/radicle-upstream/commit/37cf59146de1d12f6eb9c43796cc18c00a6bd7fb)), closes [#1104](https://github.com/radicle-dev/radicle-upstream/issues/1104)
* **ui:** show correct projects for users ([#1102](https://github.com/radicle-dev/radicle-upstream/issues/1102)) ([1d848f1](https://github.com/radicle-dev/radicle-upstream/commit/1d848f1adeb4cb5efce6d333b6d3324241016240)), closes [#1100](https://github.com/radicle-dev/radicle-upstream/issues/1100)
* **ui:** show search input hint on empty input ([#980](https://github.com/radicle-dev/radicle-upstream/issues/980)) ([5afd714](https://github.com/radicle-dev/radicle-upstream/commit/5afd7141b17f3e206c811a28b7c8dc6111ee4168))
* fix path in reset script ([#1067](https://github.com/radicle-dev/radicle-upstream/issues/1067)) ([85dd2ef](https://github.com/radicle-dev/radicle-upstream/commit/85dd2ef44b0e69960e0575027b37bb44c7b23202))
* fix "run all specs" from cypress UI ([#1051](https://github.com/radicle-dev/radicle-upstream/issues/1051)) ([77326ba](https://github.com/radicle-dev/radicle-upstream/commit/77326ba95b087305a662e6323bf4187b86f6236d))
* fix reset:state yarn command ([#996](https://github.com/radicle-dev/radicle-upstream/issues/996)) ([0008a7f](https://github.com/radicle-dev/radicle-upstream/commit/0008a7f5d5ea39cb1365861be32f26976a62a159))
* party prep ([#1034](https://github.com/radicle-dev/radicle-upstream/issues/1034)) ([76baf80](https://github.com/radicle-dev/radicle-upstream/commit/76baf80a0bc81b397b0b703cc68a0520f0619bfc))

### [0.0.16](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.15...v0.0.16) (2020-09-30)


### Features

* **proxy:** configure include file during checkout ([#946](https://github.com/radicle-dev/radicle-upstream/issues/946)) ([cca0273](https://github.com/radicle-dev/radicle-upstream/commit/cca0273066165c79dfae9e86cc4688389d2cea0d)), closes [#894](https://github.com/radicle-dev/radicle-upstream/issues/894) [#930](https://github.com/radicle-dev/radicle-upstream/issues/930)
* **proxy:** keep CoCo alive ([#977](https://github.com/radicle-dev/radicle-upstream/issues/977)) ([91c1f19](https://github.com/radicle-dev/radicle-upstream/commit/91c1f190beeadd511153c1b1c1ef4bd2adbfc7cb))
* **proxy:** waiting room requests ([#903](https://github.com/radicle-dev/radicle-upstream/issues/903)) ([cc1834b](https://github.com/radicle-dev/radicle-upstream/commit/cc1834b10d18fd388157a4bb7775916db24a7c90))
* **proxy:** waiting room subroutine ([#967](https://github.com/radicle-dev/radicle-upstream/issues/967)) ([020bf33](https://github.com/radicle-dev/radicle-upstream/commit/020bf33e8a8406020f6eac9ed1141dbfd0b6258c)), closes [#955](https://github.com/radicle-dev/radicle-upstream/issues/955)
* **ui:** use search bar to kick off project search ([#969](https://github.com/radicle-dev/radicle-upstream/issues/969)) ([e4d1996](https://github.com/radicle-dev/radicle-upstream/commit/e4d19966ede8ec83a225d955be7b46fee5c8be8c))


### Bug Fixes

* **proxy:** reintroduce sync on startup ([#979](https://github.com/radicle-dev/radicle-upstream/issues/979)) ([60926d1](https://github.com/radicle-dev/radicle-upstream/commit/60926d17b629187a7b3a667e420703bf76e99864))
* **ui:** wrap revision selector content ([#972](https://github.com/radicle-dev/radicle-upstream/issues/972)) ([26ecccd](https://github.com/radicle-dev/radicle-upstream/commit/26ecccdffd6038e1f0c26028a10b8620314b516a))
* show commit count on project page ([#962](https://github.com/radicle-dev/radicle-upstream/issues/962)) ([11f0e47](https://github.com/radicle-dev/radicle-upstream/commit/11f0e47c7a00eb04a9424f2208322365d4a4f788))
* **proxy:** move delta to config ([#968](https://github.com/radicle-dev/radicle-upstream/issues/968)) ([53a0971](https://github.com/radicle-dev/radicle-upstream/commit/53a0971fd44ccead44682d3e91953b52207c185b))
* **ui:** improve identity and project name validation ([#953](https://github.com/radicle-dev/radicle-upstream/issues/953)) ([e57863c](https://github.com/radicle-dev/radicle-upstream/commit/e57863c5d37a9a8b611242196c1f0bde2bfcb592))

### [0.0.15](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.14...v0.0.15) (2020-09-23)


### Features

* **proxy:** differentiate tracked projects from my projects ([#866](https://github.com/radicle-dev/radicle-upstream/issues/866)) ([b793264](https://github.com/radicle-dev/radicle-upstream/commit/b7932640adb9badf4a065ae25323db3285b63d5e))
* **proxy:** sync with peers when coming online ([#896](https://github.com/radicle-dev/radicle-upstream/issues/896)) ([4ea6860](https://github.com/radicle-dev/radicle-upstream/commit/4ea68609572b43afa994d656393df98adac32313)), closes [#852](https://github.com/radicle-dev/radicle-upstream/issues/852)
* **ui:** add input field hint ([#914](https://github.com/radicle-dev/radicle-upstream/issues/914)) ([4e47aff](https://github.com/radicle-dev/radicle-upstream/commit/4e47aff6a8f30536a05f88296ca0c679908204bb))
* **ui:** introduce experimental flag ([#913](https://github.com/radicle-dev/radicle-upstream/issues/913)) ([6730388](https://github.com/radicle-dev/radicle-upstream/commit/67303881a8b87152464008c66e7a6d2e525894bf))
* **ui:** project name formatting ([#938](https://github.com/radicle-dev/radicle-upstream/issues/938)) ([cefa07f](https://github.com/radicle-dev/radicle-upstream/commit/cefa07f9751f363c8aee3dba3ad22e7a9c6d785f))


### Bug Fixes

* **ui:** allow irc:// as external protocol ([#921](https://github.com/radicle-dev/radicle-upstream/issues/921)) ([0e33c06](https://github.com/radicle-dev/radicle-upstream/commit/0e33c0666d84eab88bebf57f0cfd662c6bf595ca))
* **ui:** handle repositories with no branches gracefully ([#945](https://github.com/radicle-dev/radicle-upstream/issues/945)) ([f3c5ff1](https://github.com/radicle-dev/radicle-upstream/commit/f3c5ff19c9e8221abb45f7ee3726c3d2af5ee3ea))
* **ui:** show deleted files in commit view ([#925](https://github.com/radicle-dev/radicle-upstream/issues/925)) ([410d657](https://github.com/radicle-dev/radicle-upstream/commit/410d6577fbc42d06f1f2d2b4b48246a15c0a6633))

### [0.0.14](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.13...v0.0.14) (2020-09-16)


### Features

* **proxy:** coco announce ([#838](https://github.com/radicle-dev/radicle-upstream/issues/838)) ([761f11f](https://github.com/radicle-dev/radicle-upstream/commit/761f11f5ea66aad97fda252e6eb08e683285ac0b)), closes [#602](https://github.com/radicle-dev/radicle-upstream/issues/602) [#848](https://github.com/radicle-dev/radicle-upstream/issues/848)
* **proxy:** fetch project updates ([#856](https://github.com/radicle-dev/radicle-upstream/issues/856)) ([70991c1](https://github.com/radicle-dev/radicle-upstream/commit/70991c1ee6772bbfe57a23a37b13d1b0d9e753c0))
* **proxy:** reintroduce notifications ([#865](https://github.com/radicle-dev/radicle-upstream/issues/865)) ([81c4d0b](https://github.com/radicle-dev/radicle-upstream/commit/81c4d0bb13c21ec5ec6167b0405a360cc43371f8)), closes [#864](https://github.com/radicle-dev/radicle-upstream/issues/864)
* **ui:** add feedback section in settings ([#895](https://github.com/radicle-dev/radicle-upstream/issues/895)) ([d0a13b0](https://github.com/radicle-dev/radicle-upstream/commit/d0a13b085200b3783e16a8cc46409b3831ff2362))
* **ui:** navigate-to-project modal ([#738](https://github.com/radicle-dev/radicle-upstream/issues/738)) ([a3403e2](https://github.com/radicle-dev/radicle-upstream/commit/a3403e2d0f0cd0e3166c306f3089c3de473fcfc7))
* **ui:** refresh onboarding ([#837](https://github.com/radicle-dev/radicle-upstream/issues/837)) ([bd9aed4](https://github.com/radicle-dev/radicle-upstream/commit/bd9aed435db2efa9d5680501c3c4dbd496266250))
* **ui:** show app version ([#901](https://github.com/radicle-dev/radicle-upstream/issues/901)) ([c7e1612](https://github.com/radicle-dev/radicle-upstream/commit/c7e1612440efe4a8a703c2feb133b0367c88e749))
* **ui:** tracking tab ([#776](https://github.com/radicle-dev/radicle-upstream/issues/776)) ([72a7822](https://github.com/radicle-dev/radicle-upstream/commit/72a78226d8211ad2bf3cf6228d7d54006ff07fa4))
* **ui:** visitor profile view ([#816](https://github.com/radicle-dev/radicle-upstream/issues/816)) ([2206e5d](https://github.com/radicle-dev/radicle-upstream/commit/2206e5dc2bae9140be6f24d5147aeaf5ce0819e8))


### Bug Fixes

* **ci:** proxy binary naming ([#829](https://github.com/radicle-dev/radicle-upstream/issues/829)) ([b4d3276](https://github.com/radicle-dev/radicle-upstream/commit/b4d3276dec655ced8de61dbae067184e67c24349))
* **proxy:** session not loaded on restart ([#907](https://github.com/radicle-dev/radicle-upstream/issues/907)) ([6c3832d](https://github.com/radicle-dev/radicle-upstream/commit/6c3832dd520cbcb14f4e9b1db8a9574517249446)), closes [#900](https://github.com/radicle-dev/radicle-upstream/issues/900)
* **proxy:** signed refs retrieval for projects ([#891](https://github.com/radicle-dev/radicle-upstream/issues/891)) ([08444fb](https://github.com/radicle-dev/radicle-upstream/commit/08444fb0b6cbd56d20c3db6522c1f5687796ee9f))
* **ui:** fix crash in rollup watcher on typescript error ([#872](https://github.com/radicle-dev/radicle-upstream/issues/872)) ([66fc54f](https://github.com/radicle-dev/radicle-upstream/commit/66fc54f23383caa7280ddb023e133e8d89624eaa)), closes [#871](https://github.com/radicle-dev/radicle-upstream/issues/871)
* **ui:** make rollup watch mode work again ([#870](https://github.com/radicle-dev/radicle-upstream/issues/870)) ([682f609](https://github.com/radicle-dev/radicle-upstream/commit/682f60908a1c51c284aaa7f6e827c0faa20af498))
* **ui:** update and fix rollup ([#863](https://github.com/radicle-dev/radicle-upstream/issues/863)) ([9f8981d](https://github.com/radicle-dev/radicle-upstream/commit/9f8981de6fc9b00f2d564b322a48f0050ab5543b))
* **ui:** update copy ([#892](https://github.com/radicle-dev/radicle-upstream/issues/892)) ([6e43604](https://github.com/radicle-dev/radicle-upstream/commit/6e43604f0b43a20986bae114f6dee0a80e684877))

### [0.0.13](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.12...v0.0.13) (2020-08-27)


### Features

* **ui:** maintainer badge ([#818](https://github.com/radicle-dev/radicle-upstream/issues/818)) ([a431f5b](https://github.com/radicle-dev/radicle-upstream/commit/a431f5b3b02b542d15788cf839e5602231d676bf))
* **ui:** lock screen during long backend activity ([#815](https://github.com/radicle-dev/radicle-upstream/issues/815)) ([22d52c1](https://github.com/radicle-dev/radicle-upstream/commit/22d52c17d74e3f73b8d679236b1825d3eb39bf13))


### Bug Fixes

* **proxy:** project creation from repo without master ([#825](https://github.com/radicle-dev/radicle-upstream/issues/825)) ([fa2e072](https://github.com/radicle-dev/radicle-upstream/commit/fa2e072bfcafad16d61009f601b100d68f2df01c))
* **ui:** allow "." in project names ([#814](https://github.com/radicle-dev/radicle-upstream/issues/814)) ([d4b3c55](https://github.com/radicle-dev/radicle-upstream/commit/d4b3c558e5a6a9d97d162bc286afbb216cd15d48))
* **ui:** improve error messaging in project creation ([#813](https://github.com/radicle-dev/radicle-upstream/issues/813)) ([ebbe2a8](https://github.com/radicle-dev/radicle-upstream/commit/ebbe2a898beb5a6f9c21f4191618fd6c9ef1bf0d))
* **ui:** show real reason why checkout failed ([#823](https://github.com/radicle-dev/radicle-upstream/issues/823)) ([f3cfa0f](https://github.com/radicle-dev/radicle-upstream/commit/f3cfa0f0fa06a9b7cbd0f51fc034f89fe41a1f9b))

### [0.0.12](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.11...v0.0.12) (2020-08-24)


### Features

* **proxy:** accounts endpoint group ([#681](https://github.com/radicle-dev/radicle-upstream/issues/681)) ([df82a70](https://github.com/radicle-dev/radicle-upstream/commit/df82a704296c9c57f84b59de8547084a68065de2))
* **proxy:** add option to connect to registry nodes ([#472](https://github.com/radicle-dev/radicle-upstream/issues/472)) ([17880bf](https://github.com/radicle-dev/radicle-upstream/commit/17880bfd40bdb87d9ef80f6579c14ea960aab370)), closes [#440](https://github.com/radicle-dev/radicle-upstream/issues/440)
* **proxy:** authorized endpoints ([#596](https://github.com/radicle-dev/radicle-upstream/issues/596)) ([c3d4938](https://github.com/radicle-dev/radicle-upstream/commit/c3d49382bfdc95aa77b051655902c36de62a25b3)), closes [#548](https://github.com/radicle-dev/radicle-upstream/issues/548)
* **proxy:** build and package rad-remote-helper ([#718](https://github.com/radicle-dev/radicle-upstream/issues/718)) ([657e3f5](https://github.com/radicle-dev/radicle-upstream/commit/657e3f55ae778ddd936fd1a72f23c050ecea7358))
* **proxy:** clone repos ([#796](https://github.com/radicle-dev/radicle-upstream/issues/796)) ([14362bd](https://github.com/radicle-dev/radicle-upstream/commit/14362bd4b541351e25f9b6965a980910023c7b56))
* **proxy:** coco ascension ([#414](https://github.com/radicle-dev/radicle-upstream/issues/414)) ([18acd59](https://github.com/radicle-dev/radicle-upstream/commit/18acd597e10e395bb14318299aac9c4160007f0e)), closes [#434](https://github.com/radicle-dev/radicle-upstream/issues/434)
* **proxy:** expose the account id for user and org ([#669](https://github.com/radicle-dev/radicle-upstream/issues/669)) ([84af56e](https://github.com/radicle-dev/radicle-upstream/commit/84af56e0fd7b56884573f515577ae2a748848b82))
* **proxy:** expose the account id in session ([#705](https://github.com/radicle-dev/radicle-upstream/issues/705)) ([b0cac8c](https://github.com/radicle-dev/radicle-upstream/commit/b0cac8c3310a2066704b06a56f6e061bbf486460)), closes [#690](https://github.com/radicle-dev/radicle-upstream/issues/690)
* **proxy:** feed API ([#666](https://github.com/radicle-dev/radicle-upstream/issues/666)) ([6d85990](https://github.com/radicle-dev/radicle-upstream/commit/6d85990b297f2c20ab02841f220f479ebbd97000))
* **proxy:** filter projects by user URN ([#789](https://github.com/radicle-dev/radicle-upstream/issues/789)) ([4fe22e6](https://github.com/radicle-dev/radicle-upstream/commit/4fe22e61e339a7b8a40851afe8ecf4d9794ce53e)), closes [#741](https://github.com/radicle-dev/radicle-upstream/issues/741)
* **proxy:** integrate rad/self ([#628](https://github.com/radicle-dev/radicle-upstream/issues/628)) ([69c0adf](https://github.com/radicle-dev/radicle-upstream/commit/69c0adf89cdde994224b7527bbe54085fdcebd15)), closes [#576](https://github.com/radicle-dev/radicle-upstream/issues/576) [#548](https://github.com/radicle-dev/radicle-upstream/issues/548)
* **proxy:** integrate real repo stats ([#612](https://github.com/radicle-dev/radicle-upstream/issues/612)) ([6b5b603](https://github.com/radicle-dev/radicle-upstream/commit/6b5b603e5cf208e3f544216c2334316dc18a2f92))
* **proxy:** integrate registry account_exists check ([#675](https://github.com/radicle-dev/radicle-upstream/issues/675)) ([df05654](https://github.com/radicle-dev/radicle-upstream/commit/df05654119df84a060ba6ec3493111783af07e7b))
* **proxy:** integrate repository selector ([#636](https://github.com/radicle-dev/radicle-upstream/issues/636)) ([ea28bb0](https://github.com/radicle-dev/radicle-upstream/commit/ea28bb005d5a05151d7746e025c373a848b8d2b4))
* **proxy:** list entities ([#528](https://github.com/radicle-dev/radicle-upstream/issues/528)) ([8bf7ea3](https://github.com/radicle-dev/radicle-upstream/commit/8bf7ea37d10b9e9c8352abc42e3e3d4d9398dd19))
* **proxy:** peer branches ([#678](https://github.com/radicle-dev/radicle-upstream/issues/678)) ([4aa5a05](https://github.com/radicle-dev/radicle-upstream/commit/4aa5a05d266082b998bfb1f799dbc89072a66d75))
* **proxy:** seeds api ([#638](https://github.com/radicle-dev/radicle-upstream/issues/638)) ([64371b5](https://github.com/radicle-dev/radicle-upstream/commit/64371b53e20caa9f67803842f2e0136e13fff40e))
* **proxy:** tracked identities endpoint ([#757](https://github.com/radicle-dev/radicle-upstream/issues/757)) ([30d80e0](https://github.com/radicle-dev/radicle-upstream/commit/30d80e022fe6c1567291e8cbb9cd16e315b21abf)), closes [#741](https://github.com/radicle-dev/radicle-upstream/issues/741)
* **proxy:** transfer endpoint ([#653](https://github.com/radicle-dev/radicle-upstream/issues/653)) ([1e8ccc5](https://github.com/radicle-dev/radicle-upstream/commit/1e8ccc5d09b6575381804682e9b267344e15d378))
* **proxy:** user creation ([#511](https://github.com/radicle-dev/radicle-upstream/issues/511)) ([d303923](https://github.com/radicle-dev/radicle-upstream/commit/d3039237fd0b534337d5d60c308b4d6af4f703ad))
* **proxy:** implement nuke command ([#626](https://github.com/radicle-dev/radicle-upstream/issues/626)) ([6ce2bf7](https://github.com/radicle-dev/radicle-upstream/commit/6ce2bf744eabcbf64681438288a572de67587a55)), closes [#394](https://github.com/radicle-dev/radicle-upstream/issues/394)
* **proxy:** coco seeds on startup ([#792](https://github.com/radicle-dev/radicle-upstream/issues/792)) ([1752172](https://github.com/radicle-dev/radicle-upstream/commit/17521720bcf4a23ee0898d42783984d892b7d2e3))
* **proxy:** integrate registration fee and drop deposits ([#700](https://github.com/radicle-dev/radicle-upstream/issues/700)) ([0d98130](https://github.com/radicle-dev/radicle-upstream/commit/0d98130ef7f4e2374eaa6f5cf98d1133577d05c4))
* **proxy:** project checkout ([#722](https://github.com/radicle-dev/radicle-upstream/issues/722)) ([9609e21](https://github.com/radicle-dev/radicle-upstream/commit/9609e21e57699b6cd4bddf51b4acdf5621e2a84c))
* **proxy:** set up remote helper ([#769](https://github.com/radicle-dev/radicle-upstream/issues/769)) ([a308cb0](https://github.com/radicle-dev/radicle-upstream/commit/a308cb0f0cb3a92019cb521ba6dfb7d61a381229))
* **proxy:** diff integration ([#526](https://github.com/radicle-dev/radicle-upstream/issues/526)) ([7c8fd2a](https://github.com/radicle-dev/radicle-upstream/commit/7c8fd2a07fe4b5bcd830d146270965008e184ba5))
* **proxy:** apply project registration permission ([#504](https://github.com/radicle-dev/radicle-upstream/issues/504)) ([9469148](https://github.com/radicle-dev/radicle-upstream/commit/9469148a2615c47ec4be5f4cffac4ab8f88bb479))
* **proxy:** integrate transaction costs ([#457](https://github.com/radicle-dev/radicle-upstream/issues/457)) ([408d355](https://github.com/radicle-dev/radicle-upstream/commit/408d35537e0485d504fef6f12435d27991554fba))
* **proxy:** introduce session permissions ([#471](https://github.com/radicle-dev/radicle-upstream/issues/471)) ([c0fb83d](https://github.com/radicle-dev/radicle-upstream/commit/c0fb83d6728220e8ce88e2394bf5014e2739b7bb))
* **proxy:** register member endpoint integration ([#446](https://github.com/radicle-dev/radicle-upstream/issues/446)) ([80b4a6e](https://github.com/radicle-dev/radicle-upstream/commit/80b4a6e844fbde40e1f991db92ddca48176fbe41))
* **proxy:** syntax highlighting ([#618](https://github.com/radicle-dev/radicle-upstream/issues/618)) ([a0fc530](https://github.com/radicle-dev/radicle-upstream/commit/a0fc5301c8ebea53bb106e8a87b7d08ce31280c0))
* **proxy:** user project registration ([#453](https://github.com/radicle-dev/radicle-upstream/issues/453)) ([2bbdeae](https://github.com/radicle-dev/radicle-upstream/commit/2bbdeae9b767b498cc51b74436653e567f34b124))
* **ui:** add back button on commit page ([#580](https://github.com/radicle-dev/radicle-upstream/issues/580)) ([9b56ab3](https://github.com/radicle-dev/radicle-upstream/commit/9b56ab366e8925915f5b5da60c7467a4ff344e14))
* **ui:** add keyboard shortcuts ([#759](https://github.com/radicle-dev/radicle-upstream/issues/759)) ([1485bba](https://github.com/radicle-dev/radicle-upstream/commit/1485bba8d343f00e4cb87e3a644db22948ce72d2))
* **ui:** add notification when copying urn ([#758](https://github.com/radicle-dev/radicle-upstream/issues/758)) ([c461069](https://github.com/radicle-dev/radicle-upstream/commit/c4610697f849e76aa821b99b2d42682a070b7cfd))
* **ui:** do not allow registering a member twice ([#468](https://github.com/radicle-dev/radicle-upstream/issues/468)) ([32c8c25](https://github.com/radicle-dev/radicle-upstream/commit/32c8c25c0f1908c1f3e74d2226cd931ca2f809a7))
* **ui:** feature flag behind isDev() ([#761](https://github.com/radicle-dev/radicle-upstream/issues/761)) ([48b1aa0](https://github.com/radicle-dev/radicle-upstream/commit/48b1aa0fea747fbcc4e2e5feb3fc1ddba9783d05))
* **ui:** go to user profile aka visitor view ([#795](https://github.com/radicle-dev/radicle-upstream/issues/795)) ([a04f4ec](https://github.com/radicle-dev/radicle-upstream/commit/a04f4ecd481deeeba777fd0d1ebff6457fd207f4))
* **ui:** discovery ui ([#629](https://github.com/radicle-dev/radicle-upstream/issues/629)) ([1f38dc4](https://github.com/radicle-dev/radicle-upstream/commit/1f38dc49e8e323ecc7a3f6f5b52e0835ade4bd26))
* **ui:** include orgId in member registration tx ([#431](https://github.com/radicle-dev/radicle-upstream/issues/431)) ([0669ea8](https://github.com/radicle-dev/radicle-upstream/commit/0669ea846c8aa3d101e184914e7901db62982eac))
* **ui:** prepare project checkout visuals ([#655](https://github.com/radicle-dev/radicle-upstream/issues/655)) ([80c4594](https://github.com/radicle-dev/radicle-upstream/commit/80c4594ceac026fe2eaee2a364f804be0788adb2))
* **ui:** remove application menu-bar ([#598](https://github.com/radicle-dev/radicle-upstream/issues/598)) ([3bdf615](https://github.com/radicle-dev/radicle-upstream/commit/3bdf615ede72b6dc189b80c27c823eaea06bed6c))
* **ui:** remove display name and avatar url ([#499](https://github.com/radicle-dev/radicle-upstream/issues/499)) ([0b0d05d](https://github.com/radicle-dev/radicle-upstream/commit/0b0d05d4da2f72f7c7bf998681c314a4a9f7cffc))
* **ui:** restrict register org sidebar button ([#483](https://github.com/radicle-dev/radicle-upstream/issues/483)) ([0cbbe49](https://github.com/radicle-dev/radicle-upstream/commit/0cbbe495dc3fb41caa0755c134b4ca9ef43f3c68))
* **ui:** scope repository selector by peer ([#620](https://github.com/radicle-dev/radicle-upstream/issues/620)) ([e2a345b](https://github.com/radicle-dev/radicle-upstream/commit/e2a345baca68dd7790f4aa22d124e7edce13a007))
* **ui:** send funds flow ([#712](https://github.com/radicle-dev/radicle-upstream/issues/712)) ([f266600](https://github.com/radicle-dev/radicle-upstream/commit/f2666001c367e755df0483239f754dae67de44f4))
* **ui:** show actual free balance on wallet screens ([#715](https://github.com/radicle-dev/radicle-upstream/issues/715)) ([fc06cc4](https://github.com/radicle-dev/radicle-upstream/commit/fc06cc46a075573be8e4ee96edd765ed0cb54f3c))
* **ui:** show the actual identity accountId ([#711](https://github.com/radicle-dev/radicle-upstream/issues/711)) ([93b0b6d](https://github.com/radicle-dev/radicle-upstream/commit/93b0b6d7d9ee71a487db3110c7df82b2929aa16d))
* **ui:** wallet page ([#662](https://github.com/radicle-dev/radicle-upstream/issues/662)) ([f4ee709](https://github.com/radicle-dev/radicle-upstream/commit/f4ee70964c07e2c370d2e57a93e18b4e7ca7ed6e))


### Bug Fixes

* **ci:** add timezone to test:integration command ([#750](https://github.com/radicle-dev/radicle-upstream/issues/750)) ([703f319](https://github.com/radicle-dev/radicle-upstream/commit/703f31912b0f9eba42456d0919021f85c5508b53))
* **ci:** fix flaky builds due to tmp dir limit ([#710](https://github.com/radicle-dev/radicle-upstream/issues/710)) ([5decdee](https://github.com/radicle-dev/radicle-upstream/commit/5decdee8c07125348506f387da4c2aff3b8872e3))
* **ci:** make clippy work on CI again ([#748](https://github.com/radicle-dev/radicle-upstream/issues/748)) ([3941ab1](https://github.com/radicle-dev/radicle-upstream/commit/3941ab1ddff257d645c761544756c5942a589bcd))
* **ci:** retry yarn install if it fails ([#706](https://github.com/radicle-dev/radicle-upstream/issues/706)) ([55eaaf6](https://github.com/radicle-dev/radicle-upstream/commit/55eaaf62da77bb9b00ca7bcc14d199df9dae2018))
* **ci:** fix source env file ([#497](https://github.com/radicle-dev/radicle-upstream/issues/497)) ([446ec61](https://github.com/radicle-dev/radicle-upstream/commit/446ec61c93c9b142b14f80618dd57d18ddce04ac))
* **ci:** fix clippy on CI ([#430](https://github.com/radicle-dev/radicle-upstream/issues/430)) ([9729b73](https://github.com/radicle-dev/radicle-upstream/commit/9729b739cab75a85f73c7b0e113e8b0911c5ee41))
* **proxy:** add transactionFee to example inputs ([#525](https://github.com/radicle-dev/radicle-upstream/issues/525)) ([ab4716b](https://github.com/radicle-dev/radicle-upstream/commit/ab4716ba30136b2f9eae6dcfe4e07bc1f672231e))
* **proxy:** enforce camel case in transaction messages ([#449](https://github.com/radicle-dev/radicle-upstream/issues/449)) ([149ec9b](https://github.com/radicle-dev/radicle-upstream/commit/149ec9bf57a298d91df5f1b83e84fd72dc3b95ea))
* **proxy:** fix transaction subject for project registration ([#482](https://github.com/radicle-dev/radicle-upstream/issues/482)) ([776bcfb](https://github.com/radicle-dev/radicle-upstream/commit/776bcfb275c65a073d7db9026549426e29571068))
* **proxy:** fix project init ([#727](https://github.com/radicle-dev/radicle-upstream/issues/727)) ([9e6fd78](https://github.com/radicle-dev/radicle-upstream/commit/9e6fd78df13069d7e49f8b4cd7e3b41aec227a8a))
* **proxy:** fix rad remotes ([#781](https://github.com/radicle-dev/radicle-upstream/issues/781)) ([a1e3952](https://github.com/radicle-dev/radicle-upstream/commit/a1e39520e310807fbc80e68f5a4cdbd47a0ce7b2))
* **proxy:** fix proxy lifecycle on macOS ([#425](https://github.com/radicle-dev/radicle-upstream/issues/425)) ([47021db](https://github.com/radicle-dev/radicle-upstream/commit/47021dbd2c82a25b851b26c49d5cd01acbbffcaf))
* **style:** tweak typography ([#778](https://github.com/radicle-dev/radicle-upstream/issues/778)) ([faebf1b](https://github.com/radicle-dev/radicle-upstream/commit/faebf1b94776fa1c2781c250f2028ed0b1d1d825))
* **ui:** add accessible hotkeys for other layouts ([#793](https://github.com/radicle-dev/radicle-upstream/issues/793)) ([3061b06](https://github.com/radicle-dev/radicle-upstream/commit/3061b06a82c3f4d1d4ba5c3a2127171faa40ac78))
* **ui:** check id availability in org and user namespaces ([#523](https://github.com/radicle-dev/radicle-upstream/issues/523)) ([e142779](https://github.com/radicle-dev/radicle-upstream/commit/e14277939f642b3bca08b459118ebed94466f6d0))
* **ui:** fix various styles ([#755](https://github.com/radicle-dev/radicle-upstream/issues/755)) ([c794408](https://github.com/radicle-dev/radicle-upstream/commit/c794408bb029d9184d4c3a7a5f9a2c8be3bfd096))
* **ui:** fix clone button link ([#627](https://github.com/radicle-dev/radicle-upstream/issues/627)) ([8826b2d](https://github.com/radicle-dev/radicle-upstream/commit/8826b2deee806e0286009c367ad19e6a3f619f6f))
* **ui:** fix color of description text in Settings ([#584](https://github.com/radicle-dev/radicle-upstream/issues/584)) ([09280ee](https://github.com/radicle-dev/radicle-upstream/commit/09280ee8941a9c317756deee9590f933fd734020))
* **ui:** fix error display ([#615](https://github.com/radicle-dev/radicle-upstream/issues/615)) ([361fbdf](https://github.com/radicle-dev/radicle-upstream/commit/361fbdfc15ddad9e30a6ca53c7507b09d01c26ab))
* **ui:** fix org navigation ([#496](https://github.com/radicle-dev/radicle-upstream/issues/496)) ([c41480b](https://github.com/radicle-dev/radicle-upstream/commit/c41480b3892b934c5dbb9a70c2a58b216f02bfd1))
* **ui:** fix project registration button permissions ([#699](https://github.com/radicle-dev/radicle-upstream/issues/699)) ([d31cb5c](https://github.com/radicle-dev/radicle-upstream/commit/d31cb5c02c9f9f95098e9db441ae4c3eadb705bf))
* **ui:** fix remote helper export ([#787](https://github.com/radicle-dev/radicle-upstream/issues/787)) ([a91bf00](https://github.com/radicle-dev/radicle-upstream/commit/a91bf00a2a28a385aace551ad1bc1740f26d88a0))
* **ui:** fix repository selector regressions ([#632](https://github.com/radicle-dev/radicle-upstream/issues/632)) ([3d74608](https://github.com/radicle-dev/radicle-upstream/commit/3d74608ccecb90780bee0a8548e22574d6d2c067))
* **ui:** fix various routing issues ([#595](https://github.com/radicle-dev/radicle-upstream/issues/595)) ([df7cfdc](https://github.com/radicle-dev/radicle-upstream/commit/df7cfdcdb51d9fa2534f2444dfecaf57e629cf13)), closes [#564](https://github.com/radicle-dev/radicle-upstream/issues/564)
* **ui:** open external links in default OS browser ([#634](https://github.com/radicle-dev/radicle-upstream/issues/634)) ([aa6c01a](https://github.com/radicle-dev/radicle-upstream/commit/aa6c01addc3156012832dab09e7fbee2192474f2))
* **ui:** fix race condition in commit navigation ([#702](https://github.com/radicle-dev/radicle-upstream/issues/702)) ([343e859](https://github.com/radicle-dev/radicle-upstream/commit/343e85907272808fbccd843eefcff6588007d50c))
* **ui:** remove flicker in commits view ([#611](https://github.com/radicle-dev/radicle-upstream/issues/611)) ([25d9a79](https://github.com/radicle-dev/radicle-upstream/commit/25d9a79f3e43fe3c4a52dd2a2f26aef21f3a60c1))
* **ui:** show correct commit branch ([#673](https://github.com/radicle-dev/radicle-upstream/issues/673)) ([159f860](https://github.com/radicle-dev/radicle-upstream/commit/159f8601b7feb499b21fe0e7ab6416c0bf9ad198))
* **ui:** fix transaction center toggle behavior ([3c27e16](https://github.com/radicle-dev/radicle-upstream/commit/3c27e16715033e2f3588d19f31d5d67dda77fe0f)), closes [#517](https://github.com/radicle-dev/radicle-upstream/issues/517)
* **ui:** update emojis with full set ([#485](https://github.com/radicle-dev/radicle-upstream/issues/485)) ([a82bd87](https://github.com/radicle-dev/radicle-upstream/commit/a82bd870c4fe9628dc8189795f182d1baf73d131))
* **ui:** update source browsing ([#765](https://github.com/radicle-dev/radicle-upstream/issues/765)) ([f801198](https://github.com/radicle-dev/radicle-upstream/commit/f8011981305582bb9da07de77e31c39c5471974e))
* **ui:** fix commits view ([#574](https://github.com/radicle-dev/radicle-upstream/issues/574)) ([13633a9](https://github.com/radicle-dev/radicle-upstream/commit/13633a969f36b976cf52561c209e92d9dd35a9e3))
* **ui:** show ProfileProjects page on startup ([#538](https://github.com/radicle-dev/radicle-upstream/issues/538)) ([506b83c](https://github.com/radicle-dev/radicle-upstream/commit/506b83c3c08c654d7ecaa46cb887ccb1490a6d85))
* **ui:** fix missing revision icon ([bb3f397](https://github.com/radicle-dev/radicle-upstream/commit/bb3f397b7022fa1ec3ba69680fc2966d4a5a74ae))

### [0.0.11](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.10...v0.0.11) (2020-05-25)


### Features

* **proxy:** extend session with settings ([#383](https://github.com/radicle-dev/radicle-upstream/issues/383)) ([9f6c4ff](https://github.com/radicle-dev/radicle-upstream/commit/9f6c4ffd8e3517fb224696c13da539d4f6752f47)), closes [#385](https://github.com/radicle-dev/radicle-upstream/issues/385) [#128](https://github.com/radicle-dev/radicle-upstream/issues/128) [#375](https://github.com/radicle-dev/radicle-upstream/issues/375)
* **proxy:** implement settings endpoints ([#389](https://github.com/radicle-dev/radicle-upstream/issues/389)) ([6b9446c](https://github.com/radicle-dev/radicle-upstream/commit/6b9446ca9ebcc17a17b1809924a5bbf4f65ae008)), closes [#385](https://github.com/radicle-dev/radicle-upstream/issues/385)
* **ui:** add org members list ([#402](https://github.com/radicle-dev/radicle-upstream/issues/402)) ([97187e0](https://github.com/radicle-dev/radicle-upstream/commit/97187e0dcde336af25245065bc538732e9fc4d83)), closes [#349](https://github.com/radicle-dev/radicle-upstream/issues/349)
* **ui:** streamline transaction formatting ([#373](https://github.com/radicle-dev/radicle-upstream/issues/373)) ([1f38924](https://github.com/radicle-dev/radicle-upstream/commit/1f38924820ba45afe8afb837d15d094ad75c6152)), closes [#347](https://github.com/radicle-dev/radicle-upstream/issues/347)
* **ui:** wire up settings endpoints ([#390](https://github.com/radicle-dev/radicle-upstream/issues/390)) ([d84f604](https://github.com/radicle-dev/radicle-upstream/commit/d84f6047fd4d2eba967c05e7ab3574b0d83752c8)), closes [#385](https://github.com/radicle-dev/radicle-upstream/issues/385)
* **ui:** wire up shareable entity identifiers ([#393](https://github.com/radicle-dev/radicle-upstream/issues/393)) ([a696875](https://github.com/radicle-dev/radicle-upstream/commit/a696875676957b969d40a45a3d934611c2de2d3e))
* implement tx polling ([#407](https://github.com/radicle-dev/radicle-upstream/issues/407)) ([9573df0](https://github.com/radicle-dev/radicle-upstream/commit/9573df08dd3bf2863190969b52222b569c51eebc)), closes [#347](https://github.com/radicle-dev/radicle-upstream/issues/347)


### Bug Fixes

* **proxy:** only fetch commit for root tree ([#405](https://github.com/radicle-dev/radicle-upstream/issues/405)) ([cfd88b6](https://github.com/radicle-dev/radicle-upstream/commit/cfd88b62f6c951dec0abf5c2448b9e65f0abaa7a)), closes [#350](https://github.com/radicle-dev/radicle-upstream/issues/350)
* **ui:** check for project name length before registration ([#410](https://github.com/radicle-dev/radicle-upstream/issues/410)) ([7cd7761](https://github.com/radicle-dev/radicle-upstream/commit/7cd77616d0b6eb5799b3502ab22953e00f76fe74))
* **ui:** add padding to issues screen [#412](https://github.com/radicle-dev/radicle-upstream/issues/412) ([2c3c4a9](https://github.com/radicle-dev/radicle-upstream/commit/2c3c4a918b8a6d4020bad6e3f5a551a80a299997))
* **ui:** show correct avatars in summary pages ([#399](https://github.com/radicle-dev/radicle-upstream/issues/399)) ([01053c1](https://github.com/radicle-dev/radicle-upstream/commit/01053c145d49cc18266fce01a010826162cf00c2))

### [0.0.10](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.9...v0.0.10) (2020-05-13)


### Features

* **build:** improve dev ergonomics ([#379](https://github.com/radicle-dev/radicle-upstream/issues/379)) ([d929a1a](https://github.com/radicle-dev/radicle-upstream/commit/d929a1a9cfaaf0dc11590329c0032da83e9ed261))
* **proxy:** add avatar endpoint ([#330](https://github.com/radicle-dev/radicle-upstream/issues/330)) ([d51bcfc](https://github.com/radicle-dev/radicle-upstream/commit/d51bcfcdbba10563d29a646d7fb9f30ec4c1f872))
* **proxy:** expose current user orgs list in session ([#341](https://github.com/radicle-dev/radicle-upstream/issues/341)) ([9535b18](https://github.com/radicle-dev/radicle-upstream/commit/9535b18d0e5953ea36b3ed76e75854e9a671523b)), closes [#340](https://github.com/radicle-dev/radicle-upstream/issues/340)
* **proxy:** fetch registered project ([#322](https://github.com/radicle-dev/radicle-upstream/issues/322)) ([b75c7eb](https://github.com/radicle-dev/radicle-upstream/commit/b75c7eb56ebd9664f2abe536b851a92b81dba03f))
* **proxy:** improve session ([#380](https://github.com/radicle-dev/radicle-upstream/issues/380)) ([c698330](https://github.com/radicle-dev/radicle-upstream/commit/c698330f741756e7fe2cd3a7c1a2e444a593ad6b)), closes [#378](https://github.com/radicle-dev/radicle-upstream/issues/378)
* **proxy:** add org project list endpoint ([#343](https://github.com/radicle-dev/radicle-upstream/issues/343)) ([dd3433f](https://github.com/radicle-dev/radicle-upstream/commit/dd3433f1611193b31667307267f38ece1e5c507c))
* **proxy:** persist transactions ([#370](https://github.com/radicle-dev/radicle-upstream/issues/370)) ([fbc7016](https://github.com/radicle-dev/radicle-upstream/commit/fbc70162388846cf9abe764dc8b7883a326518f4))
* **proxy:** return member list for an org ([#360](https://github.com/radicle-dev/radicle-upstream/issues/360)) ([ae0ca4f](https://github.com/radicle-dev/radicle-upstream/commit/ae0ca4f98c7d7b9e47236bbf9860c45f42f617ba))
* **ui:** add commit history view ([#337](https://github.com/radicle-dev/radicle-upstream/issues/337)) ([50eceb9](https://github.com/radicle-dev/radicle-upstream/commit/50eceb9b352986e5e5b136409c4d39d3c32b4eed))
* **ui:** finalize design on project source view ([#311](https://github.com/radicle-dev/radicle-upstream/issues/311)) ([71864b3](https://github.com/radicle-dev/radicle-upstream/commit/71864b36a5e2615efae70ec1ba40ddf960c700d1))
* **ui:** fix input avatars for org & member registration ([#366](https://github.com/radicle-dev/radicle-upstream/issues/366)) ([965f8e1](https://github.com/radicle-dev/radicle-upstream/commit/965f8e1efc4b7e4f72bff05c2596738fa320a09c))
* **ui:** implement 'Clone' button ([#329](https://github.com/radicle-dev/radicle-upstream/issues/329)) ([9685ccb](https://github.com/radicle-dev/radicle-upstream/commit/9685ccb5816b0f08772d0b90ed7730d0ad0da98b))
* **ui:** implement tracking button ([#325](https://github.com/radicle-dev/radicle-upstream/issues/325)) ([5290146](https://github.com/radicle-dev/radicle-upstream/commit/5290146bbf1210884bd26eb9bbf0829ddc32caf9))
* **ui:** add org registration visuals & validation store ([#280](https://github.com/radicle-dev/radicle-upstream/issues/280)) ([9e62e30](https://github.com/radicle-dev/radicle-upstream/commit/9e62e309a533dabafe0628944266b2cc03a02920))
* **ui:** implement project registration flow ([#292](https://github.com/radicle-dev/radicle-upstream/issues/292)) ([b5d4046](https://github.com/radicle-dev/radicle-upstream/commit/b5d40460f8af7cf896efbebc9b0a247e0e1ee0b4))
* **ui:** add SegmentedControl component ([#377](https://github.com/radicle-dev/radicle-upstream/issues/377)) ([6c92ebd](https://github.com/radicle-dev/radicle-upstream/commit/6c92ebda328d42a1f66b6421c9be560eabbfc443))
* **ui:** integrate org registration flow ([#333](https://github.com/radicle-dev/radicle-upstream/issues/333)) ([c3b33b5](https://github.com/radicle-dev/radicle-upstream/commit/c3b33b5d3080dc48a4382375f671f77c5b49ea35))
* **ui:** use Twitter SVG emojis in Avatar component ([#339](https://github.com/radicle-dev/radicle-upstream/issues/339)) ([75ba3d0](https://github.com/radicle-dev/radicle-upstream/commit/75ba3d0405765e6937c92060c484d4b02ed42149)), closes [#231](https://github.com/radicle-dev/radicle-upstream/issues/231) [#290](https://github.com/radicle-dev/radicle-upstream/issues/290)
* **ui:** implement visuals for add member to org flow ([#334](https://github.com/radicle-dev/radicle-upstream/issues/334)) ([9aede9c](https://github.com/radicle-dev/radicle-upstream/commit/9aede9ca61e62444ea8fb1b5bb61c37f82329006)), closes [#364](https://github.com/radicle-dev/radicle-upstream/issues/364)
* **ui:** wire up org project list ([#361](https://github.com/radicle-dev/radicle-upstream/issues/361)) ([fa49629](https://github.com/radicle-dev/radicle-upstream/commit/fa49629ba39667d4be9ba93a5d4cb735f554e569))
* **ui:** wire up orgs in sidebar ([#345](https://github.com/radicle-dev/radicle-upstream/issues/345)) ([eac1cfd](https://github.com/radicle-dev/radicle-upstream/commit/eac1cfd901e7e74b14560d7626805883387ed162))


### Bug Fixes

* **build:** bundle missing dependencies ([#351](https://github.com/radicle-dev/radicle-upstream/issues/351)) ([e7e04f9](https://github.com/radicle-dev/radicle-upstream/commit/e7e04f9378de486e4c91997fb4feb2f1ce3c4abd))
* **proxy:** correct attestion field name in user ([#336](https://github.com/radicle-dev/radicle-upstream/issues/336)) ([04e4892](https://github.com/radicle-dev/radicle-upstream/commit/04e4892e205638fc618badf8066b8ec670b8da44))
* **proxy:** remove 🌱 from whitelist ([#338](https://github.com/radicle-dev/radicle-upstream/issues/338)) ([d17e348](https://github.com/radicle-dev/radicle-upstream/commit/d17e348ebb386b360871d6358a52563e9b31adeb))
* **ui:** fix identity creation regression ([#354](https://github.com/radicle-dev/radicle-upstream/issues/354)) ([1e08134](https://github.com/radicle-dev/radicle-upstream/commit/1e081341f29cbb560fd7c2d6385fe959cda44446)), closes [#353](https://github.com/radicle-dev/radicle-upstream/issues/353)

### [0.0.9](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.8...v0.0.9) (2020-04-29)


### Features

* **proxy:** list all orgs by member ([#309](https://github.com/radicle-dev/radicle-upstream/issues/309)) ([a10676c](https://github.com/radicle-dev/radicle-upstream/commit/a10676c820dd6e7bbfff9784027c0c658fda137e))
* **proxy:** org API endpoints ([#300](https://github.com/radicle-dev/radicle-upstream/issues/300)) ([b480bc4](https://github.com/radicle-dev/radicle-upstream/commit/b480bc415f06920a82cc0af0597b6926441dc0d9))
* **ui:** commit changeset UI ([#298](https://github.com/radicle-dev/radicle-upstream/issues/298)) ([7475fbf](https://github.com/radicle-dev/radicle-upstream/commit/7475fbf855944a80b5c9c4510234ee7b6412a883))
* **ui:** new project page with repository selector ([#294](https://github.com/radicle-dev/radicle-upstream/issues/294)) ([f72b207](https://github.com/radicle-dev/radicle-upstream/commit/f72b207cae6bc1583cdea938a73b66785868fd65))
* **ui:** restyle notification banners ([#301](https://github.com/radicle-dev/radicle-upstream/issues/301)) ([ce7b1b2](https://github.com/radicle-dev/radicle-upstream/commit/ce7b1b2a53eeb6ef1b40150fbc05d91c9ee08bf4))
* **ui:** style forms to match new design system ([#315](https://github.com/radicle-dev/radicle-upstream/issues/315)) ([8fa9758](https://github.com/radicle-dev/radicle-upstream/commit/8fa9758df76672d9a84e6319ccab30eb7092ab03))
* **ui:** update buttons to match new design system ([#316](https://github.com/radicle-dev/radicle-upstream/issues/316)) ([68b1ddb](https://github.com/radicle-dev/radicle-upstream/commit/68b1ddbda5462b9670e8c428e205b452edff1290)), closes [#312](https://github.com/radicle-dev/radicle-upstream/issues/312)
* source browsing in the new API ([#296](https://github.com/radicle-dev/radicle-upstream/issues/296)) ([8b08279](https://github.com/radicle-dev/radicle-upstream/commit/8b0827916c16ac07d47dbc10af32fc95a2492b41)), closes [#293](https://github.com/radicle-dev/radicle-upstream/issues/293)
* switch to REST API and centralised store ([#293](https://github.com/radicle-dev/radicle-upstream/issues/293)) ([7fe30e1](https://github.com/radicle-dev/radicle-upstream/commit/7fe30e1685761abe0b2028592534b17cf8eed3d9))
* transactions through REST API ([#305](https://github.com/radicle-dev/radicle-upstream/issues/305)) ([77e6ec5](https://github.com/radicle-dev/radicle-upstream/commit/77e6ec5366fccc24bd3afab906b56d7eed06c7bd)), closes [#293](https://github.com/radicle-dev/radicle-upstream/issues/293) [#225](https://github.com/radicle-dev/radicle-upstream/issues/225)


### Bug Fixes

* **ui:** project creation ([#319](https://github.com/radicle-dev/radicle-upstream/issues/319)) ([730e31b](https://github.com/radicle-dev/radicle-upstream/commit/730e31bd53aac0a7c02687ad32b8033ada0f9fcf)), closes [#312](https://github.com/radicle-dev/radicle-upstream/issues/312)
* **ui:** fix routing on browser reload ([#297](https://github.com/radicle-dev/radicle-upstream/issues/297)) ([22d9a26](https://github.com/radicle-dev/radicle-upstream/commit/22d9a26b0c8302d99e6e8ad264a825828828bd61)), closes [#266](https://github.com/radicle-dev/radicle-upstream/issues/266)
* **ui:** fix start script ([#318](https://github.com/radicle-dev/radicle-upstream/issues/318)) ([365cff2](https://github.com/radicle-dev/radicle-upstream/commit/365cff244cb75ebf8a0ec6d841225fca9ca37cef))
* **ui:** timely session fetch ([#314](https://github.com/radicle-dev/radicle-upstream/issues/314)) ([5a545c2](https://github.com/radicle-dev/radicle-upstream/commit/5a545c2fd057339cf78bcaf09fa5a6b616e609db)), closes [#313](https://github.com/radicle-dev/radicle-upstream/issues/313)

### [0.0.8](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.7...v0.0.8) (2020-04-15)


### Features

* **proxy:** make avatar generation id based ([#267](https://github.com/radicle-dev/radicle-upstream/issues/267)) ([ba38a86](https://github.com/radicle-dev/radicle-upstream/commit/ba38a86b36b9d9cb8353eadb90dda2599f0702f3)), closes [#230](https://github.com/radicle-dev/radicle-upstream/issues/230)
* **ui:** fetch identity on app boot ([#266](https://github.com/radicle-dev/radicle-upstream/issues/266)) ([08b1c15](https://github.com/radicle-dev/radicle-upstream/commit/08b1c15d46018732863a9214476f3bf9f3282349))
* **ui:** single commit view header ([#278](https://github.com/radicle-dev/radicle-upstream/issues/278)) ([4d674f3](https://github.com/radicle-dev/radicle-upstream/commit/4d674f33838af19880e7de9772f404a7f38dc6ed))
* **ui:** implement transaction center and detail view ([#242](https://github.com/radicle-dev/radicle-upstream/issues/242)) ([3cf62b6](https://github.com/radicle-dev/radicle-upstream/commit/3cf62b6aa73322152209d195e4be0390e18dd8a3))
* **ui:** align project creation UX to new designs ([#264](https://github.com/radicle-dev/radicle-upstream/issues/264)) ([c98a082](https://github.com/radicle-dev/radicle-upstream/commit/c98a0829cc889bebd756490aebe92b77e86da55e))
* **ui:** adjust misc Basic user identity I visuals ([#263](https://github.com/radicle-dev/radicle-upstream/issues/263)) ([80d938a](https://github.com/radicle-dev/radicle-upstream/commit/80d938a34bcc90bc407db8af9bf18439d9e49f83))


### Bug Fixes

* **infra:** revert to latest version of git-platinum ([#273](https://github.com/radicle-dev/radicle-upstream/issues/273)) ([f540430](https://github.com/radicle-dev/radicle-upstream/commit/f540430b4bd40a1632eec988e292a3f69b90838b))

### [0.0.7](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.5...v0.0.7) (2020-04-01)


### Features

* **proxy:** add kind to transaction messages ([#258](https://github.com/radicle-dev/radicle-upstream/issues/258)) ([abc6ab4](https://github.com/radicle-dev/radicle-upstream/commit/abc6ab40ddf18c5532dd78f20aa46bcfedfa1f55)), closes [#253](https://github.com/radicle-dev/radicle-upstream/issues/253)
* **proxy:** add official avatar list and usage ([#222](https://github.com/radicle-dev/radicle-upstream/issues/222)) ([660fa31](https://github.com/radicle-dev/radicle-upstream/commit/660fa313cd88a3f08aa944f2f31d5084ed1f8e8f))
* **proxy:** add thresholds to transaction list ([#257](https://github.com/radicle-dev/radicle-upstream/issues/257)) ([ef65ad9](https://github.com/radicle-dev/radicle-upstream/commit/ef65ad9cbf0ebe22a3c2c7e44b35f0556a40b2fb)), closes [#254](https://github.com/radicle-dev/radicle-upstream/issues/254)
* **proxy:** extend identity with registered field ([#256](https://github.com/radicle-dev/radicle-upstream/issues/256)) ([78d69f0](https://github.com/radicle-dev/radicle-upstream/commit/78d69f0ff2370568021744af6c9055bbeac68eb1)), closes [#255](https://github.com/radicle-dev/radicle-upstream/issues/255)
* **proxy:** extend project with org/user relation ([#251](https://github.com/radicle-dev/radicle-upstream/issues/251)) ([7c2a424](https://github.com/radicle-dev/radicle-upstream/commit/7c2a424244e66a15f0f55381ccdf6ca82e3bb44f)), closes [#245](https://github.com/radicle-dev/radicle-upstream/issues/245)
* **proxy:** integrate transaction fees ([#227](https://github.com/radicle-dev/radicle-upstream/issues/227)) ([e1b7572](https://github.com/radicle-dev/radicle-upstream/commit/e1b7572a0364e81f43ae2541c53663ca82f6000b)), closes [radicle-dev/radicle-registry#255](https://github.com/radicle-dev/radicle-registry/issues/255)
* **proxy:** naive tx cache ([#247](https://github.com/radicle-dev/radicle-upstream/issues/247)) ([cb328b9](https://github.com/radicle-dev/radicle-upstream/commit/cb328b91028d691c863623e3e5719b15399aeee3))
* **proxy:** wire up Registry user registration ([#238](https://github.com/radicle-dev/radicle-upstream/issues/238)) ([8859037](https://github.com/radicle-dev/radicle-upstream/commit/8859037812924155dcc6e4ab6bcc8855a7553004)), closes [radicle-dev/radicle-registry#249](https://github.com/radicle-dev/radicle-registry/issues/249) [#185](https://github.com/radicle-dev/radicle-upstream/issues/185)
* **ui:** copyable component ([#180](https://github.com/radicle-dev/radicle-upstream/issues/180))  ([0bcfa2c](https://github.com/radicle-dev/radicle-upstream/commit/0bcfa2c65a385ecd9557655c629874c41ab470a7))
* **ui:** implement new color system ([#261](https://github.com/radicle-dev/radicle-upstream/issues/261)) ([aa0066c](https://github.com/radicle-dev/radicle-upstream/commit/aa0066c9e61b6c08e9b16c7e5b900167a1cf9872))
* **ui:** implement new navigation ([#232](https://github.com/radicle-dev/radicle-upstream/issues/232)) ([42ce1b9](https://github.com/radicle-dev/radicle-upstream/commit/42ce1b9e8cea0f5ebcedfcbc4df65c4c5744b506)), closes [#186](https://github.com/radicle-dev/radicle-upstream/issues/186)
* **ui:** extend Avatar with new fallback data ([#221](https://github.com/radicle-dev/radicle-upstream/issues/221)) ([fc2f7bd](https://github.com/radicle-dev/radicle-upstream/commit/fc2f7bdcd571b86e8c32a9333d67bbc2574d1b30))
* **ui:** implement transaction state icon ([#235](https://github.com/radicle-dev/radicle-upstream/issues/235)) ([15050c8](https://github.com/radicle-dev/radicle-upstream/commit/15050c873aa1f2151ba118287a646c5ee2831c02))
* **ui:** new identity flow ([#211](https://github.com/radicle-dev/radicle-upstream/issues/211)) ([72c522e](https://github.com/radicle-dev/radicle-upstream/commit/72c522ecaa277d05e71c0be3bb36c7b546078793))
* **ui:** new step counter ([#210](https://github.com/radicle-dev/radicle-upstream/issues/210)) ([a807551](https://github.com/radicle-dev/radicle-upstream/commit/a807551fc7bb06d8bfe8c1a6c2a94a0eb0687115))
* **ui:** user handle input component ([#205](https://github.com/radicle-dev/radicle-upstream/issues/205)) ([fa782ee](https://github.com/radicle-dev/radicle-upstream/commit/fa782ee0ad98a0df48ea3fcbcbe62765c25af8f9))
* **ui:** user handle registration modal ([#216](https://github.com/radicle-dev/radicle-upstream/issues/216)) ([eade724](https://github.com/radicle-dev/radicle-upstream/commit/eade7241e3892334697cb7411edd0a6b5688b863))


### Bug Fixes
* **proxy:** convert registry validation errors correctly ([#237](https://github.com/radicle-dev/radicle-upstream/issues/237)) ([1a3e24e](https://github.com/radicle-dev/radicle-upstream/commit/1a3e24e611ac507fd044202abe346cd94d1b87e9))
* **proxy:** swap proxy flags to respect emulator ([#223](https://github.com/radicle-dev/radicle-upstream/issues/223)) ([1a8a5c8](https://github.com/radicle-dev/radicle-upstream/commit/1a8a5c8f88e7c91f9175970287d2908ee8fb07f3))
* **ui:** fix transaction spinner icon alignment ([#248](https://github.com/radicle-dev/radicle-upstream/issues/248)) ([fc5cfb0](https://github.com/radicle-dev/radicle-upstream/commit/fc5cfb09478f1c58b9c1eee40b0071908ab64ce0))

### [0.0.6](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.5...v0.0.6) (2020-03-18)


### Features

* extend Avatar with new fallback data ([#221](https://github.com/radicle-dev/radicle-upstream/issues/221)) ([fc2f7bd](https://github.com/radicle-dev/radicle-upstream/commit/fc2f7bdcd571b86e8c32a9333d67bbc2574d1b30))
* extend schema with avatar fallback ([#218](https://github.com/radicle-dev/radicle-upstream/issues/218)) ([0593af7](https://github.com/radicle-dev/radicle-upstream/commit/0593af7bbfc6e55a54698516f2eef02ba1e97f99))
* integrate transaction fees ([#227](https://github.com/radicle-dev/radicle-upstream/issues/227)) ([e1b7572](https://github.com/radicle-dev/radicle-upstream/commit/e1b7572a0364e81f43ae2541c53663ca82f6000b)), closes [radicle-dev/radicle-registry#255](https://github.com/radicle-dev/radicle-registry/issues/255)
* implement basic avatar generation ([#217](https://github.com/radicle-dev/radicle-upstream/issues/217)) ([aa88f62](https://github.com/radicle-dev/radicle-upstream/commit/aa88f62343ef35b5d03a6ad5d15149537c4edb0b))
* new step counter ([#210](https://github.com/radicle-dev/radicle-upstream/issues/210)) ([a807551](https://github.com/radicle-dev/radicle-upstream/commit/a807551fc7bb06d8bfe8c1a6c2a94a0eb0687115))
* user handle input component ([#205](https://github.com/radicle-dev/radicle-upstream/issues/205)) ([fa782ee](https://github.com/radicle-dev/radicle-upstream/commit/fa782ee0ad98a0df48ea3fcbcbe62765c25af8f9))
* user handle registration modal ([#216](https://github.com/radicle-dev/radicle-upstream/issues/216)) ([eade724](https://github.com/radicle-dev/radicle-upstream/commit/eade7241e3892334697cb7411edd0a6b5688b863))


### Bug Fixes

* add official avatar list and usage ([#222](https://github.com/radicle-dev/radicle-upstream/issues/222)) ([660fa31](https://github.com/radicle-dev/radicle-upstream/commit/660fa313cd88a3f08aa944f2f31d5084ed1f8e8f))
* swap proxy flags to respect emulator ([#223](https://github.com/radicle-dev/radicle-upstream/issues/223)) ([1a8a5c8](https://github.com/radicle-dev/radicle-upstream/commit/1a8a5c8f88e7c91f9175970287d2908ee8fb07f3))

### [0.0.5](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.1...v0.0.5) (2020-03-04)


### Features

* add hotkey for escaping modals ([#196](https://github.com/radicle-dev/radicle-upstream/issues/196)) ([e1afb44](https://github.com/radicle-dev/radicle-upstream/commit/e1afb44467d80f6b786d846db1a32283ca19f6aa))
* automate releases ([#49](https://github.com/radicle-dev/radicle-upstream/issues/49)) ([1b66743](https://github.com/radicle-dev/radicle-upstream/commit/1b6674353621c144ae6360d8a52477cfab468a05))
* dropdown menu component ([#193](https://github.com/radicle-dev/radicle-upstream/issues/193)) ([ee4059d](https://github.com/radicle-dev/radicle-upstream/commit/ee4059d3a3dd54818d20b3c81582fc8c6fd9073b))
* extract transaction overview ([#200](https://github.com/radicle-dev/radicle-upstream/issues/200)) ([32233aa](https://github.com/radicle-dev/radicle-upstream/commit/32233aa5ae1ea673185d8f1973f76b7893afb799))
* implement Basic User Identity mocks ([#199](https://github.com/radicle-dev/radicle-upstream/issues/199)) ([2bb94e0](https://github.com/radicle-dev/radicle-upstream/commit/2bb94e0a7c2d74f294b60187d910489d8dc5b380))
* implement one-way attestation ([#105](https://github.com/radicle-dev/radicle-upstream/issues/105)) ([d596216](https://github.com/radicle-dev/radicle-upstream/commit/d596216251b62b446e1627aaa27587648ddfae9f))
* migrate from project domain to org ([#154](https://github.com/radicle-dev/radicle-upstream/issues/154)) ([c023cec](https://github.com/radicle-dev/radicle-upstream/commit/c023cec82cd3a500f1bd5f5bca0844deb7c1716b))
* project list visuals ([#187](https://github.com/radicle-dev/radicle-upstream/issues/187)) ([7c2e271](https://github.com/radicle-dev/radicle-upstream/commit/7c2e271b54231148c710e199aa11e305e1128d11))
* restyle sidebar to match current design ([#188](https://github.com/radicle-dev/radicle-upstream/issues/188)) ([c5e9123](https://github.com/radicle-dev/radicle-upstream/commit/c5e9123c37c249f01d91ae23a9fc918f51ebe1fb))
* set up Registry client against devnet ([#106](https://github.com/radicle-dev/radicle-upstream/issues/106)) ([0229eb5](https://github.com/radicle-dev/radicle-upstream/commit/0229eb5547880aba713c39ceb85b1492a8f0e702))
* show librad project repository ([#86](https://github.com/radicle-dev/radicle-upstream/issues/86)) ([17ab237](https://github.com/radicle-dev/radicle-upstream/commit/17ab2377c24104dee895438fca0d8aa0d10fd83f))
* show registered projects ([#108](https://github.com/radicle-dev/radicle-upstream/issues/108)) ([a991044](https://github.com/radicle-dev/radicle-upstream/commit/a9910446713e6551329d88d000e3171e248f2840))
* switch to new surf revparser ([#152](https://github.com/radicle-dev/radicle-upstream/issues/152)) ([44606dd](https://github.com/radicle-dev/radicle-upstream/commit/44606dd690298e796307890c7c964035a2684b89)), closes [#139](https://github.com/radicle-dev/radicle-upstream/issues/139)


### Bug Fixes

* fix app distribution ([#206](https://github.com/radicle-dev/radicle-upstream/issues/206)) ([5e6dac0](https://github.com/radicle-dev/radicle-upstream/commit/5e6dac0db67f9cc909fc6c07cb0d3725940e0c8e))
* fix eslint setup for svelte ([#170](https://github.com/radicle-dev/radicle-upstream/issues/170)) ([82514d5](https://github.com/radicle-dev/radicle-upstream/commit/82514d51d4d8dee4ba88d352f8f84dc1f5c319cc))
* fix project registration UI regression ([#161](https://github.com/radicle-dev/radicle-upstream/issues/161)) ([43b9838](https://github.com/radicle-dev/radicle-upstream/commit/43b98381c0a4f49b83139016d11b4f037f290d71))
* fix release script ([#208](https://github.com/radicle-dev/radicle-upstream/issues/208)) ([8133c84](https://github.com/radicle-dev/radicle-upstream/commit/8133c847e35d6bc4ca7cdbb15781f2859f3979b6))
* respect proxy cli arguments ([#174](https://github.com/radicle-dev/radicle-upstream/issues/174)) ([5e0cb5e](https://github.com/radicle-dev/radicle-upstream/commit/5e0cb5e08c5af5f33b72d16917cff5e3db20f7a4)), closes [#173](https://github.com/radicle-dev/radicle-upstream/issues/173)

### [0.0.4](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.3...v0.0.4) (2020-01-28)

### [0.0.3](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.2...v0.0.3) (2020-01-28)

### [0.0.2](https://github.com/radicle-dev/radicle-upstream/compare/v0.0.1...v0.0.2) (2020-01-23)
