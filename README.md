# Moss

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
```

# Usage

- Before running any applications, ensure that SurrealDB is started:

```sh
cd ./view/desktop/ && surreal start file:rocksdb
```

- Run all apps:

```sh
pnpm turbo dev
```

- Run desktop app:

```sh
pnpm desktop
```

- Run desktop web app:

```sh
pnpm desktop-web
```

- Start SurrealDB:

```sh
pnpm sd
```

- Run storybook app:

```sh
pnpm sb
```

- Run web app:

```sh
pnpm web
```

- Run docs app:

```sh
pnpm doc
```

- Generate monorepo project dependency graph:

```sh
pnpm turbo run build --graph
```

### Key Notes:

- **Desktop App**: The desktop application is developed using [TypeScript](https://www.typescriptlang.org/) and [Tauri](https://tauri.app/), which allows for building cross-platform applications with [Rust](https://www.rust-lang.org/) and Web technologies.
