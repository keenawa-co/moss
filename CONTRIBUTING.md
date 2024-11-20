# Contributing to Moss

We would love for you to contribute to Moss and help make it even better than it is today! Whether you're fixing bugs,
proposing new features, or enhancing documentation, your contributions are greatly appreciated.

## Table of Contents

- [Getting Started](#getting-started)
  - [Forking the Repository](#forking-the-repository)
  - [Cloning the Repository](#cloning-the-repository)
  - [Setting Up Your Environment](#setting-up-your-environment)
- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Proposing Features](#proposing-features)
- [Commit Message Guidelines](#commit-message-guidelines)
  - [Semantic Release Format](#semantic-release-format)
  - [Example Commit Messages](#example-commit-messages)
- [Submitting a pull request](#submitting-a-pull-request)
- [Commit your changes](#commit-your-changes)
- [Create a pull request](#create-a-pull-request)
- [Code Review Process](#code-review-process)

## Getting Started

Before starting the project, ensure you have the following installed:

- [SurrealDB](https://surrealdb.com/)
- [Tauri](https://tauri.app/)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/)

### Forking the Repository

1. Navigate to the Moss repository on GitHub, <https://github.com/keenawa-co/moss>
2. Click the "Fork" button at the top right of the page, or click here, <https://github.com/keenawa-co/moss/fork>

### Cloning the Repository

1. Open your terminal.
2. Clone your forked repository:
   ```bash
   git clone https://github.com/<your-username>/moss.git
   ```

### Setting Up Your Environment

1. Navigate to the project directory: `cd moss`
2. Install the necessary dependencies: `cargo install`
3. Make sure the project requirements are met. Follow the [requirements](README.md#requirements) section in the project
   Readme.

**Note for Ubuntu Linux users:**

Based on personal experience, some additional libraries may be required:

```sh
sudo apt install libwebkit2gtk-4.1-dev
sudo apt install libjavascriptcoregtk-4.1-dev
sudo apt install libsoup-3.0-dev
sudo apt install clang
```

**Note for Windows users:**

You need to first install GNU make before running the `make` scripts:

1. Install Chocolatey from https://chocolatey.org/install

2. In an administrative shell, run `choco install make`

<!-- ## Nix usage (not ready to be used!)

Before starting the project, ensure you have [NIX](https://nixos.org/download/) installed and enable
the [flakes](https://nixos.wiki/wiki/Flakes) experimental feature.

To do this, add the following line to your Nix configuration file:

- For user-specific settings, edit `~/.config/nix/nix.conf`:

- For system-wide settings, edit `/etc/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

### Installing Dependencies

To install the necessary dependencies for the project, run the following command:

```bash
nix develop
```

This command will set up a development environment with all the required tools and libraries specified in the
`flake.nix` file.

**Note**: You will need to run `nix develop` in every new terminal session before starting development to make the tools
available in that shell. This is because the environment is only active within the current shell session and does not
persist across multiple terminal sessions. -->

## Code of conduct

Help us keep Moss open and inclusive. Please read and follow our [Code of Conduct](/CODE_OF_CONDUCT.md).

## How to Contribute

### Reporting Bugs

1. Ensure the bug has not already been reported by searching the issues.
2. Open a new issue with the title "Bug: [Descriptive Title]" and provide a detailed description.

### Proposing Features

1. Search for existing feature requests to avoid duplicates.
2. Open a new issue with the title "Feature Request: [Descriptive Title]" and describe the proposed feature.

### Introducing new features

We would Love your contribution to Moss, but we would also like to make sure Moss is as great as possible. Before
introducing a new pull request,

1. Create a new branch on your fork for your new feature: `git checkout -b feature/my-new-feature`
2. Make your changes and commit them following the commit message guidelines.

## Commit Message Guidelines

We use [semantic-release](https://github.com/semantic-release/semantic-release) to automate the versioning and release
process. Your commit messages should follow
the [Conventional Commits](https://semantic-release.gitbook.io/semantic-release) specification.

### Semantic Release Format

**feat**: A new feature
**fix**: A bug fix
**docs**: Documentation only changes
**perf**: A code change that improves performance
**style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
**refactor**: A code change that neither fixes a bug nor adds a feature
**test**: Adding missing or correcting existing tests
**build**: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
**ci**: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
**chore**: Other changes that don't modify src or test files
**revert**: Reverts a previous commit

### Example Commit Messages

- **feat:** add user authentication module
- **fix:** resolve issue with data fetching in dashboard
- **docs:** update contributing guidelines
- **style:** format code according to eslint rules
- **refactor:** simplify data processing logic
- **perf:** improve query performance
- **test:** add tests for user service
- **build:** update npm dependencies
- **ci:** configure Travis CI for automated testing
- **chore:** clean up old configuration files
- **revert:** revert "feat: add user authentication module"

## Submitting a pull request

The branch name is your first opportunity to give your task context. Branch naming convention is as follows,

`<TYPE>-<ISSUE_ID>-<DESCRIPTION>`

It is recommended to combine the relevant [GitHub Issue](https://github.com/keenawa-co/moss/issues) with a short
description that describes the task resolved in this branch. If you don't have GitHub issue for your PR, then you may
avoid the prefix, but keep in mind that more likely you have to create the issue first. For example:

```
feat-123-add-user-authentication-module
```

Where `TYPE` can be any of the types discussed in the [Semantic Release Format](#semantic-release-format)

## Commit your changes

- **Write a descriptive summary:** The first line of your commit message should be a concise summary of the changes you
  are making. It should be no more than 50 characters and should describe the change in a way that is easy to
  understand.

- **Provide more details in the body:** The body of the commit message should provide more details about the changes you
  are making. Explain the problem you are solving, the changes you are making, and the reasoning behind those changes.

- **Use the commit history in your favour:** Small and self-contained commits allow the reviewer to see exactly how you
  solved the problem. By reading the commit history of the PR, the reviewer can already understand what they'll be
  reviewing, even before seeing a single line of code.

## Create a pull request

- The **title** of your pull request should be clear and descriptive. It should summarize the changes you are making in
  a concise manner.

- Provide a detailed **description** of the changes you are making. Explain the reasoning behind the changes, the
  problem it solves, and the impact it may have on the codebase. Keep in mind that a reviewer was not working on your
  task, so you should explain why you wrote the code the way you did.

- Describe the scene and provide everything that will help to understand the background and a context for the reviewers
  by adding related [GitHub Issue](https://github.com/keenawa-co/moss/issues) to the description, and links to the
  related PRs, projects or third-party documentation. If there are any potential drawbacks or trade-offs to your
  changes, be sure to mention them too.

- Be sure to request reviews from the appropriate people. This might include the project maintainers, other
  contributors, or anyone else who is familiar with the codebase and can provide valuable feedback.

## Code Review Process

1. Submit a pull request to the `main` branch.
2. Ensure that all checks pass.
3. A maintainer will review your pull request. Feedback will be provided for any necessary changes.
4. Once approved, your pull request will be merged.
