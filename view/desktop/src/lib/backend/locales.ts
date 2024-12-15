import { LocaleDescriptor } from "@repo/moss-desktop";

import { invokeTauriIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LocaleDescriptor[], string>> => {
  return await invokeTauriIpc<LocaleDescriptor[], string>("get_locales");
};

export default getLocales;
