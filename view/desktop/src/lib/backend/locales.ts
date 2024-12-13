import { LanguagePack } from "@/store/language";

import { invokeTauriIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LanguagePack[], string>> => {
  return await invokeTauriIpc<LanguagePack[], string>("get_locales");
};

export default getLocales;
