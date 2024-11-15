# Moss

Cross-platform code analyzer.

# Technologies

- **Tauri** (framework for building desktop applications using JS, HTML, and CSS)
- **Vite** (framework for building fast, efficient, and scalable web apps)
- **React** (framework for creating interactive UI components)
- **TypeScript** (type-safety support)
- **Tailwind CSS** (CSS styling)
- **Redux** (React state manager)
- **i18next** (internationalization)
- **pNPM** (package manager)
- **Storybook** (workshop for building, testing, documenting and sharing UI components)
- **Turborepo** (smart monorepo build system / monorepo orchestrator) solves monorepos' scaling problem. Its remote cache stores the result of all your tasks, meaning that your CI never needs to do the same work twice. Visualize dependency graphs. Remote Vercel caching.
- **husky** Automatically lint commit messages, code, and run tests upon committing or pushing.

# Architecture

A monorepo is a single repository containing multiple distinct projects, with well-defined relationships.

## Monorepo structure:

### Apps:

- `desktop`: A [Tauri](https://tauri.app) app.
- `web`: A [React](https://reactjs.org) webapp.
- `landing`: A [React](https://reactjs.org) app using [Next.js](https://nextjs.org).
- `cli`: A [Rust](https://www.rust-lang.org) command line interface. (planned)
- `storybook`: A [React](https://reactjs.org) storybook for the UI components.

### Packages:

- `assets`: Shared assets (images, fonts, etc).
- `client`: A [TypeScript](https://www.typescriptlang.org/) client library to handle dataflow via RPC between UI and the Rust core.
- `config`: `eslint` configurations (includes `eslint-config-next`, `eslint-config-prettier` and all `tsconfig.json` configs used throughout the monorepo).
- `ui`: A [React](https://reactjs.org) Shared component library.

### Icons

To add new icon add **`.svg`** file to `tools/icongen/assets` directory and than run `pnpm build` inside `tools/icongen`
