import { Window } from "@tauri-apps/api/window";

import { createDecorator } from "../instantiation/instantiation";

export interface IWindowService {
  readonly _serviceBrand: undefined;

  getCurrentWindow(): Window;
}
export const IWindowService = createDecorator<IWindowService>("windowService");
