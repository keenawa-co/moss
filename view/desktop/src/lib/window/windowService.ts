import { getCurrentWindow, Window } from "@tauri-apps/api/window";
import { IWindowService } from "./window";

export class WindowService implements IWindowService {
  declare readonly _serviceBrand: undefined;
  private _appWindow: Window;

  constructor() {
    this._appWindow = getCurrentWindow();
  }
  getCurrentWindow(): Window {
    return this._appWindow;
  }
}
