import { contextBridge } from 'electron'
import { Titlebar, TitlebarColor } from "custom-electron-titlebar";

if (!process.contextIsolated) {
  throw new Error('contextIsolation must be enabled in the BrowserWindow')
}

try {
  contextBridge.exposeInMainWorld('context', {
    //TODO: Add your preload functions here
  })
} catch (error) {
  console.error(error)
}

// Custom TitlebarColor
window.addEventListener('DOMContentLoaded', () => {
  const options = {
    backgroundColor: TitlebarColor.fromHex("#42b3f5"),
    onlyShowMenubar: false,
    //enableMnemonics: true
  };

  new Titlebar(options);
})
