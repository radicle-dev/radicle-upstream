<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as notification from "ui/src/notification";
  import * as router from "ui/src/router";

  import { ValidationStatus } from "ui/src/validation";
  import {
    theme,
    themeOptions,
    codeFont,
    codeFontOptions,
    uiFont,
    uiFontOptions,
    primaryColor,
    primaryColorOptions,
    primaryColorHex,
  } from "ui/src/appearance";

  import {
    Avatar,
    Button,
    Checkbox,
    SegmentedColorPicker,
    Copyable,
    CopyableIdentifier,
    Dropdown,
    FollowToggle,
    Icon,
    IdentifierLink,
    Notification,
    SegmentedControl,
    Spinner,
    SupportButton,
    TextInput,
    ThreeDotsMenu,
    Tooltip,
  } from "ui/DesignSystem";

  import Section from "./DesignSystemGuideModal/Section.svelte";
  import TypographySwatch from "./DesignSystemGuideModal/TypographySwatch.svelte";
  import ColorSwatch from "./DesignSystemGuideModal/ColorSwatch.svelte";

  // TODO: fix types on this.
  const colors = Array.from(document.styleSheets)
    .filter(
      sheet =>
        sheet.href === null || sheet.href.startsWith(window.location.origin)
    )
    .reduce<string[]>(
      (acc, sheet) =>
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        (acc = [
          ...acc,
          // eslint-disable-next-line @typescript-eslint/ban-ts-comment
          // @ts-ignore
          ...Array.from(sheet.cssRules).reduce(
            (def, rule) =>
              // eslint-disable-next-line @typescript-eslint/ban-ts-comment
              // @ts-ignore
              (def =
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                rule.selectorText === ":root"
                  ? [
                      ...def,
                      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                      // @ts-ignore
                      ...Array.from(rule.style).filter(name =>
                        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                        // @ts-ignore
                        name.startsWith("--color")
                      ),
                    ]
                  : def),
            []
          ),
        ]),
      []
    );

  const colorGroups = [
    ...new Set(
      colors.map(color => {
        const match = color.match(/--color-(\w*)-?/);
        if (match) {
          return match[1];
        } else {
          return "";
        }
      })
    ),
  ];

  function onKeydown(event: KeyboardEvent) {
    if (event.target === document.body && event.code === "Escape") {
      router.pop();
    }
  }

  // TODO: fix types on this.
  function getIconComponent(name: string) {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    return Icon[name];
  }
</script>

<style>
  .close-button {
    cursor: pointer;
    position: absolute;
    right: 32px;
    top: 22px;
  }

  .fullscreen {
    align-items: center;
    display: flex;
    height: 100vh;
    justify-content: center;
    overflow: auto;
    width: 100vw;
  }

  .content {
    overflow: visible;
    height: 100%;
    width: 100%;
  }

  table {
    margin-bottom: 32px;
  }

  td {
    vertical-align: middle;
    padding: 5px;
  }

  .layout {
    padding: 32px;
  }

  .swatch {
    display: flex;
    margin-bottom: 32px;
    align-items: flex-end;
  }

  .icon-swatch {
    display: grid;
    grid-template-columns: repeat(10, 1.5rem);
    grid-gap: 1.5rem;
    margin-bottom: 34px;
  }

  .settings {
    right: 132px;
    top: 24px;
    position: absolute;
    z-index: 10;
    display: flex;
    align-items: center;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="close-button">
  <Button
    style="padding:0.5rem;"
    on:click={() => {
      router.pop();
    }}
    variant="transparent">
    <Icon.Cross />
  </Button>
</div>

<div class="fullscreen">
  <div class="content">
    <div class="layout">
      <div class="settings">
        <Tooltip value="Theme" position="bottom">
          <SegmentedControl
            style="background-color: var(--color-background); margin-right: 2rem;"
            active={$theme}
            options={themeOptions}
            on:select={ev => theme.set(ev.detail)} />
        </Tooltip>
        <Tooltip value="Primary color" position="bottom">
          <div style="display: flex;">
            <SegmentedColorPicker
              active={$primaryColor}
              options={primaryColorOptions}
              on:select={ev => primaryColor.set(ev.detail)}
              bind:colorValue={$primaryColorHex} />
          </div>
        </Tooltip>
        <Tooltip value="UI font" position="bottom">
          <SegmentedControl
            style="background-color: var(--color-background); margin: 0 2rem;"
            active={$uiFont}
            options={uiFontOptions}
            on:select={ev => uiFont.set(ev.detail)} />
        </Tooltip>
        <Tooltip value="Code font" position="bottom">
          <SegmentedControl
            style="background-color: var(--color-background); margin-right: 2rem;"
            active={$codeFont}
            options={codeFontOptions}
            on:select={ev => codeFont.set(ev.detail)} />
        </Tooltip>
      </div>
      <h1 style="margin-bottom: 92px">Primitives</h1>

      <Section title="Colors" subTitle="Primary and grays">
        {#each colorGroups as colorGroup}
          <div>
            {#each colors.filter(color => {
              return color.match(colorGroup);
            }) as color}
              <ColorSwatch {color} style="margin: 0 1rem 1rem 0;" />
            {/each}
          </div>
        {/each}
      </Section>

      <Section
        title="Typography"
        subTitle="Using Inter and Source Code Pro fonts">
        <TypographySwatch title="<h1>">
          <h1>Radicle Upstream</h1>
        </TypographySwatch>

        <TypographySwatch title="<h2>">
          <h2>Radicle Upstream</h2>
        </TypographySwatch>

        <TypographySwatch title="<h3>">
          <h3>Radicle Upstream</h3>
        </TypographySwatch>

        <TypographySwatch title={`<h3 class="typo-mono-bold">`}>
          <h3 class="typo-mono-bold">Radicle Upstream</h3>
        </TypographySwatch>

        <TypographySwatch title="<h4>">
          <h4>Radicle Upstream</h4>
        </TypographySwatch>

        <TypographySwatch title={`<p> or <p class="typo-text">`}>
          <p>Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-text-bold">`}>
          <p class="typo-text-bold">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-text-mono">`}>
          <p class="typo-text-mono">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-mono-bold">`}>
          <p class="typo-mono-bold">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-text-small">`}>
          <p class="typo-text-small">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-text-small-bold">`}>
          <p class="typo-text-small-bold">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-text-small-bold">`}>
          <p class="typo-text-small-bold">0123456789</p>
        </TypographySwatch>

        <TypographySwatch title={`<a class="typo-link" href="#relative-link">`}>
          <a class="typo-link" href="#relative-link">Relative link</a>
        </TypographySwatch>

        <TypographySwatch
          title={`<a class="typo-link" href="https://radicle.xyz">`}>
          <a class="typo-link" href="https://radicle.xyz">External link</a>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-all-caps">`}>
          <p class="typo-all-caps">Radicle Upstream</p>
        </TypographySwatch>

        <TypographySwatch title={`<p class="typo-enable-calt">`}>
          <p class="typo-enable-calt">100x20</p>
        </TypographySwatch>
      </Section>

      <Section title="Icons" subTitle="Icons at 24px width and height">
        <div class="icon-swatch">
          {#each Object.keys(Icon) as iconName}
            <Copyable
              name={`"<Icon.${iconName} />"`}
              clipboardContent={`<Icon.${iconName} />`}>
              <svelte:component this={getIconComponent(iconName)} />
            </Copyable>
          {/each}
        </div>

        <div class="swatch">
          <Spinner />
        </div>
      </Section>

      <Section
        title="Buttons"
        subTitle="Vanilla, Primary, Cancel, disabled state">
        <table>
          <thead>
            <tr>
              <td>
                <h5>Variant</h5>
              </td>
              <td>
                <h5>Disabled</h5>
              </td>
              <td>
                <h5>Variant</h5>
              </td>
              <td>
                <h5>Disabled</h5>
              </td>
            </tr>
          </thead>
          <tr>
            <td>
              <Button variant="primary">Primary</Button>
            </td>
            <td>
              <Button variant="primary" disabled>Primary</Button>
            </td>
            <td>
              <Button icon={Icon.Minus} variant="primary">Primary</Button>
            </td>
            <td>
              <Button icon={Icon.Minus} variant="primary" disabled>
                Primary
              </Button>
            </td>
          </tr>
          <tr>
            <td>
              <Button variant="vanilla">Vanilla</Button>
            </td>
            <td>
              <Button variant="vanilla" disabled>Vanilla</Button>
            </td>
            <td>
              <Button icon={Icon.Plus} variant="vanilla">Vanilla</Button>
            </td>
            <td>
              <Button icon={Icon.Plus} variant="vanilla" disabled
                >Vanilla</Button>
            </td>
          </tr>
          <tr>
            <td>
              <Button variant="outline">Outline</Button>
            </td>
            <td>
              <Button variant="outline" disabled>Outline</Button>
            </td>
            <td>
              <Button icon={Icon.Fork} variant="outline">Outline</Button>
            </td>
            <td>
              <Button icon={Icon.Fork} variant="outline" disabled
                >Outline</Button>
            </td>
          </tr>
          <tr>
            <td>
              <Button variant="transparent">Transparent</Button>
            </td>
            <td>
              <Button variant="transparent" disabled>Transparent</Button>
            </td>
            <td>
              <Button icon={Icon.Check} variant="transparent"
                >Transparent</Button>
            </td>
            <td>
              <Button icon={Icon.Check} variant="transparent" disabled>
                Transparent
              </Button>
            </td>
          </tr>
          <tr>
            <td>
              <Button variant="destructive">Destructive</Button>
            </td>
            <td>
              <Button variant="destructive" disabled>Destructive</Button>
            </td>
            <td>
              <Button icon={Icon.Cross} variant="destructive"
                >Destructive</Button>
            </td>
            <td>
              <Button icon={Icon.Cross} variant="destructive" disabled>
                Destructive
              </Button>
            </td>
          </tr>
        </table>
      </Section>

      <Section
        title="Form elements"
        subTitle="Inputs, text areas, dropdowns, etc.">
        <div class="swatch">
          <TextInput placeholder="Hey, I'm an input." />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Hey, I'm a full-width input."
            style="flex: 1" />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Hey, I'm a full-width input with a hint"
            hint="↵"
            style="flex: 1" />
        </div>

        <div class="swatch">
          <TextInput
            style="flex: 1;"
            disabled
            placeholder="Hey, I'm a disabled input with a placeholder." />
        </div>

        <div class="swatch">
          <TextInput
            style="flex: 1;"
            disabled
            value="I'm a disabled input with a value." />
        </div>

        <div class="swatch">
          <TextInput style="flex: 1;" value="I have a..." suffix="suffix" />
        </div>

        <div class="swatch">
          <TextInput
            style="flex: 1;"
            value="I have a slot on the left side"
            showLeftItem={true}>
            <svelte:fragment slot="left">
              <Icon.MagnifyingGlass />
            </svelte:fragment>
          </TextInput>
        </div>

        <div class="swatch">
          <TextInput
            placeholder="And I'm an input with a validation error."
            style="flex: 1"
            validation={{
              status: ValidationStatus.Error,
              message: "Well, that didn't go well...",
            }} />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Enter user name"
            style="width: 100%"
            showSuccessCheck
            validation={{ status: ValidationStatus.Success }} />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Enter user name"
            style="width: 100%"
            validation={{ status: ValidationStatus.Loading }}
            value="user123" />
        </div>

        <div class="swatch">
          <TextInput
            style="width: 100%;"
            concealed={true}
            placeholder="Please enter a password" />
        </div>

        <div class="swatch">
          <TextInput
            concealed={true}
            style="width: 100%;"
            value="my super long password" />
        </div>

        <div class="swatch">
          <TextInput
            concealed={true}
            style="width: 100%;"
            value="too short"
            validation={{
              status: ValidationStatus.Error,
              message: "Password too short.",
            }} />
        </div>

        <div class="swatch">
          <Checkbox>How about a checkbox?</Checkbox>
        </div>

        <div class="swatch">
          <Dropdown
            placeholder="Select option..."
            options={[
              { value: "1", title: "Option 1" },
              {
                value: "2",
                title: "Longer option keeps going",
              },
            ]} />
        </div>

        <div class="swatch">
          <Dropdown
            options={[{ value: "1", title: "Option 1" }]}
            placeholder="Select option..."
            disabled={true} />
        </div>
      </Section>

      <h1 style="margin-bottom: 92px">Components</h1>

      <Section
        title="Avatars"
        subTitle="User and Org avatars in various sizes.">
        <div class="swatch">
          <Avatar
            style="margin-right: 16px"
            size="small"
            kind={{
              type: "userImage",
              url: "https://avatars.githubusercontent.com/u/4406983",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="small"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: "rad:git:hnrk8cgbe4mgkubojmkdt6enka84ryfkdxhcy",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="small"
            kind={{
              type: "orgImage",
              url: "https://app.radicle.network/images/alt-clients.png",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="small"
            kind={{
              type: "orgEmoji",
              uniqueIdentifier: "0x8152237402E0f194176154c3a6eA1eB99b611482",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="small"
            kind={{ type: "pendingOrg" }} />
        </div>

        <div class="swatch">
          <Avatar
            style="margin-right: 16px"
            size="regular"
            kind={{
              type: "userImage",
              url: "https://avatars.githubusercontent.com/u/4406983",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="regular"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: "rad:git:hnrk8cgbe4mgkubojmkdt6enka84ryfkdxhcy",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="regular"
            kind={{
              type: "orgImage",
              url: "https://app.radicle.network/images/alt-clients.png",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="regular"
            kind={{
              type: "orgEmoji",
              uniqueIdentifier: "0x8152237402E0f194176154c3a6eA1eB99b611482",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="regular"
            kind={{ type: "pendingOrg" }} />
        </div>

        <div class="swatch">
          <Avatar
            style="margin-right: 16px"
            size="large"
            kind={{
              type: "userImage",
              url: "https://avatars.githubusercontent.com/u/4406983",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="large"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: "rad:git:hnrk8cgbe4mgkubojmkdt6enka84ryfkdxhcy",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="large"
            kind={{
              type: "orgImage",
              url: "https://app.radicle.network/images/alt-clients.png",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="large"
            kind={{
              type: "orgEmoji",
              uniqueIdentifier: "0x8152237402E0f194176154c3a6eA1eB99b611482",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="large"
            kind={{ type: "pendingOrg" }} />
        </div>

        <div class="swatch">
          <Avatar
            style="margin-right: 16px"
            size="huge"
            kind={{
              type: "userImage",
              url: "https://avatars.githubusercontent.com/u/4406983",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="huge"
            kind={{
              type: "userEmoji",
              uniqueIdentifier: "rad:git:hnrk8cgbe4mgkubojmkdt6enka84ryfkdxhcy",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="huge"
            kind={{
              type: "orgImage",
              url: "https://app.radicle.network/images/alt-clients.png",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="huge"
            kind={{
              type: "orgEmoji",
              uniqueIdentifier: "0x8152237402E0f194176154c3a6eA1eB99b611482",
            }} />
          <Avatar
            style="margin-right: 16px"
            size="huge"
            kind={{ type: "pendingOrg" }} />
        </div>
      </Section>

      <Section title="Notifications" subTitle="Info, Warnings and Errors">
        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Info, {
              message: "Snackbar",
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Info, {
              message: "Info with icon",
              showIcon: true,
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Info, {
              message: "Info without default action",
              actions: [],
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Error, {
              message: "Just plain error",
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Error, {
              message: "Error with icon",
              showIcon: true,
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Error, {
              message: "Error with one action",
              actions: [{ label: "Action", handler: () => {} }],
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Error, {
              message: "Error with two actions",
              actions: [
                { label: "Action 1", handler: () => {} },
                { label: "Action 2", handler: () => {} },
              ],
            })} />
        </div>

        <div class="swatch">
          <Notification
            notification={notification.create(notification.Variant.Primary, {
              message: "Primary notification",
              actions: [
                { label: "Action 1", handler: () => {} },
                { label: "Action 2", handler: () => {} },
              ],
            })} />
        </div>
      </Section>

      <Section title="Tooltips" subTitle="Top, Right, Bottom, Left">
        <div class="swatch">
          <Tooltip value="Top" position="top">
            <Button variant="outline">Hover me!</Button>
          </Tooltip>
        </div>

        <div class="swatch">
          <Tooltip value="Right" position="right">
            <Button variant="outline">Hover me!</Button>
          </Tooltip>
        </div>

        <div class="swatch">
          <Tooltip value="Bottom" position="bottom">
            <Button variant="outline">Hover me!</Button>
          </Tooltip>
        </div>

        <div class="swatch">
          <Tooltip value="Left" position="left">
            <Button variant="outline">Hover me!</Button>
          </Tooltip>
        </div>
      </Section>

      <Section
        title="Copyable identifiers"
        subTitle="Various Radicle and Ethereum identifiers that can be copied to clipboard">
        <div class="swatch">
          <CopyableIdentifier
            kind="radicleId"
            value="rad:git:hwd1yre8ttugonm77udfkti4ou89p4e37gdebmj3o544hzrg3r8dupn8hmr"
            showIcon={false} />
        </div>

        <div class="swatch">
          <CopyableIdentifier
            style="margin-right: 1rem;"
            kind="radicleId"
            value="rad:git:hwd1yre8ttugonm77udfkti4ou89p4e37gdebmj3o544hzrg3r8dupn8hmr" />

          <CopyableIdentifier
            style="margin-right: 1rem;"
            kind="deviceId"
            value="hyyo6u8rhnuswory4c6symx471yseke74oq1myfesoig7zggcixejy" />

          <CopyableIdentifier
            style="margin-right: 1rem;"
            kind="ethAddress"
            value="0xA66A5686D5c3A42C0b6c76FEd05e58C6bc851E9f" />
        </div>

        <div class="swatch">
          <CopyableIdentifier
            kind="seedAddress"
            value="hybz9gfgtd9d4pd14a6r66j5hz6f77fed4jdu7pana4fxaxbt369kg@setzling.radicle.xyz:12345" />
        </div>

        <div class="swatch">
          <CopyableIdentifier
            kind="commitHash"
            value="20436154e1118b39f1b2bf3c049ab040ca910846" />
        </div>
      </Section>

      <Section
        title="Identifier links"
        subTitle="Various Radicle and Ethereum identifiers that link within Upstream or to external resources">
        <div class="swatch">
          <IdentifierLink
            params={{
              type: "transactionHash",
              url: "https://rinkeby.etherscan.io/tx/0xcf23b34a9f09245226c19114af534ee094b028922a1280003226fd98acb410ea",
              hash: "0xcf23b34a9f09245226c19114af534ee094b028922a1280003226fd98acb410ea",
            }} />
        </div>
        <div class="swatch">
          <IdentifierLink
            params={{
              type: "commitHash",
              hash: "20436154e1118b39f1b2bf3c049ab040ca910846",
              onClick: () => {},
            }} />
        </div>
      </Section>

      <Section title="Copyable" subTitle="Copy to clipboard">
        <div class="swatch">
          <Copyable name="command" clipboardContent="git push rad">
            <pre
              style="typo-text-mono">
              git push rad
            </pre>
          </Copyable>
        </div>

        <div class="swatch">
          <Copyable
            name="WalletConnect link"
            clipboardContent="wc:ca1716bb-aeb0-4420-aee9-8ee15e577740@1?bridge=https%3A%2F%2Fe.bridge.walletconnect.org&key=199be30aedc8fff6e19019ea3a7a9a7f998a499b7f50ba89f06bf918fc2dab39">
            wc:ca171…dab39
          </Copyable>
        </div>

        <div class="swatch">
          <Copyable>when no props are supplied</Copyable>
        </div>
      </Section>

      <Section title="Misc" subTitle="Everything else">
        <div class="swatch">
          <ThreeDotsMenu
            style="margin-right: 1rem;"
            menuItems={[
              {
                title: "Add something",
                icon: Icon.Plus,
                event: () => {},
                tooltip: "Here be tooltip",
              },
              {
                title: "Send something",
                icon: Icon.ArrowUp,
                event: () => {},
                disabled: true,
              },
            ]} />

          <SupportButton style="margin-right: 1rem;" />

          <SegmentedControl
            active={1}
            options={[
              {
                title: "Open",
                value: 0,
              },
              {
                title: "Closed",
                value: 1,
              },
              {
                title: "All",
                value: 2,
              },
            ]} />
        </div>

        <div class="swatch">
          <FollowToggle disabled style="margin-right: 1rem;" />
          <FollowToggle disabled style="margin-right: 1rem;" following />
          <FollowToggle style="margin-right: 1rem;" />
          <FollowToggle following />
        </div>
      </Section>
    </div>
  </div>
</div>
