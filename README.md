# Moss

### Join Our Team 🚀

We're looking for talented developers with skills in either **TypeScript**, **React**, or **Rust** to help us build _Moss Studio_.

#### ⚠️ Interested? 👉 g10z3r@duck.com

- [Contributing](#contributing)
- [Requirements](#requirements)
- [Usage](#usage)
- [Project Folder Structure](#project-folder-structure)
- [Key Notes](#key-notes)

## Contributing

We would for you to get involved with Moss development! If you wish to help, you can learn more about how you can contribute to this project in the [contribution guide](CONTRIBUTING.md).

# Requirements

Before starting the project, ensure you have the following installed:

- [SurrealDB](https://surrealdb.com/)
- [Tauri](https://tauri.app/)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/)

On the first app start, Tauri may prompt you to install additional dependencies. These dependencies are described in the [Tauri Getting Started Guide](https://tauri.app/v1/guides/getting-started/prerequisites).

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

Now you should be free to go!

<!-- ## Nix usage (not ready to be used!)

Before starting the project, ensure you have [NIX](https://nixos.org/download/) installed and enable the [flakes](https://nixos.wiki/wiki/Flakes) experimental feature.

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

This command will set up a development environment with all the required tools and libraries specified in the `flake.nix` file.

**Note**: You will need to run `nix develop` in every new terminal session before starting development to make the tools available in that shell. This is because the environment is only active within the current shell session and does not persist across multiple terminal sessions. -->

# Usage

- Before running any applications, ensure that SurrealDB is started:

```sh
make run-database
```

- Run all apps:

```sh
pnpm turbo dev
```

- Run desktop app:

```sh
make run-desktop
```

- Run desktop web app:

```sh
make run-desktop-web
```

- Start SurrealDB:

```sh
make run-database
```

- Run storybook app:

```sh
make run-storybook
```

- Run web app:

```sh
make run-web
```

- Run docs app:

```sh
make run-docs
```

- Generate monorepo project dependency graph:

```sh
pnpm turbo run build --graph
```

### Key Notes:

- **Desktop App**: The desktop application is developed using [TypeScript](https://www.typescriptlang.org/) and [Tauri](https://tauri.app/), which allows for building cross-platform applications with [Rust](https://www.rust-lang.org/) and Web technologies.
