import { LanguageCode } from "@/store/language";

import { invokeTauriIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LanguageCode[], string>> => {
  return await invokeTauriIpc<LanguageCode[], string>("get_locales");
};

export default getLocales;
