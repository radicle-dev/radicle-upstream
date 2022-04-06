<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { clean, satisfies } from "semver";

  import * as ipc from "ui/src/ipc";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";
  import * as session from "ui/src/session";

  import RadicleLogo from "design-system/RadicleLogo.svelte";
  import CheckItem from "./OnboardingModal/CheckItem.svelte";
  import CodeBlock from "./OnboardingModal/CodeBlock.svelte";

  enum Step {
    installRadCli,
    createRadIdentity,
    addUpstreamCliToPath,
    setUpGit,
  }

  type DependencyCheckResult =
    | {
        step: Step.createRadIdentity;
        passed: true;
        name: string;
      }
    | {
        step: Step;
        passed: boolean;
        version?: string;
      };

  let activeStep: Step = Step.installRadCli;
  let doneSteps: Step[] = [];

  let radCliVersion: string;
  let identityName: string;
  let gitVersion: string;

  const stepSequence: Step[] = [
    Step.installRadCli,
    Step.createRadIdentity,
    Step.addUpstreamCliToPath,
    Step.setUpGit,
  ];

  async function markStepAsDone(stepName: Step) {
    doneSteps = [...doneSteps, stepName];

    while (doneSteps.includes(activeStep)) {
      // If we're at the end of the sequence, update the proxy session status
      // and push the user to their profile.
      if (activeStep === stepSequence[stepSequence.length - 1]) {
        await session.fetch();
        router.replace({ type: "profile" });
      }

      activeStep = stepSequence[stepSequence.indexOf(activeStep) + 1];
    }
  }

  const check = async (forStep: Step): Promise<DependencyCheckResult> => {
    switch (forStep) {
      case Step.installRadCli: {
        const radCliCheck = await ipc.checkRadCliVersion();

        if (radCliCheck) {
          const verString = clean(radCliCheck.replace("rad", ""));

          if (!verString) {
            return { step: forStep, passed: false };
          }

          radCliVersion = verString;

          return { step: forStep, passed: true, version: verString };
        }

        break;
      }
      case Step.createRadIdentity: {
        try {
          const proxySession = await proxy.client.sessionGet();

          if (proxySession.identity) {
            identityName = proxySession.identity.metadata.handle;

            return {
              step: Step.createRadIdentity,
              passed: true,
              name: proxySession.identity.metadata.handle,
            };
          }

          break;
        } catch {
          return { step: forStep, passed: false };
        }
      }
      case Step.setUpGit: {
        const gitCheck = await ipc.checkGitVersion();

        if (gitCheck) {
          const verString = clean(gitCheck.replace("git version", ""));

          if (!verString) {
            return { step: forStep, passed: false };
          }

          gitVersion = verString;

          if (satisfies(verString, ">=2.35.1")) {
            return { step: forStep, passed: true, version: verString };
          }
        }

        break;
      }
    }

    return { step: forStep, passed: false };
  };

  const performCheck = async (forStep: Step) => {
    const result = await check(forStep);

    if (result.passed) {
      markStepAsDone(forStep);
    }
  };

  let checkInterval: number;

  onMount(async () => {
    // Run all dependency checks on startup once to make sure already satisfied
    // requirements are displayed as such immediately.
    for (const step of stepSequence) {
      await performCheck(step);
    }

    // While a given step is active, re-run the step's dependency check (if any)
    // every second.
    checkInterval = window.setInterval(() => {
      performCheck(activeStep);
    }, 1000);
  });

  onDestroy(() => {
    clearInterval(checkInterval);
  });
</script>

<style>
  .wrapper {
    width: 100vw;
  }
  .container {
    width: 100vw;
    padding: 128px;
    max-width: 1024px;
    margin: 0 auto;
  }

  .welcome-text {
    margin: 96px 0;
  }

  .welcome-text > h1 {
    margin-bottom: 16px;
  }

  h1,
  p {
    max-width: 560px;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .check-item-container {
    margin: 0 -16px;
  }

  .step-content > *:not(:last-child) {
    margin-bottom: 24px;
  }

  .step-content > *:not(:first-child) {
    margin-top: 24px;
  }
</style>

<div class="wrapper">
  <div class="container">
    <RadicleLogo />
    <div class="welcome-text">
      <h1>Welcome to Radicle Upstream</h1>
      <p>
        Radicle is a free and open-source way to host, share, and build software
        together. To get started, we just need to complete a few simple steps.
      </p>
    </div>
    <div class="check-item-container" data-cy="onboarding-checklist">
      <CheckItem
        expanded={activeStep === Step.installRadCli}
        title="Install the Radicle CLI"
        onSkip={() => markStepAsDone(Step.installRadCli)}
        done={doneSteps.includes(Step.installRadCli)}
        waitingFor="the Radicle CLI to be installed"
        badge={radCliVersion && `Version ${radCliVersion} installed`}>
        <div class="step-content" slot="content">
          <p>
            First, let's install the Radicle CLI. You'll use the CLI to create
            and publish projects to the Radicle network, or clone existing ones
            to your machine.
          </p>
          <p>
            To get started, ensure you have <a
              class="typo-link"
              href="https://doc.rust-lang.org/cargo/getting-started/installation.html"
              >Cargo</a>
            and <a class="typo-link" href="https://cmake.org/install/">CMake</a>
            installed, then run:
          </p>
          <CodeBlock
            command="cargo install --force --locked --git https://seed.alt-clients.radicle.xyz/radicle-cli.git radicle-cli" />
          <p>
            On devices that run macOS on x86_64 hardware, you can alternatively
            use Homebrew to install the CLI:
          </p>
          <CodeBlock
            command="brew tap radicle/cli https://seed.alt-clients.radicle.xyz/radicle-cli-homebrew.git && brew install radicle/cli/core" />
        </div>
      </CheckItem>
      <CheckItem
        done={doneSteps.includes(Step.createRadIdentity)}
        expanded={activeStep === Step.createRadIdentity}
        title="Create your Radicle identity"
        waitingFor="Radicle identity to be created"
        badge={identityName && `Hello, ${identityName} 👋`}>
        <div class="step-content" slot="content">
          <p>
            To interact with the Radicle network, you need an identity. Create
            one by running the following command:
          </p>
          <CodeBlock command="rad auth">
            <span slot="output">
              Initializing your 🌱 profile and identity<br />
              <br />
              <span style="color: var(--color-positive)">ok</span> Username ·
              koops<br />
              <span style="color: var(--color-positive)">ok</span> Passphrase ·
              ********<br />
              <span style="color: var(--color-positive)">ok</span> Creating your
              🌱 Ed25519 keypair...<br />
              <span style="color: var(--color-positive)">ok</span> Adding to
              ssh-agent...<br />
              <span style="color: var(--color-positive)">ok</span> Profile
              3ae66df3-6ac7-4466-9013-83839749ed05 created.<br />
              <br />
              Your radicle Peer ID is
              <span style="color: var(--color-positive)"
                >hyncoz7x4s8x9447g6yogy4iy41q8i4juy5uhou57w1ga7obt644wo</span
              >. This identifies your device.<br />
              Your personal 🌱 URN is
              <span style="color: var(--color-positive)"
                >rad:git:hnrkmx6trm4bu19bwa4apbxj8ftw8f7amfdyy</span
              >. This identifies you across devices.<br />
              <br />
              <span style="color: var(--color-primary)"
                >=> To create a radicle project, run `rad init` from a git
                repository.</span
              ><br />
            </span>
          </CodeBlock>
        </div>
      </CheckItem>
      <CheckItem
        done={doneSteps.includes(Step.addUpstreamCliToPath)}
        expanded={activeStep === Step.addUpstreamCliToPath}
        onSkip={() => markStepAsDone(Step.addUpstreamCliToPath)}
        title="Add the Upstream CLI to your shell PATH">
        <div class="step-content" slot="content">
          <p>
            Upstream installs additional commands needed for code collaboration
            in a custom location. Please make sure that these binaries are in
            your shell PATH by adding the following line to your shell
            configuration:
          </p>
          <CodeBlock command="export PATH=&quot;$HOME/.radicle/bin:$PATH" />
        </div>
      </CheckItem>
      <CheckItem
        done={doneSteps.includes(Step.setUpGit)}
        expanded={activeStep === Step.setUpGit}
        onSkip={() => markStepAsDone(Step.setUpGit)}
        title="Set up Git"
        badge={gitVersion && `Version ${gitVersion} installed`}>
        <div class="step-content" slot="content">
          <p>
            Radicle is built on Git. In order to collaborate with others, you'll
            need at least Git version 2.35.1 installed.
          </p>
          <p>
            You can download the latest version from the <a
              href="https://git-scm.com/download/"
              class="typo-link">official website</a
            >.
          </p>
        </div>
      </CheckItem>
    </div>
  </div>
</div>
