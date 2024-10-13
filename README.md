# Moss

- [Contributing](#contributing)
- [Requirements](#requirements)
- [Usage](#usage)
- [Project Folder Structure](#project-folder-structure)
- [Key Notes](#key-notes)

## Contributing

We would for you to get involved with Moss development! If you wish to help, you can learn more about how you can contribute to this project in the [contribution guide](CONTRIBUTING.md).

# Requirements

Before starting the project, ensure you have [NIX](https://nixos.org/download/) installed and enable the [flakes](https://nixos.wiki/wiki/Flakes) experimental feature.

To do this, add the following line to your Nix configuration file:

- For user-specific settings, edit `~/.config/nix/nix.conf`:

- For system-wide settings, edit `/etc/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

## Installing Dependencies

To install the necessary dependencies for the project, run the following command:

```bash
nix develop
```

This command will set up a development environment with all the required tools and libraries specified in the `flake.nix` file.

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
