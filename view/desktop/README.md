# About

Moss Desktop Application 

# Technologies
- **Electron** (framework for building desktop applications using JS, HTML, and CSS)
- **React** (framework for creating interactive UI components) 
- **TypeScript** (type-safety support)
- **Tailwind CSS** (CSS styling)
- **Redux** (React state manager)
- **i18next** (internationalization)

## Recommended IDE Setup

- [VSCode](https://code.visualstudio.com/) + [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint) + [Prettier](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)

## Project Setup

### Install

```bash
$ yarn
```
or
```bash
$ npm install
```

### Development

```bash
$ yarn dev
```
or
```bash
$ npm run dev
```

### Build

```bash
# For windows
$ yarn build:win

# For macOS
$ yarn build:mac

# For Linux
$ yarn build:linux
```

# Convert jsonnet file to json file
```bash
$ jsonnet .\test.jsonnet -o test.json
```
