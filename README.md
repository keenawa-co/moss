# Moss

- [Contributing](#contributing)

## Contributing

We would for you to get involved with Moss development! If you wish to help, you can learn more about how you can contribute to this project in the [contribution guide](CONTRIBUTING.md).


# Usage

- Generate monorepo project dependency graph:
```sh
pnpm turbo run build --graph
```
Gives:
```sh
digraph {
        compound = "true"
        newrank = "true"
        subgraph "root" {
                "[root] @repo/eslint-config#build" -> "[root] ___ROOT___"
                "[root] @repo/tailwind-config#build" -> "[root] @repo/typescript-config#build"
                "[root] @repo/typescript-config#build" -> "[root] ___ROOT___"
                "[root] @repo/ui#build" -> "[root] @repo/eslint-config#build"
                "[root] @repo/ui#build" -> "[root] @repo/tailwind-config#build"
                "[root] @repo/ui#build" -> "[root] @repo/typescript-config#build"
                "[root] docs#build" -> "[root] @repo/eslint-config#build"
                "[root] docs#build" -> "[root] @repo/tailwind-config#build"
                "[root] docs#build" -> "[root] @repo/typescript-config#build"
                "[root] docs#build" -> "[root] @repo/ui#build"
                "[root] web#build" -> "[root] @repo/eslint-config#build"
                "[root] web#build" -> "[root] @repo/tailwind-config#build"
                "[root] web#build" -> "[root] @repo/typescript-config#build"
                "[root] web#build" -> "[root] @repo/ui#build"
        }
}
```
- Run all apps:
```sh
pnpm turbo dev
```








## Turborepo Tailwind CSS starter

This is an official starter Turborepo.

## Using this example

Run the following command:

```sh
npx create-turbo@latest -e with-tailwind
```

## What's inside?

This Turborepo includes the following packages/apps:

### Apps and Packages

- `docs`: a [Next.js](https://nextjs.org/) app with [Tailwind CSS](https://tailwindcss.com/)
- `web`: another [Next.js](https://nextjs.org/) app with [Tailwind CSS](https://tailwindcss.com/)
- `ui`: a stub React component library with [Tailwind CSS](https://tailwindcss.com/) shared by both `web` and `docs` applications
- `@repo/eslint-config`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `@repo/typescript-config`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Building packages/ui

This example is set up to produce compiled styles for `ui` components into the `dist` directory. The component `.tsx` files are consumed by the Next.js apps directly using `transpilePackages` in `next.config.js`. This was chosen for several reasons:

- Make sharing one `tailwind.config.js` to apps and packages as easy as possible.
- Make package compilation simple by only depending on the Next.js Compiler and `tailwindcss`.
- Ensure Tailwind classes do not overwrite each other. The `ui` package uses a `ui-` prefix for it's classes.
- Maintain clear package export boundaries.

Another option is to consume `packages/ui` directly from source without building. If using this option, you will need to update the `tailwind.config.js` in your apps to be aware of your package locations, so it can find all usages of the `tailwindcss` class names for CSS compilation.

For example, in [tailwind.config.js](packages/tailwind-config/tailwind.config.js):

```js
  content: [
    // app content
    `src/**/*.{js,ts,jsx,tsx}`,
    // include packages if not transpiling
    "../../packages/ui/*.{js,ts,jsx,tsx}",
  ],
```

If you choose this strategy, you can remove the `tailwindcss` and `autoprefixer` dependencies from the `ui` package.

### Utilities

This Turborepo has some additional tools already setup for you:

- [Tailwind CSS](https://tailwindcss.com/) for styles
- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

