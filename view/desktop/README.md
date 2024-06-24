# About

Moss Desktop Application

# Technologies

- **Tauri** (framework for building desktop applications using JS, HTML, and CSS)
- **Vite** (framework for building fast, efficient, and scalable web apps)
- **React** (framework for creating interactive UI components)
- **TypeScript** (type-safety support)
- **Tailwind CSS** (CSS styling)
- **Redux** (React state manager)
- **i18next** (internationalization)
- **Jsonnet**
- **pNPM** (package manager)
- **Storybook** (workshop for building, testing, documenting and sharing UI components)

# Usage

```bash
  $ yarn tauri dev #start Tauri Desktop App
  $ pnpm run dev #start web app
  $ pnpm run build
  $ pnpm run lint
  $ pnpm run preview
  $ pnpm run interface #generate i18next translation interface
  $ pnpm run jsonnet #convert test jsonnet file to json file
```

## Available scripts

```json
 "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "preview": "vite preview",
    "interface": "i18next-resources-for-ts interface -i ./public/locales/de -o ./src/@types/resources.d.ts",
    "jsonnet": "jsonnet test.jsonnet -o test.json"
  },
```

# React + TypeScript + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react/README.md) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## Expanding the ESLint configuration

If you are developing a production application, we recommend updating the configuration to enable type aware lint rules:

- Configure the top-level `parserOptions` property like this:

```js
export default {
  // other rules...
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: ['./tsconfig.json', './tsconfig.node.json'],
    tsconfigRootDir: __dirname
  }
}
```

- Replace `plugin:@typescript-eslint/recommended` to `plugin:@typescript-eslint/recommended-type-checked` or `plugin:@typescript-eslint/strict-type-checked`
- Optionally add `plugin:@typescript-eslint/stylistic-type-checked`
- Install [eslint-plugin-react](https://github.com/jsx-eslint/eslint-plugin-react) and add `plugin:react/recommended` & `plugin:react/jsx-runtime` to the `extends` list
