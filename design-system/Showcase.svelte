<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import CheckIcon from "./icons/Check.svelte";
  import CrossIcon from "./icons/Cross.svelte";
  import ForkIcon from "./icons/Fork.svelte";
  import MinusIcon from "./icons/Minus.svelte";
  import PlusIcon from "./icons/Plus.svelte";
  import ArrowUpIcon from "./icons/ArrowUp.svelte";

  import Avatar from "./Avatar.svelte";
  import Button from "./Button.svelte";
  import Checkbox from "./Checkbox.svelte";
  import Dropdown from "./Dropdown.svelte";
  import TrackToggle from "./TrackToggle.svelte";
  import IdentifierLink from "./IdentifierLink.svelte";
  import Loading from "./Loading.svelte";
  import SegmentedControl from "./SegmentedControl.svelte";
  import SupportButton from "./SupportButton.svelte";
  import TextInput from "./TextInput.svelte";
  import ThreeDotsMenu from "./ThreeDotsMenu.svelte";
  import Tooltip from "./Tooltip.svelte";

  import ColorSwatch from "./Showcase/ColorSwatch.svelte";
  import ElevationSwatch from "./Showcase/ElevationSwatch.svelte";
  import Section from "./Showcase/Section.svelte";
  import TypographySwatch from "./Showcase/TypographySwatch.svelte";

  export let onClose: (() => void) | undefined = undefined;

  function extractCssVariables(variableName: string) {
    return Array.from(document.styleSheets)
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
                          name.startsWith(variableName)
                        ),
                      ]
                    : def),
              []
            ),
          ]),
        []
      );
  }

  const colors = extractCssVariables("--color");
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

  const elevations = extractCssVariables("--elevation");

  function onKeydown(event: KeyboardEvent) {
    if (
      event.target === document.body &&
      event.code === "Escape" &&
      onClose !== undefined
    ) {
      onClose();
    }
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
</style>

<svelte:window on:keydown={onKeydown} />

{#if onClose !== undefined}
  <div class="close-button">
    <Button style="padding:0.5rem;" on:click={onClose} variant="transparent">
      <CrossIcon />
    </Button>
  </div>
{/if}

<div class="fullscreen">
  <div class="content">
    <div class="layout">
      <slot name="top" />

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
        title="Elevations"
        subTitle="Three levels of elevation"
        contentStyle="display: flex; gap: 3rem;">
        {#each elevations as elevation}
          <ElevationSwatch {elevation} />
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
              <Button icon={MinusIcon} variant="primary">Primary</Button>
            </td>
            <td>
              <Button icon={MinusIcon} variant="primary" disabled>
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
              <Button icon={PlusIcon} variant="vanilla">Vanilla</Button>
            </td>
            <td>
              <Button icon={PlusIcon} variant="vanilla" disabled
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
              <Button icon={ForkIcon} variant="outline">Outline</Button>
            </td>
            <td>
              <Button icon={ForkIcon} variant="outline" disabled
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
              <Button icon={CheckIcon} variant="transparent"
                >Transparent</Button>
            </td>
            <td>
              <Button icon={CheckIcon} variant="transparent" disabled>
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
              <Button icon={CrossIcon} variant="destructive"
                >Destructive</Button>
            </td>
            <td>
              <Button icon={CrossIcon} variant="destructive" disabled>
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
            placeholder="I'm an input with a validation error."
            style="flex: 1"
            validationState={{
              type: "invalid",
              message: "That doesn't look good!",
            }} />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Enter user name"
            style="width: 100%"
            showSuccessCheck
            validationState={{ type: "valid" }} />
        </div>

        <div class="swatch">
          <TextInput
            placeholder="Enter user name"
            style="width: 100%"
            validationState={{ type: "pending" }}
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
            validationState={{
              type: "invalid",
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
              url: "https://pbs.twimg.com/profile_images/1372563232850870274/aREQff_C_400x400.jpg",
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
              url: "https://pbs.twimg.com/profile_images/1372563232850870274/aREQff_C_400x400.jpg",
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
              url: "https://pbs.twimg.com/profile_images/1372563232850870274/aREQff_C_400x400.jpg",
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
              url: "https://pbs.twimg.com/profile_images/1372563232850870274/aREQff_C_400x400.jpg",
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

      <Section title="Misc" subTitle="Everything else">
        <div class="swatch">
          <ThreeDotsMenu
            style="margin-right: 1rem;"
            menuItems={[
              {
                title: "Add something",
                icon: PlusIcon,
                event: () => {},
                tooltip: "Here be tooltip",
              },
              {
                title: "Send something",
                icon: ArrowUpIcon,
                event: () => {},
                disabled: true,
              },
            ]} />

          <SupportButton style="margin-right: 1rem;" />

          <SegmentedControl
            active={"closed"}
            options={[
              {
                title: "Open",
                value: "open",
              },
              {
                title: "Closed",
                value: "closed",
              },
              {
                title: "All",
                value: "all",
              },
            ]} />
        </div>

        <div class="swatch">
          <TrackToggle disabled style="margin-right: 1rem;" />
          <TrackToggle disabled style="margin-right: 1rem;" tracking />
          <TrackToggle style="margin-right: 1rem;" />
          <TrackToggle tracking />
        </div>

        <div class="swatch">
          <Loading />
        </div>
      </Section>

      <slot name="bottom" />
    </div>
  </div>
</div>
